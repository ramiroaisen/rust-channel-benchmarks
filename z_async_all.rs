#[allow(unused)]
async fn mpmc<T: BenchType + 'static>(cap: Option<usize>) {
    let (tx, rx) = new(cap);
    let mut list = Vec::new();
    for _ in 0..THREADS {
        let tx = tx.clone();
        let h = tokio::spawn(async move {
            for i in 1..MESSAGES / THREADS + 1 {
                tx.send(T::new(i)).await.unwrap();
            }
        });
        list.push(h);
    }

    for _ in 0..THREADS {
        let rx = rx.clone();
        let h = tokio::spawn(async move {
            for _ in 0..MESSAGES / THREADS {
                rx.recv().await.unwrap().test()
            }
        });
        list.push(h);
    }

    for h in list {
        h.await.unwrap();
    }
}

#[allow(unused)]
async fn mpsc<T: BenchType + 'static>(cap: Option<usize>) {
    let (tx, rx) = new(cap);
    let mut list = Vec::new();

    for _ in 0..THREADS {
        let tx = tx.clone();
        let h = tokio::spawn(async move {
            for i in 1..MESSAGES / THREADS + 1 {
                tx.send(T::new(i)).await.unwrap();
            }
        });
        list.push(h);
    }

    list.push(tokio::spawn(async move {
        for _ in 0..MESSAGES {
            rx.recv().await.unwrap().test()
        }
    }));

    for h in list {
        h.await.unwrap();
    }
}

#[allow(unused)]
async fn seq<T: BenchType + 'static>(cap: Option<usize>) {
    let (tx, rx) = new(cap);

    let h = tokio::spawn(async move {
        for i in 1..MESSAGES + 1 {
            tx.send(T::new(i)).await.unwrap();
        }

        for _ in 0..MESSAGES {
            rx.recv().await.unwrap().test()
        }
    });

    h.await.unwrap();
}

#[allow(unused)]
async fn spsc<T: BenchType + 'static>(cap: Option<usize>) {
    let (tx, rx) = new(cap);

    let htx = tokio::spawn(async move {
        for i in 1..MESSAGES + 1 {
            tx.send(T::new(i)).await.unwrap();
        }
    });
    let hrx = tokio::spawn(async move {
        for _ in 0..MESSAGES {
            rx.recv().await.unwrap().test()
        }
    });

    htx.await.unwrap();
    hrx.await.unwrap();
}
