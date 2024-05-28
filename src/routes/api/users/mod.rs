use serde::Serialize;

pub(crate) mod me;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerMetaInfo {
    version: String,
    name: String,
    commit_hash: String,
    commit_message: String,
    commit_author_name: String,
    commit_branch: String,
}
fn gather_meta_info() -> ServerMetaInfo {
    ServerMetaInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        name: env!("CARGO_PKG_NAME").to_string(),
        commit_hash: env!("GK_SERVER_COMMIT_HASH").to_string(),
        commit_message: env!("GK_SERVER_COMMIT_MSG").to_string(),
        commit_author_name: env!("GK_SERVER_COMMITTER").to_string(),
        commit_branch: env!("GK_SERVER_BRANCH").to_string(),
    }
}