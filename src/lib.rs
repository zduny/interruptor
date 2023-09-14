//! Collection of functions generating Unix process signal receivers
//! for use in the [crossbeam_channel::select!] macro.
//!
//! ## Example
//!
//! ```ignore
//! let tick = tick(Duration::from_secs(1));
//! let stop = interruption_or_termination();
//!
//! loop {
//!     select! {
//!         recv(tick) -> _ => println!("Running!"),
//!         recv(stop) -> _ => break,
//!     }
//! }
//! ```
//!

use crossbeam_channel::{unbounded, Receiver};
pub use simple_signal::Signal;

/// Create (single) receiver for Unix process signals listed in `signals`.
pub fn signal(signals: &[Signal]) -> Receiver<()> {
    let (sender, receiver) = unbounded();
    simple_signal::set_handler(signals, {
        move |_| {
            let _ = sender.send(());
        }
    });

    receiver
}

/// Create receiver for `SIGHUP` signal.
pub fn hang_up() -> Receiver<()> {
    signal(&[Signal::Hup])
}

/// Create receiver for `SIGINT` signal.
pub fn interruption() -> Receiver<()> {
    signal(&[Signal::Int])
}

/// Create receiver for `SIGTERM` signal.
pub fn termination() -> Receiver<()> {
    signal(&[Signal::Term])
}

/// Create (single) receiver for `SIGINT` and `SIGTERM` signal.
pub fn interruption_or_termination() -> Receiver<()> {
    signal(&[Signal::Int, Signal::Term])
}
