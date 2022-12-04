#![no_std]
#![no_main]

// O semihosting impede o programa de rodar fora do debug do gdb
#[cfg(debug_assertions)]
use panic_semihosting as _;
#[cfg(not(debug_assertions))]
use panic_halt as _;

use rtic::app;

#[app(device = stm32f4xx_hal::pac, dispatchers = [SPI3])]
mod app {
    #[cfg(debug_assertions)]
    use cortex_m_semihosting::hprintln;
    #[cfg(not(debug_assertions))]
    macro_rules! hprintln {
        ( $( $x:expr ), * ) => { () };
    }

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        hprintln!("Init");

        task_1::spawn().unwrap();
        ( Shared {},Local {}, init::Monotonics() )
    }

    #[task(priority = 1, capacity = 2)]
    fn task_1(_: task_1::Context) {
        hprintln!("Got to the task 1");
    }

    #[idle]
    fn _idle(_: _idle::Context) -> ! {
        hprintln!("Got to the idle for the first time! (:");

        loop {
            hprintln!("Going to sleep zzz");
            rtic::export::wfi();
        }
    }
}

