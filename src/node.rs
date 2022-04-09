use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteNode<'v> {
    #[serde(flatten)]
    pub source: crate::source::Source,
    pub node: Node<'v>,
}

pub type NodeChildren<'v> = HashMap<String, crate::source::Origin<Node<'v>>>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Node<'v> {
    pub attention: NodeVisibility,
    pub confidence: NodeVisibility,
    pub value: crate::value::Value<'v>,
    pub children: NodeChildren<'v>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum NodeVisibility {
    Always,
    Virtually,
    Usually,
    Sometimes,
    Ever,
    Never,
}
