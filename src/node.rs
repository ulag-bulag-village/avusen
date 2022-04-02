use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteNode {
    #[serde(flatten)]
    pub source: crate::source::Source,
    pub node: Node,
}

pub type NodeChildren = HashMap<String, crate::source::Origin<Node>>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub attention: NodeVisibility,
    pub confidence: NodeVisibility,
    pub value: crate::value::Value,
    pub children: NodeChildren,
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
