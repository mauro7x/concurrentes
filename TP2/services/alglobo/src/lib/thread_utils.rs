use std::{
    sync::mpsc::{self, Sender, TryRecvError},
    thread,
};

use crate::{
    constants::{control::THREAD_OK, errors::THREAD_JOIN_ERROR},
    types::{common::BoxResult, control::SafeThread},
};

// ----------------------------------------------------------------------------

pub fn safe_spawn<C: 'static, F: 'static>(
    mut c: C,
    f: F,
    threads: &mut Vec<SafeThread>,
) -> BoxResult<()>
where
    C: std::marker::Send,
    F: Fn(&mut C, Sender<String>) + std::marker::Send,
{
    let (tx, rx) = mpsc::channel();
    let safe_thread = SafeThread {
        joiner: thread::spawn(move || {
            f(&mut c, tx);
        }),
        channel: rx,
    };

    threads.push(safe_thread);

    Ok(())
}

pub fn check_threads(threads: &mut Vec<SafeThread>) -> BoxResult<()> {
    while let Some(thread) = threads.pop() {
        match thread.channel.try_recv() {
            Ok(msg) => match msg.as_str() {
                THREAD_OK => {
                    thread.joiner.join().map_err(|_| THREAD_JOIN_ERROR)?;
                }
                err => return Err(err.into()),
            },
            Err(TryRecvError::Empty) => threads.push(thread),
            Err(TryRecvError::Disconnected) => {
                return Err("Thread close its channel unexpectedly".into())
            }
        };
    }

    Ok(())
}
