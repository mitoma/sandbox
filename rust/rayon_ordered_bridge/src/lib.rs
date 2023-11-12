use std::sync::mpsc::{channel, sync_channel, Receiver, SyncSender, TrySendError};

use rayon::prelude::{ParallelBridge, ParallelIterator};

pub struct Waiter {
    handlers: Vec<std::thread::JoinHandle<()>>,
}

impl Waiter {
    pub fn wait(self) {
        for handler in self.handlers {
            handler.join().unwrap();
        }
    }
}

pub fn bounded_parralel_map<I, F, T, U>(bound: usize, iter: I, f: F) -> (Receiver<U>, Waiter)
where
    I: Iterator<Item = T> + Send + 'static,
    F: Fn(T) -> U,
    F: Send + Sync + 'static,
    T: Send,
    U: Send + 'static,
{
    let (collect_tx, collect_rx) = sync_channel::<Receiver<U>>(bound);
    let (result_tx, result_rx) = sync_channel::<U>(bound);

    let map_handler = std::thread::spawn(move || {
        iter.map(|v| {
            let (s, mut r) = channel::<U>();
            loop {
                r = match collect_tx.try_send(r) {
                    Ok(_) => break (v, s),
                    Err(TrySendError::Full(r)) => r,
                    Err(TrySendError::Disconnected(_)) => panic!("this section is unreachable"),
                };
                std::thread::yield_now();
            }
        })
        .par_bridge()
        .for_each(|(v, s)| {
            let r = f(v);
            s.send(r).unwrap();
        });
    });

    let sort_handler = std::thread::spawn(move || {
        collect_rx.iter().for_each(|r| {
            let mut result = r.recv().unwrap();
            loop {
                result = match result_tx.try_send(result) {
                    Ok(_) => break,
                    Err(TrySendError::Full(r)) => r,
                    Err(TrySendError::Disconnected(_)) => panic!("this section is unreachable"),
                };
                std::thread::yield_now();
            }
        });
    });

    (
        result_rx,
        Waiter {
            handlers: vec![map_handler, sort_handler],
        },
    )
}

pub fn bounded_parralel_map_iterator<I, F, T, U>(
    bound: usize,
    iter: I,
    f: F,
) -> (impl Iterator<Item = U>, Waiter)
where
    I: Iterator<Item = T> + Send + 'static,
    F: Fn(T) -> U,
    F: Send + Sync + 'static,
    T: Send,
    U: Send + 'static,
{
    let (r, w) = bounded_parralel_map(bound, iter, f);
    (r.into_iter(), w)
}

pub fn bounded_parralel_map_channel<T, U, F>(
    bound: usize,
    f: F,
) -> (SyncSender<T>, Receiver<U>, Waiter)
where
    F: Fn(T) -> U,
    F: Send + Sync + 'static,
    T: Send + 'static,
    U: Send + 'static,
{
    let (source_tx, source_rx) = sync_channel::<T>(0);
    let (result_rx, waiter) = bounded_parralel_map(bound, source_rx.into_iter(), f);
    (source_tx, result_rx, waiter)
}

#[cfg(test)]
mod tests {
    use std::{sync::mpsc::TrySendError, thread::sleep, time::Duration};

    use crate::bounded_parralel_map_channel;

    #[test]
    fn test_bounded_parralel_map_channel() {
        let (result_rx, waiter) = {
            let (source_tx, result_rx, waiter) = bounded_parralel_map_channel(2, |x: u64| {
                sleep(Duration::from_millis(1000u64 - x * 100));
                x.to_string()
            });
            std::thread::spawn(move || {
                (1..10).for_each(|x| {
                    let mut x = x;
                    loop {
                        x = match source_tx.try_send(x) {
                            Ok(_) => break,
                            Err(TrySendError::Full(x)) => x,
                            Err(TrySendError::Disconnected(x)) => x,
                        };
                        std::thread::yield_now();
                    }
                });
            });
            (result_rx, waiter)
        };

        result_rx.iter().for_each(|r| {
            println!("case1 {}", r);
        });
        waiter.wait();
    }
}
