//! Log helpers.

#[macro_export]
macro_rules! log {
    // todo(xdoardo): find a way to have a single `print!` call.
    ($app:expr, $level:expr, $($e:expr),*) => {
        if $app.verbosity.log_level().is_some_and(|v| v >= $level) {
            $(
                print!("{}", $e);
            )*
            use std::io::Write;
            _ = std::io::stdout().flush();
        }
    };
}

#[macro_export]
macro_rules! logln {
    // todo(xdoardo): find a way to have a single `print!` call.
    ($app:expr, $level:expr, $($e:expr),*) => {
        if $app.verbosity.log_level().is_some_and(|v| v >= $level) {
            $(
                print!("{}", $e);
            )*

            println!()
        }
    };
}

#[macro_export]
macro_rules! trace {
    ($app:expr, $($e:expr),+) => {

        log!($app, clap_verbosity_flag::log::Level::Trace, $($e),*)
    };
}

#[macro_export]
macro_rules! traceln {
    ($app:expr, $($e:expr),*) => {

        logln!($app, clap_verbosity_flag::log::Level::Trace, $($e),*);
    };
}

#[macro_export]
macro_rules! info {
    ($app:expr, $($e:expr),+) => {

        log!($app, clap_verbosity_flag::log::Level::Info, $($e),*)
    };
}

#[macro_export]
macro_rules! infoln {
    ($app:expr, $($e:expr),*) => {

        logln!($app, clap_verbosity_flag::log::Level::Info, $($e),*);
    };
}

#[macro_export]
macro_rules! warning {
    ($app:expr, $($e:expr),+) => {

        log!($app, clap_verbosity_flag::log::Level::Warn, "warn: ".bright_yellow(), $($e),*)
    };
}

#[macro_export]
macro_rules! warnln {
    ($app:expr, $($e:expr),*) => {

        logln!($app, clap_verbosity_flag::log::Level::Warn, "warn: ".bright_yellow(), $($e),*);
    };
}

#[macro_export]
macro_rules! err {
    ($app:expr, $($e:expr),+) => {

        log!($app, clap_verbosity_flag::log::Level::Error, "error: ".bright_red(), $($e),*)
    };
}

#[macro_export]
macro_rules! errln {
    ($app:expr, $($e:expr),*) => {

        logln!($app, clap_verbosity_flag::log::Level::Error, "error: ".bright_red(), $($e),*);
    };
}

#[allow(unused)]
pub use {err, errln, info, infoln, log, logln, trace, traceln, warning, warnln};
