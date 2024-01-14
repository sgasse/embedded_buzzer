use core::sync::atomic::Ordering;

use crate::{ButtonChannel, Irqs, NetPeripherals, INIT_TIME, LED_CHANGE_Q};
use common::{ButtonPress, Message, MsgBuffer};
use defmt::*;
use embassy_futures::select::{select, Either};
use embassy_net::tcp::TcpSocket;
use embassy_net::{tcp::Error::ConnectionReset, Ipv4Address, Ipv4Cidr, Stack, StackResources};
use embassy_stm32::eth::PacketQueue;
use embassy_stm32::eth::{generic_smi::GenericSMI, Ethernet};
use embassy_stm32::peripherals::ETH;
use embassy_time::{Duration, Instant, Timer};
use embedded_io_async::Write;
use heapless::Vec;
use postcard::to_slice;
use static_cell::StaticCell;

pub type Device = Ethernet<'static, ETH, GenericSMI>;

#[embassy_executor::task]
pub async fn net_task(stack: &'static Stack<Device>) -> ! {
    stack.run().await
}

pub fn init_net_stack(net_p: NetPeripherals, seed: u64) -> &'static Stack<Device> {
    let mac_addr = [0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF];

    static PACKETS: StaticCell<PacketQueue<16, 16>> = StaticCell::new();

    let device = Ethernet::new(
        PACKETS.init(PacketQueue::<16, 16>::new()),
        net_p.eth,
        Irqs,
        net_p.pa1,
        net_p.pc3,
        net_p.pa2,
        net_p.pc1,
        net_p.pa7,
        net_p.pc4,
        net_p.pc5,
        net_p.pb0,
        net_p.pb1,
        net_p.pg13,
        net_p.pg12,
        net_p.pc2,
        net_p.pe2,
        net_p.pg11,
        GenericSMI::new(1),
        mac_addr,
    );

    // Set laptop IP to 192.168.100.1 and listen with `netcat -l 8000`
    let config = embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
        address: Ipv4Cidr::new(Ipv4Address::new(192, 168, 100, 5), 24),
        dns_servers: Vec::new(),
        gateway: Some(Ipv4Address::new(192, 168, 100, 1)),
    });

    static STACK: StaticCell<Stack<Device>> = StaticCell::new();
    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    &*STACK.init(Stack::new(
        device,
        config,
        RESOURCES.init(StackResources::<3>::new()),
        seed,
    ))
}

#[embassy_executor::task]
pub async fn tcp_task(stack: &'static Stack<Device>, button_channel: &'static ButtonChannel) -> ! {
    let mut tx_buf = [0u8; 1024];
    let mut rx_buf = [0u8; 1024];

    let endpoint_ip = embassy_net::IpAddress::Ipv4(Ipv4Address([192, 168, 100, 1]));
    let endpoint = embassy_net::IpEndpoint::new(endpoint_ip, 8000);

    'outer: loop {
        info!("Connecting TCP socket to {:?}", endpoint);
        let mut tcp_socket = TcpSocket::new(stack, &mut rx_buf, &mut tx_buf);

        if let Err(e) = tcp_socket.connect(endpoint).await {
            warn!("Failed to connect to endpoint {:?}: {}", &endpoint, e);
            Timer::after(Duration::from_secs(1)).await;
            continue 'outer;
        }

        let (mut reader, mut writer) = tcp_socket.split();

        // Reader state
        let mut msg_buffer = MsgBuffer::<1024>::default();
        let mut remaining_err_on_zero = 3;

        // Writer state
        let mut serialize_buffer = [0u8; 128];

        'inner: loop {
            // Create futures for reading and receivng a button press update.
            let read_fut = reader.read(msg_buffer.as_buf());
            let button_fut = button_channel.receive();

            match select(read_fut, button_fut).await {
                Either::First(read_res) => match read_res {
                    Ok(0) => {
                        // Nothing new read, try to deserialize.
                        if !msg_buffer.process_msgs_ok(handle_message) {
                            if remaining_err_on_zero <= 0 {
                                warn!("Reconnecting");
                                continue 'outer;
                            }
                            remaining_err_on_zero -= 1;
                        }
                    }
                    Ok(num_read) => {
                        msg_buffer.cursor += num_read;
                        msg_buffer.process_msgs_ok(handle_message);
                    }
                    Err(e) => {
                        warn!("Error while reading: {}", e);
                        if e == ConnectionReset {
                            continue 'outer;
                        }
                    }
                },
                Either::Second((button_id, press_time)) => {
                    let millis_since_init =
                        (press_time as u32).saturating_sub(INIT_TIME.load(Ordering::Acquire));
                    if millis_since_init == 0 {
                        warn!(
                            "Button press {} registered before last reset, skipping",
                            button_id
                        );
                        continue 'inner;
                    }

                    let message = Message::ButtonPress(ButtonPress {
                        button_id,
                        millis_since_init,
                    });

                    debug!("Sending message: {:?}", message);

                    let serialized = to_slice(&message, &mut serialize_buffer).unwrap();

                    let r = writer.write_all(serialized).await;
                    if let Err(e) = r {
                        warn!("write error: {:?}", e);

                        if e == ConnectionReset {
                            continue 'outer;
                        }

                        continue 'inner;
                    }
                }
            }
        }
    }
}

fn handle_message(message: Message) {
    match message {
        Message::InitBoard | Message::InitReactionGame(_) => {
            info!("Received InitBoard instruction");
            let instant_millis = Instant::now().as_millis() as u32;
            INIT_TIME.store(instant_millis, Ordering::Release);
        }
        Message::Ping(ping_nr) => {
            info!("Received Ping({})", ping_nr);
        }
        Message::ButtonPress(_) => {
            warn!("Board should not be receiving ButtonPress data");
        }
        Message::LedUpdate(update) => {
            info!("Received LED update: {:?}", update);
            LED_CHANGE_Q.enqueue(update).ok();
        }
    }
}
