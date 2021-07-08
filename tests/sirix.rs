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

#[cfg(test)]
#[cfg(feature = "sync")]
mod synchronous {
    use sirix_rust_client::synchronous::{auth::auth, sirix::Sirix};
    #[test]
    fn sirix_info_with_resources() {
        let url = "http://localhost:9443";
        let agent = ureq::agent();

        let lock = auth(agent.clone(), url, "admin", "admin");
        let sirix = Sirix::new(url.to_string(), agent.clone(), Some(lock));
        let result = sirix.info_with_resources();
        assert!(result.is_ok());
        let databases = result.unwrap().body.databases;
        assert!(databases.len() == 0);
    }
}
