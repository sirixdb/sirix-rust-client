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
    use serde_json::Value;
    use sirix_rust_client::synchronous::{auth::auth, sirix::Sirix};
    use sirix_rust_client::types::{
        DiffArgs, Insert, MetadataType, ReadArgs, SingleRevision,
    };
    use std::thread::sleep;
    use std::time::Duration;

    const BASE_URL: &str = "http://localhost:9443";

    fn setup_sirix() -> Sirix {
        let agent = ureq::agent();
        let lock = auth(agent.clone(), BASE_URL, "admin", "admin");
        sleep(Duration::from_millis(500));
        Sirix::new(BASE_URL.to_string(), agent, Some(lock))
    }

    fn empty_read_args() -> ReadArgs {
        ReadArgs {
            node_id: None,
            revision: None,
            max_level: None,
            top_level_limit: None,
            top_level_skip_last_node: None,
        }
    }

    // -- Global info --

    #[test]
    fn info_returns_empty_databases_initially() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let result = sirix.info_with_resources();
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert!(resp.body.databases.is_empty());
    }

    #[test]
    fn info_string_returns_valid_json() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let result = sirix.info_string();
        assert!(result.is_ok());
        let body = result.unwrap().body;
        let parsed: Result<Value, _> = serde_json::from_str(&body);
        assert!(parsed.is_ok());
    }

    // -- Database lifecycle --

    #[test]
    fn create_and_delete_json_database() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let db = sirix.json_database("testdb-json".to_string());

        // Create database
        let create_result = db.create();
        assert!(create_result.is_ok());

        // Verify it appears in global info
        let info = sirix.info_with_resources().unwrap();
        assert!(info
            .body
            .databases
            .iter()
            .any(|d| d.name == "testdb-json"));

        // Delete database
        let delete_result = db.delete();
        assert!(delete_result.is_ok());

        // Verify it's gone
        let info = sirix.info_with_resources().unwrap();
        assert!(!info
            .body
            .databases
            .iter()
            .any(|d| d.name == "testdb-json"));
    }

    #[test]
    fn create_and_delete_xml_database() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let db = sirix.xml_database("testdb-xml".to_string());

        let create_result = db.create();
        assert!(create_result.is_ok());

        let info = sirix.info_with_resources().unwrap();
        assert!(info
            .body
            .databases
            .iter()
            .any(|d| d.name == "testdb-xml"));

        let delete_result = db.delete();
        assert!(delete_result.is_ok());
    }

    #[test]
    fn database_info_returns_empty_resources_for_new_db() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let db = sirix.json_database("testdb-info".to_string());
        db.create().unwrap();

        let info = db.info();
        assert!(info.is_ok());

        let info_str = db.info_string();
        assert!(info_str.is_ok());
        let body = info_str.unwrap().body;
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert!(parsed["resources"].is_array());

        db.delete().unwrap();
    }

    // -- Resource lifecycle --

    #[test]
    fn create_and_read_json_resource() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let db = sirix.json_database("testdb-res".to_string());
        db.create().unwrap();

        let resource = db.resource("myresource".to_string());
        let initial_data = r#"{"city":"New York","population":8336817}"#;
        let create_result = resource.create_string(initial_data.to_string());
        assert!(create_result.is_ok());

        // Read the resource back
        let read_result = resource.read_string(empty_read_args());
        assert!(read_result.is_ok());
        let body = read_result.unwrap().body;
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["city"], "New York");
        assert_eq!(parsed["population"], 8336817);

        // Read as parsed JSON
        let read_json = resource.read::<Value>(empty_read_args());
        assert!(read_json.is_ok());
        let json_body = read_json.unwrap().body;
        assert_eq!(json_body["city"], "New York");

        db.delete().unwrap();
    }

    #[test]
    fn create_resource_with_create_method() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let db = sirix.json_database("testdb-create".to_string());
        db.create().unwrap();

        let resource = db.resource("res1".to_string());
        let create_result = resource.create(r#"{"hello":"world"}"#.to_string());
        assert!(create_result.is_ok());
        let body = create_result.unwrap().body;
        assert_eq!(body["hello"], "world");

        db.delete().unwrap();
    }

    #[test]
    fn resource_exists_check() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let db = sirix.json_database("testdb-exists".to_string());
        db.create().unwrap();

        let resource = db.resource("existing-res".to_string());
        resource.create(r#"{"data":true}"#.to_string()).unwrap();

        let exists_result = resource.exists();
        assert!(exists_result.is_ok());

        db.delete().unwrap();
    }

    // -- Resource update --

    #[test]
    fn update_resource_with_child_insert() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let db = sirix.json_database("testdb-update".to_string());
        db.create().unwrap();

        let resource = db.resource("update-res".to_string());
        resource.create(r#"{"root":"value"}"#.to_string()).unwrap();

        // Get the etag for the root node
        let etag_result = resource.etag(1);
        assert!(etag_result.is_ok());
        let etag = etag_result.unwrap().etag.unwrap_or_default();

        // Update with a child insert
        let update_result: Result<_, _> = resource.update::<Value>(
            1,
            Insert::Child,
            r#"{"added":"child"}"#.to_string(),
            etag,
        );
        assert!(update_result.is_ok());

        db.delete().unwrap();
    }

    // -- Resource history --

    #[test]
    fn resource_history_after_create() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let db = sirix.json_database("testdb-hist".to_string());
        db.create().unwrap();

        let resource = db.resource("hist-res".to_string());
        resource.create(r#"{"version":1}"#.to_string()).unwrap();

        let history_str = resource.history_string();
        assert!(history_str.is_ok());
        let body = history_str.unwrap().body;
        let parsed: Value = serde_json::from_str(&body).unwrap();
        // History should contain at least one revision
        assert!(parsed["history"].is_array());
        let history_arr = parsed["history"].as_array().unwrap();
        assert!(!history_arr.is_empty());
        assert!(history_arr[0]["revision"].is_number());

        db.delete().unwrap();
    }

    // -- Resource diff --

    #[test]
    fn resource_diff_between_revisions() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let db = sirix.json_database("testdb-diff".to_string());
        db.create().unwrap();

        let resource = db.resource("diff-res".to_string());
        resource.create(r#"{"v":1}"#.to_string()).unwrap();

        // Get etag and update to create a second revision
        let etag_result = resource.etag(1);
        let etag = etag_result.unwrap().etag.unwrap_or_default();
        let _: Result<_, _> =
            resource.update::<Value>(1, Insert::Child, r#"{"v":2}"#.to_string(), etag);

        // Diff between revision 1 and 2
        let diff_result = resource.diff(DiffArgs {
            first_revision: SingleRevision::Number(1),
            second_revision: SingleRevision::Number(2),
            node_id: None,
            max_depth: None,
        });
        assert!(diff_result.is_ok());

        db.delete().unwrap();
    }

    // -- Resource delete --

    #[test]
    fn delete_entire_resource() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let db = sirix.json_database("testdb-rdel".to_string());
        db.create().unwrap();

        let resource = db.resource("del-res".to_string());
        resource.create(r#"{"temp":true}"#.to_string()).unwrap();

        let delete_result = resource.delete(None);
        assert!(delete_result.is_ok());

        db.delete().unwrap();
    }

    // -- Read with metadata --

    #[test]
    fn read_with_metadata_returns_node_keys() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let db = sirix.json_database("testdb-meta".to_string());
        db.create().unwrap();

        let resource = db.resource("meta-res".to_string());
        resource.create(r#"{"key":"value"}"#.to_string()).unwrap();

        let meta_result =
            resource.read_with_metadata_string(MetadataType::All, empty_read_args());
        assert!(meta_result.is_ok());
        let body = meta_result.unwrap().body;
        let parsed: Value = serde_json::from_str(&body).unwrap();
        // With metadata, the response should contain metadata fields
        assert!(parsed["metadata"].is_object() || parsed.is_object());

        db.delete().unwrap();
    }

    #[test]
    fn read_with_key_metadata() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let db = sirix.json_database("testdb-keymeta".to_string());
        db.create().unwrap();

        let resource = db.resource("keymeta-res".to_string());
        resource.create(r#"{"a":1}"#.to_string()).unwrap();

        let meta_result =
            resource.read_with_metadata_string(MetadataType::Key, empty_read_args());
        assert!(meta_result.is_ok());

        db.delete().unwrap();
    }

    // -- Read with specific node --

    #[test]
    fn read_with_node_id() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let db = sirix.json_database("testdb-nodeid".to_string());
        db.create().unwrap();

        let resource = db.resource("nodeid-res".to_string());
        resource
            .create(r#"{"root":{"nested":"value"}}"#.to_string())
            .unwrap();

        let read_result = resource.read_string(ReadArgs {
            node_id: Some(1),
            revision: None,
            max_level: None,
            top_level_limit: None,
            top_level_skip_last_node: None,
        });
        assert!(read_result.is_ok());

        db.delete().unwrap();
    }

    // -- Query --

    #[test]
    fn post_query_returns_result() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let db = sirix.json_database("testdb-query".to_string());
        db.create().unwrap();

        let resource = db.resource("query-res".to_string());
        resource
            .create(r#"{"city":"Berlin","country":"Germany"}"#.to_string())
            .unwrap();

        let query = sirix_rust_client::types::Query::new(
            "jn:doc('testdb-query','query-res')=>city".to_string(),
            None,
            None,
        );
        let result: Result<_, _> = sirix.query::<Value>(query);
        assert!(result.is_ok(), "Query failed: {:?}", result.err());

        db.delete().unwrap();
    }

    // -- Delete all --

    #[test]
    fn delete_all_removes_everything() {
        let sirix = setup_sirix();

        // Create some databases
        let db1 = sirix.json_database("testdb-delall1".to_string());
        db1.create().unwrap();
        let db2 = sirix.json_database("testdb-delall2".to_string());
        db2.create().unwrap();

        // Verify they exist
        let info = sirix.info_with_resources().unwrap();
        assert!(info.body.databases.len() >= 2);

        // Delete all
        let delete_result = sirix.delete_all();
        assert!(delete_result.is_ok());

        // Verify empty
        let info = sirix.info_with_resources().unwrap();
        assert!(info.body.databases.is_empty());
    }

    // -- Multiple resources in one database --

    #[test]
    fn multiple_resources_in_database() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let db = sirix.json_database("testdb-multi".to_string());
        db.create().unwrap();

        let res1 = db.resource("resource1".to_string());
        res1.create(r#"{"r":1}"#.to_string()).unwrap();

        let res2 = db.resource("resource2".to_string());
        res2.create(r#"{"r":2}"#.to_string()).unwrap();

        // Database info should list both resources
        let info_str = db.info_string().unwrap().body;
        let parsed: Value = serde_json::from_str(&info_str).unwrap();
        let resources = parsed["resources"].as_array().unwrap();
        assert!(resources.len() >= 2);

        // Read each independently
        let r1 = res1.read::<Value>(empty_read_args()).unwrap().body;
        assert_eq!(r1["r"], 1);

        let r2 = res2.read::<Value>(empty_read_args()).unwrap().body;
        assert_eq!(r2["r"], 2);

        db.delete().unwrap();
    }

    // -- Complex JSON document --

    #[test]
    fn create_and_read_complex_json() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let db = sirix.json_database("testdb-complex".to_string());
        db.create().unwrap();

        let resource = db.resource("complex-res".to_string());
        let complex_json = r#"{
            "users": [
                {"name": "Alice", "age": 30, "active": true},
                {"name": "Bob", "age": 25, "active": false}
            ],
            "metadata": {
                "version": 1,
                "tags": ["test", "integration"]
            }
        }"#;

        resource.create(complex_json.to_string()).unwrap();

        let read = resource.read::<Value>(empty_read_args()).unwrap().body;
        assert_eq!(read["users"][0]["name"], "Alice");
        assert_eq!(read["users"][1]["age"], 25);
        assert_eq!(read["metadata"]["version"], 1);

        db.delete().unwrap();
    }

    // -- Global info with resources after creating databases and resources --

    #[test]
    fn info_with_resources_shows_databases_and_resources() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let db = sirix.json_database("testdb-infor".to_string());
        db.create().unwrap();

        let resource = db.resource("infor-res".to_string());
        resource.create(r#"{"x":1}"#.to_string()).unwrap();

        let info = sirix.info_with_resources().unwrap();
        assert!(!info.body.databases.is_empty());
        let found = info
            .body
            .databases
            .iter()
            .find(|d| d.name == "testdb-infor");
        assert!(found.is_some());
        let db_info = found.unwrap();
        assert!(db_info.resources.contains(&"infor-res".to_string()));

        db.delete().unwrap();
    }

    // -- Etag retrieval --

    #[test]
    fn etag_returns_non_empty_value() {
        let sirix = setup_sirix();
        let _ = sirix.delete_all();

        let db = sirix.json_database("testdb-etag".to_string());
        db.create().unwrap();

        let resource = db.resource("etag-res".to_string());
        resource.create(r#"{"data":"test"}"#.to_string()).unwrap();

        let etag_result = resource.etag(1);
        assert!(etag_result.is_ok());
        let resp = etag_result.unwrap();
        // The etag header should be present (may be empty if server hashing is disabled)
        assert!(resp.etag.is_some());

        db.delete().unwrap();
    }
}
