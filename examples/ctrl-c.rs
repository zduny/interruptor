use std::time::Duration;

use crossbeam_channel::{select, tick};
use interruptor::interruption_or_termination;

fn main() {
    let tick = tick(Duration::from_secs(1));
    let stop = interruption_or_termination();

    loop {
        select! {
            recv(tick) -> _ => println!("Running!"),
            recv(stop) -> _ => break,
        }
    };
}
