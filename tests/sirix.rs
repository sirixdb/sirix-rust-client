#[cfg(test)]
#[cfg(feature = "async")]
mod asynchronous {
    use hyper::http::uri::Uri;
    use sirix_rust_client::asynchronous::auth::auth;
    use sirix_rust_client::asynchronous::client::spawn_client;
    use sirix_rust_client::asynchronous::sirix::Sirix;
    use std::time::Duration;
    #[tokio::test]
    async fn sirix_info_with_resources() {
        // create message channel
        let (sender, receiver) = tokio::sync::mpsc::channel(32);
        spawn_client(hyper::Client::new(), receiver);
        let uri = "http://localhost:9443".parse::<Uri>().unwrap();
        // initiate auth coroutine
        let (watch_rx, _kill_switch) = auth("admin", "admin", &uri.to_string(), sender.clone())
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(1000)).await;
        let sirix = Sirix::new(uri, sender, Some(watch_rx));
        let result = sirix.info_with_resources().await;
        match result {
            Ok(response) => println!("{:#?}", response),
            Err(err) => println!("{}", err),
        }
    }
}
