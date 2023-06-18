#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use buzzer_board::net::{init_net_stack, net_task, rx_task, tx_task};
use buzzer_board::{create_net_peripherals, gen_random_seed};
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::time::mhz;
use embassy_stm32::Config;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let mut config = Config::default();
    config.rcc.sys_ck = Some(mhz(400));
    config.rcc.hclk = Some(mhz(200));
    config.rcc.pll1.q_ck = Some(mhz(100));
    let p = embassy_stm32::init(config);

    let seed = gen_random_seed(p.RNG);

    let net_p = create_net_peripherals!(p);
    let stack = init_net_stack(net_p, seed);

    // Launch network task
    unwrap!(spawner.spawn(net_task(&stack)));
    info!("Network task initialized");

    unwrap!(spawner.spawn(rx_task(&stack)));
    unwrap!(spawner.spawn(tx_task(&stack)));

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
