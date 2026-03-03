use super::types::{ReadArgs, RevisionArg, SingleRevision, TwoRevisions};

pub fn build_read_params(read_args: ReadArgs) -> Vec<(String, String)> {
    let mut params: Vec<(String, String)> = Vec::new();
    match read_args.node_id {
        Some(node_id) => {
            params.push(("nodeId".to_owned(), node_id.to_string()));
        }
        None => (),
    };
    match read_args.max_level {
        Some(max_level) => {
            params.push(("maxLevel".to_owned(), max_level.to_string()));
        }
        None => (),
    };
    match read_args.top_level_limit {
        Some(top_level_limit) => {
            params.push(("nextTopLevelNodes".to_owned(), top_level_limit.to_string()));
        }
        None => (),
    };
    match read_args.top_level_skip_last_node {
        Some(top_level_skip_last_node) => {
            params.push((
                "lastTopLevelNodeKey".to_owned(),
                top_level_skip_last_node.to_string(),
            ));
        }
        None => (),
    };
    match read_args.revision {
        Some(revision) => match revision {
            RevisionArg::SingleRevision(revision) => match revision {
                SingleRevision::Number(revision) => {
                    params.push(("revision".to_owned(), revision.to_string()));
                }
                SingleRevision::Timestamp(revision) => {
                    params.push(("revision-timestamp".to_owned(), revision.to_string()));
                }
            },
            RevisionArg::TwoRevisions(revisions) => {
                match revisions {
                    TwoRevisions::Number(first_revision, second_revision) => {
                        params.push(("start-revision".to_owned(), first_revision.to_string()));
                        params.push(("end-revision".to_owned(), second_revision.to_string()));
                    }
                    TwoRevisions::Timestamp(first_revision, second_revision) => {
                        params.push(("start-revision-timestamp".to_owned(), first_revision));
                        params.push(("end-revision-timestamp".to_owned(), second_revision));
                    }
                };
            }
        },
        None => {}
    };
    return params;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_args() -> ReadArgs {
        ReadArgs {
            node_id: None,
            revision: None,
            max_level: None,
            top_level_limit: None,
            top_level_skip_last_node: None,
        }
    }

    #[test]
    fn no_args_returns_empty_params() {
        let params = build_read_params(empty_args());
        assert!(params.is_empty());
    }

    #[test]
    fn node_id_produces_node_id_param() {
        let args = ReadArgs {
            node_id: Some(42),
            ..empty_args()
        };
        let params = build_read_params(args);
        assert_eq!(params, vec![("nodeId".to_string(), "42".to_string())]);
    }

    #[test]
    fn max_level_produces_max_level_param() {
        let args = ReadArgs {
            max_level: Some(5),
            ..empty_args()
        };
        let params = build_read_params(args);
        assert_eq!(params, vec![("maxLevel".to_string(), "5".to_string())]);
    }

    #[test]
    fn top_level_limit_produces_next_top_level_nodes_param() {
        let args = ReadArgs {
            top_level_limit: Some(10),
            ..empty_args()
        };
        let params = build_read_params(args);
        assert_eq!(
            params,
            vec![("nextTopLevelNodes".to_string(), "10".to_string())]
        );
    }

    #[test]
    fn top_level_skip_last_node_produces_last_top_level_node_key_param() {
        let args = ReadArgs {
            top_level_skip_last_node: Some(7),
            ..empty_args()
        };
        let params = build_read_params(args);
        assert_eq!(
            params,
            vec![("lastTopLevelNodeKey".to_string(), "7".to_string())]
        );
    }

    #[test]
    fn single_revision_number_produces_revision_param() {
        let args = ReadArgs {
            revision: Some(RevisionArg::SingleRevision(SingleRevision::Number(3))),
            ..empty_args()
        };
        let params = build_read_params(args);
        assert_eq!(params, vec![("revision".to_string(), "3".to_string())]);
    }

    #[test]
    fn single_revision_timestamp_produces_revision_timestamp_param() {
        let args = ReadArgs {
            revision: Some(RevisionArg::SingleRevision(SingleRevision::Timestamp(
                "2021-01-01T00:00:00Z".to_string(),
            ))),
            ..empty_args()
        };
        let params = build_read_params(args);
        assert_eq!(
            params,
            vec![(
                "revision-timestamp".to_string(),
                "2021-01-01T00:00:00Z".to_string()
            )]
        );
    }

    #[test]
    fn two_revisions_number_produces_start_and_end_params() {
        let args = ReadArgs {
            revision: Some(RevisionArg::TwoRevisions(TwoRevisions::Number(1, 5))),
            ..empty_args()
        };
        let params = build_read_params(args);
        assert_eq!(
            params,
            vec![
                ("start-revision".to_string(), "1".to_string()),
                ("end-revision".to_string(), "5".to_string()),
            ]
        );
    }

    #[test]
    fn two_revisions_timestamp_produces_start_and_end_timestamp_params() {
        let args = ReadArgs {
            revision: Some(RevisionArg::TwoRevisions(TwoRevisions::Timestamp(
                "2021-01-01".to_string(),
                "2021-06-01".to_string(),
            ))),
            ..empty_args()
        };
        let params = build_read_params(args);
        assert_eq!(
            params,
            vec![
                (
                    "start-revision-timestamp".to_string(),
                    "2021-01-01".to_string()
                ),
                (
                    "end-revision-timestamp".to_string(),
                    "2021-06-01".to_string()
                ),
            ]
        );
    }

    #[test]
    fn multiple_args_produce_all_params_in_order() {
        let args = ReadArgs {
            node_id: Some(1),
            max_level: Some(3),
            top_level_limit: Some(10),
            top_level_skip_last_node: Some(5),
            revision: Some(RevisionArg::SingleRevision(SingleRevision::Number(2))),
        };
        let params = build_read_params(args);
        assert_eq!(params.len(), 5);
        assert_eq!(params[0].0, "nodeId");
        assert_eq!(params[1].0, "maxLevel");
        assert_eq!(params[2].0, "nextTopLevelNodes");
        assert_eq!(params[3].0, "lastTopLevelNodeKey");
        assert_eq!(params[4].0, "revision");
    }
}
