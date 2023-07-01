use defmt::info;
use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Input, Pull},
};
use embassy_time::Instant;
use futures::{future::FutureExt, select_biased};

use crate::BUTTON_PRESS_Q;

const NUM_BUTTONS: usize = 7;
const MIN_DEBOUNCE_MILLIS: u64 = 100;

macro_rules! unpack_buttons {
    ($buttons:expr; $($button:ident),+) => {
        let [
            $(
                mut $button,
            )+
        ] = $buttons;
    };
}

macro_rules! fused_futures {
    ($(($button:expr, $fut:pat)),+) => {
        $(
            let $fut= $button.wait_for_rising_edge().fuse();
        )+
    };
}

macro_rules! pinned_futures {
    ($($fut:tt),+) => {
        $(
            futures::pin_mut!($fut);
        )+
    };
}

#[embassy_executor::task]
pub async fn debounced_button_presses(
    buttons: [(embassy_stm32::gpio::AnyPin, embassy_stm32::exti::AnyChannel); NUM_BUTTONS],
) -> ! {
    info!("Launching button task");

    let mut last_pressed: [u64; NUM_BUTTONS] = [0; NUM_BUTTONS];

    let buttons: [_; NUM_BUTTONS] = buttons.map(|(button, exti_input)| {
        let button = Input::new(button, Pull::Down);
        ExtiInput::new(button, exti_input)
    });

    unpack_buttons!(buttons; b0, b1, b2, b3, b4, b5, b6);

    loop {
        fused_futures!(
            (b0, f0),
            (b1, f1),
            (b2, f2),
            (b3, f3),
            (b4, f4),
            (b5, f5),
            (b6, f6)
        );
        pinned_futures!(f0, f1, f2, f3, f4, f5, f6);

        select_biased! {
            _ = f0 => {
                let now = Instant::now().as_millis();
                if now.saturating_sub(last_pressed[0]) > MIN_DEBOUNCE_MILLIS {
                    BUTTON_PRESS_Q.enqueue((0, now)).ok();
                }
                last_pressed[0] = now;
            },
            _ = f1 => {
                let now = Instant::now().as_millis();
                if now.saturating_sub(last_pressed[1]) > MIN_DEBOUNCE_MILLIS {
                    BUTTON_PRESS_Q.enqueue((1, now)).ok();
                }
                last_pressed[1] = now;
            },
            _ = f2 => {
                let now = Instant::now().as_millis();
                if now.saturating_sub(last_pressed[2]) > MIN_DEBOUNCE_MILLIS {
                    BUTTON_PRESS_Q.enqueue((2, now)).ok();
                }
                last_pressed[2] = now;
            },
            _ = f3 => {
                let now = Instant::now().as_millis();
                if now.saturating_sub(last_pressed[3]) > MIN_DEBOUNCE_MILLIS {
                    BUTTON_PRESS_Q.enqueue((3, now)).ok();
                }
                last_pressed[3] = now;
            },
            _ = f4 => {
                let now = Instant::now().as_millis();
                if now.saturating_sub(last_pressed[4]) > MIN_DEBOUNCE_MILLIS {
                    BUTTON_PRESS_Q.enqueue((4, now)).ok();
                }
                last_pressed[4] = now;
            },
            _ = f5 => {
                let now = Instant::now().as_millis();
                if now.saturating_sub(last_pressed[5]) > MIN_DEBOUNCE_MILLIS {
                    BUTTON_PRESS_Q.enqueue((5, now)).ok();
                }
                last_pressed[5] = now;
            },
            _ = f6 => {
                let now = Instant::now().as_millis();
                if now.saturating_sub(last_pressed[6]) > MIN_DEBOUNCE_MILLIS {
                    BUTTON_PRESS_Q.enqueue((6, now)).ok();
                }
                last_pressed[6] = now;
            },
        };
    }
}
