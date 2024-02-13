#![no_std]
#![no_main]

#[cfg(not(debug_assertions))]
use panic_halt as _;

#[cfg(debug_assertions)]
use panic_probe as _;

#[cfg(debug_assertions)]
use defmt_rtt as _;

use rtic::app;

#[app(device = stm32f4xx_hal::pac, dispatchers = [SPI3])]
mod app {
    use stm32f4xx_hal::prelude::*;

    #[cfg(debug_assertions)]
    use defmt::{dbg, error, info, warn};

    // Get the fake implementations for empty macros (no rtt) on release
    #[cfg(not(debug_assertions))]
    use probe_test::{dbg, error, info, warn};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        info!("Started the template main!");
        dbg!("Started the template main! (Debug)");
        warn!("Started the template main! (warn)");
        error!("Started the template main! (Error)");

        task_1::spawn(20).unwrap();

        (Shared {}, Local {}, init::Monotonics())
    }

    #[task(priority = 1, capacity = 2)]
    fn task_1(_: task_1::Context, x: i32) {
        info!("Got to task 1 with parameter ({})", x);
    }

    #[idle]
    fn _idle(_: _idle::Context) -> ! {
        #[allow(clippy::empty_loop)]
        loop {}
    }
}
