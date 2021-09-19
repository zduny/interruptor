use crossbeam_channel::{Receiver, unbounded};
pub use simple_signal::Signal;

pub fn signal(signals: &[Signal]) -> Receiver<()>
{
    let (sender, receiver) = unbounded();
    simple_signal::set_handler(signals, {
        move |_| {
            let _ = sender.send(());
        }
    });

    receiver
}

pub fn interruption() -> Receiver<()> {
    signal(&[Signal::Int])
}

pub fn termination() -> Receiver<()>
{
    signal(&[Signal::Int, Signal::Term])
}
