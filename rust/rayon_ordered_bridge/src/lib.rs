use rayon::prelude::*;
use std::{
    sync::mpsc::{channel, sync_channel, Receiver, SyncSender},
    thread::spawn,
};

struct OrderedBridge<T, U, F>
where
    T: Send + Sync,
    U: Send + Sync,
    F: Fn(T) -> U + Send + Sync,
{
    source: Receiver<T>,
    collect_tx: SyncSender<Receiver<U>>,
    collect_rx: Receiver<Receiver<U>>,
    closure: F,
}

impl<T, U, F> OrderedBridge<T, U, F>
where
    T: Send + Sync + 'static,
    U: Send + Sync + 'static,
    F: Fn(T) -> U,
    F: Send + Sync + 'static,
{
    fn new(source: Receiver<T>, closure: F, bound: usize) -> Self {
        let (collect_tx, collect_rx) = sync_channel::<Receiver<U>>(bound);
        Self {
            source,
            collect_tx,
            collect_rx,
            closure,
        }
    }

    fn calc(self) -> Receiver<U>
    where
        F: Fn(T) -> U + Send + Sync,
    {
        let (result_tx, result_rx) = channel::<U>();

        spawn(move || {
            self.source
                .into_iter()
                .map(|v| {
                    let (s, r) = channel::<U>();
                    self.collect_tx.send(r).unwrap();
                    (v, s)
                })
                .par_bridge()
                .for_each(|(v, s)| {
                    let r = (self.closure)(v);
                    s.send(r).unwrap();
                });
        });
        spawn(move || {
            self.collect_rx.iter().for_each(|r| {
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

    use crate::calc;

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
}
