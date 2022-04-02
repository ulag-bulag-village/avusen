#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "origin")]
pub enum Origin<T> {
    Local(T),
    Remote(Source),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "source")]
pub enum Source {
    Ipfs {
        author: crate::account::Decoder,
        host: Option<String>,
        path: String,
    },
}
