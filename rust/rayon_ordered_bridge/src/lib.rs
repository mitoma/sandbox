use std::{
    sync::mpsc::{channel, sync_channel, Receiver, SyncSender},
    thread::spawn,
};

use rayon::prelude::{ParallelBridge, ParallelIterator};

fn internal_bounded_parralel<I, F, T, U>(bound: usize, iter: I, f: F) -> Receiver<U>
where
    I: Iterator<Item = T> + Send + 'static,
    F: Fn(T) -> U,
    F: Send + Sync + 'static,
    T: Send,
    U: Send + 'static,
{
    let (collect_tx, collect_rx) = sync_channel::<Receiver<U>>(bound);
    let (result_tx, result_rx) = sync_channel::<U>(bound);

    spawn(move || {
        iter.map(|v| {
            let (s, r) = channel::<U>();
            collect_tx.send(r).unwrap();
            (v, s)
        })
        .par_bridge()
        .for_each(|(v, s)| {
            let r = f(v);
            s.send(r).unwrap();
        });
    });

    spawn(move || {
        collect_rx.iter().for_each(|r| {
            result_tx.send(r.recv().unwrap()).unwrap();
        });
    });

    result_rx
}

pub fn bounded_parralel_mapped_iterator<I, F, T, U>(
    bound: usize,
    iter: I,
    f: F,
) -> impl Iterator<Item = U>
where
    I: Iterator<Item = T> + Send + 'static,
    F: Fn(T) -> U,
    F: Send + Sync + 'static,
    T: Send,
    U: Send + 'static,
{
    internal_bounded_parralel(bound, iter, f).into_iter()
}

pub fn bounded_parralel_mapped_channel<T, U, F>(bound: usize, f: F) -> (SyncSender<T>, Receiver<U>)
where
    F: Fn(T) -> U,
    F: Send + Sync + 'static,
    T: Send + 'static,
    U: Send + 'static,
{
    let (source_tx, source_rx) = sync_channel::<T>(0);
    (
        source_tx,
        internal_bounded_parralel(bound, source_rx.into_iter(), f),
    )
}

pub trait OrderedParallelBridge: Sized {
    fn ordered_parallel_receiver(self) -> OrderedParallelReceiver<Self>;
}

impl<T> OrderedParallelBridge for Receiver<T> {
    fn ordered_parallel_receiver(self) -> OrderedParallelReceiver<Self> {
        OrderedParallelReceiver { recv: self }
    }
}

#[derive(Debug, Clone)]
pub struct OrderedParallelReceiver<Recv> {
    recv: Recv,
}

impl<T: Send + 'static> OrderedParallelReceiver<Receiver<T>> {
    pub fn map<F, U>(self, bound: usize, func: F) -> Receiver<U>
    where
        F: Fn(T) -> U,
        F: Send + Sync + 'static,
        U: Send + 'static,
    {
        let (collect_tx, collect_rx) = sync_channel::<Receiver<U>>(bound);
        let (result_tx, result_rx) = channel::<U>();

        spawn(move || {
            self.recv
                .into_iter()
                .map(|v| {
                    let (s, r) = channel::<U>();
                    collect_tx.send(r).unwrap();
                    (v, s)
                })
                .par_bridge()
                .for_each(|(v, s)| {
                    let r = (func)(v);
                    s.send(r).unwrap();
                });
        });

        spawn(move || {
            collect_rx.iter().for_each(|r| {
                result_tx.send(r.recv().unwrap()).unwrap();
            });
        });

        result_rx
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use crate::{bounded_parralel_mapped_channel, bounded_parralel_mapped_iterator};

    #[test]
    fn test_bounded_parralel_map_channel() {
        let result_rx = {
            let (source_tx, result_rx) = bounded_parralel_mapped_channel(2, |x: u64| {
                sleep(Duration::from_millis(1000u64 - x * 100));
                x.to_string()
            });
            std::thread::spawn(move || {
                (1..10).for_each(|x| {
                    source_tx.send(x).unwrap();
                });
            });
            result_rx
        };

        result_rx.iter().for_each(|r| {
            println!("{}", r);
        });
    }

    #[test]
    fn test_bounded_parralel_map_iter() {
        let sut = vec![1, 2, 3, 4, 5, 6, 7];
        let result: Vec<i32> =
            bounded_parralel_mapped_iterator(3, sut.into_iter(), |x| x * x).collect();
        assert!(result == vec![1, 4, 9, 16, 25, 36, 49]);
    }
}
