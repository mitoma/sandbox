use rayon::prelude::*;
use std::{
    sync::mpsc::{channel, sync_channel, Receiver},
    thread::spawn,
};

pub trait OrderedParallelBridge: Sized {
    fn ordering_par_bridge(self) -> OrderedIterBridge<Self>;
}

impl<T: Send> OrderedParallelBridge for Receiver<T> {
    fn ordering_par_bridge(self) -> OrderedIterBridge<Self> {
        OrderedIterBridge { iter: self }
    }
}

#[derive(Debug, Clone)]
pub struct OrderedIterBridge<T> {
    iter: T,
}

impl<T: Send + 'static> OrderedIterBridge<Receiver<T>> {
    pub fn parallel_map_with_order<F, U>(self, bound: usize, func: F) -> Receiver<U>
    where
        F: Fn(T) -> U,
        F: Send + Sync + 'static,
        U: Send + Sync + 'static,
    {
        let (collect_tx, collect_rx) = sync_channel::<Receiver<U>>(bound);
        let (result_tx, result_rx) = channel::<U>();

        let source = self.iter;

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
    use std::{sync::mpsc::channel, thread::sleep, time::Duration};

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
            .ordering_par_bridge()
            .parallel_map_with_order(10, |x| {
                sleep(Duration::from_millis(1000 - x * 100));
                x
            })
            .iter()
            .for_each(|r| {
                println!("{}", r);
            });
    }
}
