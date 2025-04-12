#[cfg(test)]
mod tests {
    use std::{
        sync::mpsc::{Receiver, Sender, channel, sync_channel},
        thread::{self, sleep},
        time::Duration,
    };

    #[test]
    fn test_array() {
        let array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut result: Vec<u64> = vec![];

        use rayon::prelude::*;
        array
            .par_iter()
            .map(|x| {
                // calc
                sleep(Duration::from_millis(1000 - (x * 100)));
                *x
            })
            .inspect(|x| println!("{}", x) /* 降順で出力される */)
            .collect_into_vec(&mut result);

        result.iter().for_each(|x| {
            println!("{}", x); /* 昇順で出力される */
        });
    }

    #[test]
    fn test_for_each() {
        let (tx, rx) = channel::<u64>();
        (1..10).for_each(|x| {
            tx.send(x).unwrap();
        });
        drop(tx);

        use rayon::prelude::*;
        rx.into_iter()
            .par_bridge()
            .map(|x| {
                // calc
                sleep(Duration::from_millis(1000 - (x * 100)));
                x
            })
            .for_each(|x| {
                println!("{}", x);
            });
    }

    #[test]
    fn test_collect() {
        let (tx, rx) = channel::<u64>();
        (1..10).for_each(|x| {
            tx.send(x).unwrap();
        });
        drop(tx);

        use rayon::prelude::*;
        let result: Vec<_> = rx
            .into_iter()
            .par_bridge()
            .map(|x| {
                // calc
                sleep(Duration::from_millis(1000 - (x * 100)));
                x
            })
            .collect();

        println!("{:?}", result);
    }

    // rayon で重たい計算処理は並列処理しつつ、結果の受け取りは順序保障する
    #[test]
    fn test_chan_chan() {
        // 最初のストリームを表す channel
        let (source_tx, source_rx) = channel::<(u64, Sender<u64>)>();
        // 処理後のストリームを表す channel
        let (collect_tx, collect_rx) = channel::<Receiver<u64>>();

        (1..10).for_each(|x| {
            let (t, r) = channel::<u64>();
            source_tx.send((x, t)).unwrap();
            collect_tx.send(r).unwrap();
        });

        // drop しないと Receiver が無限に待つので drop する
        drop(source_tx);
        drop(collect_tx);

        use rayon::prelude::*;
        source_rx
            .into_iter()
            .par_bridge()
            .map(|(x, sender)| {
                // ここが重たい計算計算処理
                sleep(Duration::from_millis(1000 - (x * 100)));
                (x, sender)
            })
            .for_each(|(x, sender)| {
                sender.send(x).unwrap();
            });

        collect_rx.into_iter().for_each(|r| {
            println!("{}", r.recv().unwrap());
        });
    }

    // rayon で重たい計算処理は並列処理しつつ、結果の受け取りは順序保障する。また、同時処理数を制限する。
    #[test]
    fn test_chan_chan_bounded() {
        let (source_rx, collect_rx) = {
            // 最初のストリームを表す channel
            let (source_tx, source_rx) = channel::<(u64, Sender<u64>)>();
            // 処理後のストリームを表す channel
            let (collect_tx, collect_rx) = sync_channel::<Receiver<u64>>(5);

            thread::spawn(move || {
                (1..10).for_each(|x| {
                    println!("store: {}", x);
                    let (t, r) = channel::<u64>();
                    source_tx.send((x, t)).unwrap();
                    collect_tx.send(r).unwrap();
                });
            });

            (source_rx, collect_rx)
        };

        thread::spawn(move || {
            use rayon::prelude::*;
            source_rx
                .into_iter()
                .par_bridge()
                .map(|(x, sender)| {
                    println!("calc: {}", x);
                    // ここが重たい計算計算処理
                    sleep(Duration::from_millis(1000 - (x * 100)));
                    (x, sender)
                })
                .for_each(|(x, sender)| {
                    sender.send(x).unwrap();
                });
        });

        collect_rx.into_iter().for_each(|r| {
            println!("{}", r.recv().unwrap());
        });
    }
}
