use crate::{singleton, NetPeripherals};
use embassy_net::{Ipv4Address, Ipv4Cidr, Stack, StackResources};
use embassy_stm32::eth::PacketQueue;
use embassy_stm32::eth::{generic_smi::GenericSMI, Ethernet};
use embassy_stm32::interrupt;
use embassy_stm32::peripherals::ETH;
use heapless::Vec;

pub type Device = Ethernet<'static, ETH, GenericSMI>;

#[embassy_executor::task]
pub async fn net_task(stack: &'static Stack<Device>) -> ! {
    stack.run().await
}

pub fn init_net_stack(net_p: NetPeripherals, seed: u64) -> &'static Stack<Device> {
    let eth_int = interrupt::take!(ETH);
    let mac_addr = [0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF];

    let device = Ethernet::new(
        singleton!(PacketQueue::<16, 16>::new()),
        net_p.eth,
        eth_int,
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
        GenericSMI,
        mac_addr,
        1,
    );

    // Set laptop IP to 192.168.100.1 and listen with `netcat -l 8000`
    let config = embassy_net::Config::Static(embassy_net::StaticConfig {
        address: Ipv4Cidr::new(Ipv4Address::new(192, 168, 100, 5), 24),
        dns_servers: Vec::new(),
        gateway: Some(Ipv4Address::new(192, 168, 100, 1)),
    });

    // Init network stack
    let stack = &*singleton!(Stack::new(
        device,
        config,
        singleton!(StackResources::<2>::new()),
        seed
    ));

    stack
}
