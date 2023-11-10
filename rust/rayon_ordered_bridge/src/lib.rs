use std::sync::mpsc::{channel, sync_channel, Receiver, SyncSender};

use rayon::{
    prelude::{ParallelBridge, ParallelIterator},
    Scope,
};

pub fn bounded_parralel_map<'a, I, F, T, U>(
    s: &Scope<'a>,
    bound: usize,
    iter: I,
    f: F,
) -> Receiver<U>
where
    I: Iterator<Item = T> + Send + 'a,
    F: Fn(T) -> U,
    F: Send + Sync + 'a,
    T: Send,
    U: Send + 'a,
{
    let (collect_tx, collect_rx) = sync_channel::<Receiver<U>>(bound);
    let (result_tx, result_rx) = sync_channel::<U>(bound);

    s.spawn(move |_| {
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

    s.spawn(move |_| {
        collect_rx.iter().for_each(|r| {
            result_tx.send(r.recv().unwrap()).unwrap();
        });
    });

    result_rx
}

pub fn bounded_parralel_map_iterator<'a, I, F, T, U>(
    s: &Scope<'a>,
    bound: usize,
    iter: I,
    f: F,
) -> impl Iterator<Item = U>
where
    I: Iterator<Item = T> + Send + 'a,
    F: Fn(T) -> U,
    F: Send + Sync + 'a,
    T: Send,
    U: Send + 'a,
{
    bounded_parralel_map(s, bound, iter, f).into_iter()
}

pub fn bounded_parralel_map_channel<'a, T, U, F>(
    s: &Scope<'a>,
    bound: usize,
    f: F,
) -> (SyncSender<T>, Receiver<U>)
where
    F: Fn(T) -> U,
    F: Send + Sync + 'a,
    T: Send + 'a,
    U: Send + 'a,
{
    let (source_tx, source_rx) = sync_channel::<T>(0);
    (
        source_tx,
        bounded_parralel_map(s, bound, source_rx.into_iter(), f),
    )
}

pub fn bounded_parralel_map_scope<'a, I, F, T, U>(
    s: &Scope<'a>,
    bound: usize,
    iter: I,
    f: F,
) -> Receiver<U>
where
    I: Iterator<Item = T> + Send + 'a,
    F: Fn(T) -> U,
    F: Send + Sync + 'a,
    T: Send,
    U: Send + 'a,
{
    let (collect_tx, collect_rx) = sync_channel::<Receiver<U>>(bound);
    let (result_tx, result_rx) = sync_channel::<U>(bound);

    s.spawn(move |_| {
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

    s.spawn(move |_| {
        collect_rx.iter().for_each(|r| {
            result_tx.send(r.recv().unwrap()).unwrap();
        });
    });

    result_rx
}

pub fn bounded_parralel_map_iterator_scope<'a, I, F, T, U>(
    s: &Scope<'a>,
    bound: usize,
    iter: I,
    f: F,
) -> impl Iterator<Item = U>
where
    I: Iterator<Item = T> + Send + 'a,
    F: Fn(T) -> U,
    F: Send + Sync + 'a,
    T: Send,
    U: Send + 'a,
{
    bounded_parralel_map_scope(s, bound, iter, f).into_iter()
}

pub fn bounded_parralel_map_channel_scope<'a, T, U, F>(
    s: &Scope<'a>,
    bound: usize,
    f: F,
) -> (SyncSender<T>, Receiver<U>)
where
    F: Fn(T) -> U,
    F: Send + Sync + 'a,
    T: Send + 'a,
    U: Send + 'a,
{
    let (source_tx, source_rx) = sync_channel::<T>(0);
    (
        source_tx,
        bounded_parralel_map_scope(s, bound, source_rx.into_iter(), f),
    )
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use rayon::scope;

    use crate::{bounded_parralel_map_channel_scope, bounded_parralel_map_iterator_scope};

    #[test]
    fn test_bounded_parralel_map_channel_scope() {
        scope(|s| {
            let result_rx = {
                let (source_tx, result_rx) = bounded_parralel_map_channel_scope(s, 2, |x: u64| {
                    sleep(Duration::from_millis(1000u64 - x * 100));
                    x.to_string()
                });
                s.spawn(move |_| {
                    (1..10).for_each(|x| {
                        source_tx.send(x).unwrap();
                    });
                });
                result_rx
            };

            result_rx.iter().for_each(|r| {
                println!("{}", r);
            });
        });
    }

    #[test]
    fn test_bounded_parralel_map_iterator() {
        scope(|s| {
            let sut = vec![1, 2, 3, 4, 5, 6, 7];

            let result: Vec<i32> =
                bounded_parralel_map_iterator_scope(s, 3, sut.into_iter(), |x| x * x).collect();
            assert!(result == vec![1, 4, 9, 16, 25, 36, 49]);
        });
    }
}
