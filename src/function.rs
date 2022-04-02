#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub caller: crate::account::Encoder,
    pub program: crate::source::Source,
    pub inputs: crate::node::NodeChildren,
    pub outputs: crate::node::NodeChildren,
}
