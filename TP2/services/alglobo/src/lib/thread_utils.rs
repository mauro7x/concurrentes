use std::{
    sync::mpsc::{self, TryRecvError},
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
    F: Fn(&mut C) -> BoxResult<()> + std::marker::Send,
{
    let (tx, rx) = mpsc::channel();
    let joiner = thread::spawn(move || {
        match f(&mut c) {
            Ok(_) => tx.send(THREAD_OK.to_string()),
            Err(err) => tx.send(err.to_string()),
        }
        .unwrap();
    });

    let safe_thread = SafeThread {
        joiner,
        channel: rx,
    };
    threads.push(safe_thread);

    Ok(())
}

pub fn check_threads(threads: &mut Vec<SafeThread>) -> BoxResult<()> {
    let mut running_threads = vec![];

    while let Some(thread) = threads.pop() {
        match thread.channel.try_recv() {
            Ok(msg) => match msg.as_str() {
                THREAD_OK => {
                    thread.joiner.join().map_err(|_| THREAD_JOIN_ERROR)?;
                }
                err => return Err(err.into()),
            },
            Err(TryRecvError::Empty) => running_threads.push(thread),
            Err(TryRecvError::Disconnected) => {
                return Err("Thread close its channel unexpectedly".into())
            }
        };
    }

    *threads = running_threads;

    Ok(())
}
