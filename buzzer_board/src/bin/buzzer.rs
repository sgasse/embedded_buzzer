#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use buzzer_board::button_task::debounced_button_presses;
use buzzer_board::leds::led_task;
use buzzer_board::net::{init_net_stack, net_task, rx_task, tx_task};
use buzzer_board::{create_net_peripherals, gen_random_seed, singleton, NUM_LEDS};
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::exti::Channel;
use embassy_stm32::gpio::{AnyPin, Level, Output, Pin, Speed};
use embassy_stm32::time::mhz;
use embassy_stm32::Config;
use embassy_time::{Duration, Timer};
use heapless::Vec;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let mut config = Config::default();
    config.rcc.sys_ck = Some(mhz(400));
    config.rcc.hclk = Some(mhz(200));
    config.rcc.pll1.q_ck = Some(mhz(100));
    let p = embassy_stm32::init(config);

    let seed = gen_random_seed(p.RNG);

    // Configure LED pins
    let led_pins: [AnyPin; NUM_LEDS] = [
        p.PA6.degrade(),
        p.PA8.degrade(),
        p.PI8.degrade(),
        p.PB6.degrade(),
        p.PE3.degrade(),
        p.PB15.degrade(),
    ];
    let mut led_outputs: Vec<Output<AnyPin>, NUM_LEDS> = Vec::new();

    for pin in led_pins {
        led_outputs
            .push(Output::new(pin, Level::Low, Speed::Low))
            .ok();
    }

    let led_outputs: &'static mut Vec<Output<'static, AnyPin>, NUM_LEDS> = singleton!(led_outputs);
    unwrap!(spawner.spawn(led_task(led_outputs)));

    let net_p = create_net_peripherals!(p);
    let stack = init_net_stack(net_p, seed);

    // Launch network task
    unwrap!(spawner.spawn(net_task(&stack)));
    info!("Network task initialized");

    unwrap!(spawner.spawn(rx_task(&stack)));
    unwrap!(spawner.spawn(tx_task(&stack)));

    unwrap!(spawner.spawn(debounced_button_presses([
        (p.PG3.degrade(), p.EXTI3.degrade()),
        (p.PK1.degrade(), p.EXTI1.degrade()),
        (p.PE6.degrade(), p.EXTI6.degrade()),
        (p.PB7.degrade(), p.EXTI7.degrade()),
        (p.PH15.degrade(), p.EXTI15.degrade()),
        (p.PB4.degrade(), p.EXTI4.degrade()),
        // Blue onboard user button `B1`
        (p.PC13.degrade(), p.EXTI13.degrade()),
    ])));

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
