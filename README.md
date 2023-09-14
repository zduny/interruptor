# interruptor

[![Test Status](https://github.com/zduny/interruptor/actions/workflows/rust.yml/badge.svg)](https://github.com/zduny/interruptor/actions)
[![Crate](https://img.shields.io/crates/v/interruptor.svg)](https://crates.io/crates/interruptor)
[![API](https://docs.rs/interruptor/badge.svg)](https://docs.rs/interruptor)

Collection of functions generating Unix process signal receivers 
for use in the [Crossbeam Channel](https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-channel) `select!` macro.

## example

```rust
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
```

## see also

[Crossbeam](https://github.com/crossbeam-rs/crossbeam)

[Crossbeam Channel](https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-channel)
