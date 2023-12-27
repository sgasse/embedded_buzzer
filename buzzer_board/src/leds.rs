use embassy_stm32::gpio::Level;
use embassy_time::Timer;

use crate::{LedOutputs, LED_CHANGE_Q, THROTTLE_TIME};

#[embassy_executor::task]
pub async fn led_task(led_outputs: LedOutputs) -> ! {
    loop {
        match LED_CHANGE_Q.dequeue() {
            None => Timer::after(THROTTLE_TIME).await,
            Some(led_update) => {
                if let Some(led) = led_outputs.get_mut(led_update.button_id as usize) {
                    if led_update.on {
                        led.set_level(Level::High);
                    } else {
                        led.set_level(Level::Low);
                    }
                }
            }
        }
    }
}
