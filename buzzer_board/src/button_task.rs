use defmt::info;
use embassy_stm32::{
    exti::{self, ExtiInput},
    gpio::{self, Input, Pull},
};
use embassy_time::Instant;
use futures::{future::FutureExt, select_biased};

use crate::ButtonChannel;

/// Total number of buzzer buttons.
const NUM_BUTTONS: usize = 7;

/// Minimum time in milliseconds to pass between two flanks to be considered a new press.
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
    buzzer_buttons: [(gpio::AnyPin, exti::AnyChannel); NUM_BUTTONS - 1],
    board_button: (gpio::AnyPin, exti::AnyChannel),
    button_channel: &'static ButtonChannel,
) -> ! {
    info!("Launching button task");

    let buttons: [_; NUM_BUTTONS - 1] = buzzer_buttons.map(|(button, exti_input)| {
        let button = Input::new(button, Pull::Up);
        ExtiInput::new(button, exti_input)
    });
    unpack_buttons!(buttons; b0, b1, b2, b3, b4, b5);

    // The board button works with Pull:Down and has to be set up individually.
    let mut b6 = ExtiInput::new(Input::new(board_button.0, Pull::Down), board_button.1);

    let mut last_pressed: [u64; NUM_BUTTONS] = [0; NUM_BUTTONS];

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
                    button_channel.send((0, now)).await;
                }
                last_pressed[0] = now;
            },
            _ = f1 => {
                let now = Instant::now().as_millis();
                if now.saturating_sub(last_pressed[1]) > MIN_DEBOUNCE_MILLIS {
                    button_channel.send((1, now)).await;
                }
                last_pressed[1] = now;
            },
            _ = f2 => {
                let now = Instant::now().as_millis();
                if now.saturating_sub(last_pressed[2]) > MIN_DEBOUNCE_MILLIS {
                    button_channel.send((2, now)).await;
                }
                last_pressed[2] = now;
            },
            _ = f3 => {
                let now = Instant::now().as_millis();
                if now.saturating_sub(last_pressed[3]) > MIN_DEBOUNCE_MILLIS {
                    button_channel.send((3, now)).await;
                }
                last_pressed[3] = now;
            },
            _ = f4 => {
                let now = Instant::now().as_millis();
                if now.saturating_sub(last_pressed[4]) > MIN_DEBOUNCE_MILLIS {
                    button_channel.send((4, now)).await;
                }
                last_pressed[4] = now;
            },
            _ = f5 => {
                let now = Instant::now().as_millis();
                if now.saturating_sub(last_pressed[5]) > MIN_DEBOUNCE_MILLIS {
                    button_channel.send((5, now)).await;
                }
                last_pressed[5] = now;
            },
            _ = f6 => {
                let now = Instant::now().as_millis();
                if now.saturating_sub(last_pressed[6]) > MIN_DEBOUNCE_MILLIS {
                    button_channel.send((6, now)).await;
                }
                last_pressed[6] = now;
            },
        };
    }
}
