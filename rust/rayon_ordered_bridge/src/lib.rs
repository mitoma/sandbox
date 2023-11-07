use std::{
    sync::mpsc::{channel, sync_channel, Receiver, Sender},
    thread::spawn,
};

use rayon::prelude::*;

fn calc<T, U, F>(bound: usize, source: Receiver<T>, closure: F) -> Receiver<U>
where
    T: Send,
    U: Send,
    F: Fn(T) -> U + Send + Sync + 'static,
{
    let (collect_tx, collect_rx) = sync_channel::<Receiver<U>>(bound);
    let (result_tx, result_rx) = channel::<U>();

    source
        .into_iter()
        .map(|v| {
            let (s, r) = channel::<U>();
            collect_tx.send(r).unwrap();
            (v, s)
        })
        .par_bridge()
        .for_each(|(v, s)| {
            s.send(closure(v)).unwrap();
        });

    collect_rx.iter().for_each(|r| {
        result_tx.send(r.recv().unwrap()).unwrap();
    });

    result_rx
}
