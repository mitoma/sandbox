use std::{
    sync::mpsc::{channel, sync_channel, Receiver},
    thread::spawn,
};

use rayon::prelude::{ParallelBridge, ParallelIterator};

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
    use std::{
        sync::mpsc::{channel, Receiver},
        thread::sleep,
        time::Duration,
    };

    use crate::OrderedParallelBridge;

    #[test]
    fn test_ordering_par_bridge() {
        let source_rx = {
            let (source_tx, source_rx) = channel::<u64>();
            (1..10).for_each(|x| {
                source_tx.send(x).unwrap();
            });
            source_rx
        };
        source_rx
            .ordered_parallel_receiver()
            .map(10, |x| {
                sleep(Duration::from_millis(100 - x * 10));
                x
            })
            .iter()
            .for_each(|r| {
                println!("{}", r);
            });
    }

    #[test]
    fn test_ordering_par_bridge_2() {
        let source_rx = {
            let (source_tx, source_rx) = channel::<u64>();
            (1..10).for_each(|x| {
                source_tx.send(x).unwrap();
            });
            source_rx
        };
        let r = other_func(source_rx);
        r.iter().for_each(|r| {
            println!("{}", r);
        });
    }

    fn other_func(rx: Receiver<u64>) -> Receiver<String> {
        rx.ordered_parallel_receiver().map(1, |x| x.to_string())
    }
}
