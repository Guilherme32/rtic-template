#![no_std]
#![no_main]

// semihosting prevents the program from running without gdb (which enables
// the semihosting on the other side)
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

    use stm32f4xx_hal::{
        prelude::*,
        gpio::{self, PushPull, PinState, Output},
        pac::TIM2,
        timer::{self, Event},
    };

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: gpio::PC13<Output<PushPull>>,
        timer: timer::CounterMs<TIM2>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        hprintln!("Started the blink example using a hardware timer!");

        let dp = ctx.device;
        let gpioc = dp.GPIOC.split();
        let mut led = gpioc.pc13.into_push_pull_output();
        led.set_state(PinState::Low);

        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();
        let mut timer = dp.TIM2.counter_ms(&clocks);
        timer.start(1000.millis()).unwrap();
        timer.listen(Event::Update);

        task_1::spawn(20).unwrap();
        task_1::spawn(69).unwrap();
        match task_1::spawn(420) {
            Ok(_) => hprintln!("Started third task (should not)"),
            Err(_) => hprintln!("task_1 capacity is 2, so a third won't go in line"),
        }

        //ctx.core.SCB.set_sleeponexit();  // With this, it sleeps before reaching idle
        // this sleep is shallow, and will wake with any interrupt in up to 6 cycles
        // however, the consumption falls only to about half
        // (sleepdeep and standby drop it a lot lower, but loose on the other advantages)

        (
            Shared {},
            Local {led, timer},
            init::Monotonics()
         )
    }

    #[task(priority = 1, capacity = 2)]
    fn task_1(_: task_1::Context, x:i32) {
        hprintln!("Got to the task 1 with parameter {}! :)", x);
    }

    #[task(binds = TIM2, local=[led, timer], priority = 2)]
    fn timer_interrupt(ctx: timer_interrupt::Context) {
        ctx.local.led.toggle();
        ctx.local.timer.clear_interrupt(Event::Update);
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

