# interruptor

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
