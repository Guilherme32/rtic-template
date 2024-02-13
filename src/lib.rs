#![no_std]

pub mod empty_defmt {
    #[macro_export]
    macro_rules! dbg {
        ( $( $x:expr ), * ) => {
            ()
        };
    }

    #[macro_export]
    macro_rules! info {
        ( $( $x:expr ), * ) => {
            ()
        };
    }

    #[macro_export]
    macro_rules! warn {
        ( $( $x:expr ), * ) => {
            ()
        };
    }

    #[macro_export]
    macro_rules! error {
        ( $( $x:expr ), * ) => {
            ()
        };
    }
}
