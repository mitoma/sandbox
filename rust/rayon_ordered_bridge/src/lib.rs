use rayon::prelude::*;
use std::{
    sync::mpsc::{channel, sync_channel, Receiver},
    thread::spawn,
};

pub trait OrderedParallelBridge: Sized {
    /// Creates a bridge from this type to a `ParallelIterator`.
    fn ordering_par_bridge(self) -> OrderedIterBridge<Self>;
}

impl<T: Iterator + Send> OrderedParallelBridge for T
where
    T::Item: Send,
{
    fn ordering_par_bridge(self) -> OrderedIterBridge<Self> {
        OrderedIterBridge { iter: self }
    }
}

#[derive(Debug, Clone)]
pub struct OrderedIterBridge<Iter> {
    iter: Iter,
}

impl<Iter: Iterator + Send + 'static> OrderedIterBridge<Iter>
where
    Iter::Item: Send,
{
    pub fn parallel_map_with_order<F, U>(self, bound: usize, func: F) -> Receiver<U>
    where
        F: Fn(Iter::Item) -> U,
        F: Send + Sync + 'static,
        U: Send + Sync + 'static,
    {
        let (collect_tx, collect_rx) = sync_channel::<Receiver<U>>(bound);
        let (result_tx, result_rx) = channel::<U>();

        let source = self.iter;

        spawn(move || {
            source
                .into_iter()
                .map(|v: <Iter as Iterator>::Item| {
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

pub fn calc<T, U, F>(source: Receiver<T>, closure: F, bound: usize) -> Receiver<U>
where
    T: Send + Sync + 'static,
    U: Send + Sync + 'static,
    F: Fn(T) -> U,
    F: Send + Sync + 'static,
{
    let (collect_tx, collect_rx) = sync_channel::<Receiver<U>>(bound);
    let (result_tx, result_rx) = channel::<U>();

    spawn(move || {
        source
            .into_iter()
            .map(|v| {
                let (s, r) = channel::<U>();
                collect_tx.send(r).unwrap();
                (v, s)
            })
            .par_bridge()
            .for_each(|(v, s)| {
                let r = (closure)(v);
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

#[cfg(test)]
mod tests {
    use std::{sync::mpsc::channel, thread::sleep, time::Duration};

    use crate::{calc, OrderedParallelBridge};

    #[test]
    fn test_calc() {
        // 最初のストリームを表す channel
        let (source_tx, source_rx) = channel::<u64>();
        (1..10).for_each(|x| {
            source_tx.send(x).unwrap();
        });

        drop(source_tx);

        let result = calc(
            source_rx,
            |x| {
                sleep(Duration::from_millis(1000 - x * 100));
                x
            },
            0,
        );

        result.iter().for_each(|r| {
            println!("{}", r);
        });
    }

    #[test]

    fn test_ordering_par_bridge() {
        let sut = (1..1000).ordering_par_bridge();
        sut.parallel_map_with_order(100, |i| {
            sleep(Duration::from_millis(1000 - i));
            i * 2
        })
        .iter()
        .for_each(|r| {
            println!("{}", r);
        })
    }
}
