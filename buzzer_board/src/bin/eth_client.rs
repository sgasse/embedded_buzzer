#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::num::Wrapping;

use buzzer_board::net::{init_net_stack, net_task};
use buzzer_board::{gen_random_seed, NetPeripherals};
use defmt::*;
use embassy_executor::Spawner;
use embassy_net::tcp::client::{TcpClient, TcpClientState};
use embassy_net::tcp::Error::ConnectionReset;
use embassy_stm32::time::mhz;
use embassy_stm32::Config;
use embassy_time::{Duration, Timer};
use embedded_io::asynch::Write;
use embedded_nal_async::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpConnect};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let mut config = Config::default();
    config.rcc.sys_ck = Some(mhz(400));
    config.rcc.hclk = Some(mhz(200));
    config.rcc.pll1.q_ck = Some(mhz(100));
    let p = embassy_stm32::init(config);

    let seed = gen_random_seed(p.RNG);

    let net_p = NetPeripherals {
        eth: p.ETH,
        pa1: p.PA1,
        pa2: p.PA2,
        pa7: p.PA7,
        pb0: p.PB0,
        pb1: p.PB1,
        pc1: p.PC1,
        pc2: p.PC2,
        pc3: p.PC3,
        pc4: p.PC4,
        pc5: p.PC5,
        pe2: p.PE2,
        pg11: p.PG11,
        pg12: p.PG12,
        pg13: p.PG13,
    };

    let stack = init_net_stack(net_p, seed);

    // Launch network task
    unwrap!(spawner.spawn(net_task(&stack)));
    info!("Network task initialized");

    static STATE: TcpClientState<1, 1024, 1024> = TcpClientState::new();
    let client = TcpClient::new(&stack, &STATE);

    loop {
        let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(192, 168, 100, 1), 8000));

        info!("connecting...");
        let r = client.connect(addr).await;
        if let Err(e) = r {
            error!("connect error: {:?}", e);
            Timer::after(Duration::from_secs(1)).await;
            continue;
        }

        let mut connection = r.unwrap();
        info!("connected!");

        let mut counter = Wrapping(0_usize);

        loop {
            let mut buf = [0u8; 64];

            info!("Sending counter {}", counter.0);

            let s: &str = format_no_std::show(
                &mut buf,
                format_args!("GET /some/path/{counter} HTTP/1.1\r\n\r\n"),
            )
            .unwrap();

            let r = connection.write_all(s.as_bytes()).await;
            if let Err(e) = r {
                info!("write error: {:?}", e);

                if e == ConnectionReset {
                    break;
                }

                continue;
            }

            counter += 1;

            Timer::after(Duration::from_secs(1)).await;
        }
    }
}
