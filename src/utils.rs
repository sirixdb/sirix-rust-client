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
