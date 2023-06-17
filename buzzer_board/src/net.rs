use embassy_net::{Ipv4Address, Ipv4Cidr, Stack, StackResources};
use embassy_stm32::eth::{generic_smi::GenericSMI, Ethernet};
use embassy_stm32::peripherals::ETH;

pub type Device = Ethernet<'static, ETH, GenericSMI>;

#[embassy_executor::task]
pub async fn net_task(stack: &'static Stack<Device>) -> ! {
    stack.run().await
}
