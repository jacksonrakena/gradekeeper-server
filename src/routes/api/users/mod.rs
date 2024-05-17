use serde::Serialize;

pub(crate) mod me;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerMetaInfo {
    version: String,
    name: String
}
fn gather_meta_info() -> ServerMetaInfo {
    ServerMetaInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        name: env!("CARGO_PKG_NAME").to_string()
    }
}