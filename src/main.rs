use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use inputbot::KeybdKey::EqualKey;
use rand::{Rng, thread_rng};
use winapi::um::winuser::{INPUT, INPUT_MOUSE, MOUSEEVENTF_MOVE, SendInput};

const SLEEP_MOVE_MIN: u64 = 30 * 1000;
const SLEEP_MOVE_MAX: u64 = 60 * 1000;

const CHECK_EVERY: u64 = 1000;

const ORDERING: Ordering = Ordering::Relaxed;

static mut SWITCH: bool = false;

#[tokio::main]
async fn main()
{
    let mut times = 0;

    let enabled = Arc::new(AtomicBool::new(false));
    let enabled_clone = Arc::clone(&enabled);

    let enable_logic = move || {
        let current_value = &enabled_clone.load(ORDERING);
        enabled_clone.store(!current_value, ORDERING);

        let status = if !current_value { "ENABLED" } else { "DISABLED" };
        println!("Mouse move is now {}.", status);
    };

    EqualKey.bind(enable_logic.clone());

    tokio::spawn(async move {
        loop {
            if !enabled.load(ORDERING) {
                tokio::time::sleep(Duration::from_millis(CHECK_EVERY)).await;
                continue;
            }

            let next_sleep = generate_random_time(SLEEP_MOVE_MIN, SLEEP_MOVE_MAX);

            unsafe {
                raw_moving_logic();

                times += 1;
                println!("{} - Cursor moved (next one in ~{} seconds).", times, next_sleep.as_secs());
            }

            tokio::time::sleep(next_sleep).await;
        }
    });

    println!(" ");
    println!("░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░█▀█░█░█░▀█▀░█▀▄░▀█▀░█▀█░░░▀█▀░░░█▀█░█▄█░░░█▀▀░▀█▀░▀█▀░█░░░█░░░░░█░█░█▀▀░█▀▄░█▀▀░
░█░█░▀▄▀░░█░░█░█░░█░░█▀█░░░░█░░░░█▀█░█░█░░░▀▀█░░█░░░█░░█░░░█░░░░░█▀█░█▀▀░█▀▄░█▀▀░
░▀░▀░░▀░░▀▀▀░▀▀░░▀▀▀░▀░▀░░░▀▀▀░░░▀░▀░▀░▀░░░▀▀▀░░▀░░▀▀▀░▀▀▀░▀▀▀░░░▀░▀░▀▀▀░▀░▀░▀▀▀░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░");
    println!(" ");
    println!("Press the '=' key to enable/disable the mouse movement.");
    println!(" ");

    enable_logic();

    inputbot::handle_input_events();
}

unsafe fn raw_moving_logic()
{
    let sign = if SWITCH { -1 } else { 1 };
    SWITCH = !SWITCH;

    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe { std::mem::zeroed::<winapi::um::winuser::INPUT_u>() },
    };

    let mut mouse_input = std::mem::zeroed::<winapi::um::winuser::MOUSEINPUT>();

    mouse_input.dx = sign;
    mouse_input.dy = sign;
    mouse_input.dwFlags = MOUSEEVENTF_MOVE;

    *input.u.mi_mut() = mouse_input;

    SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
}

fn generate_random_number(start: u64, end: u64) -> u64
{
    let mut rng = thread_rng();
    rng.gen_range(start..end)
}

fn generate_random_time(start: u64, end: u64) -> Duration
{
    Duration::from_millis(generate_random_number(start, end))
}
