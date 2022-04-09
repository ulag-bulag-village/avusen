#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Function<'v> {
    pub caller: crate::account::Encoder,
    pub program: crate::source::Source,
    pub inputs: crate::node::NodeChildren<'v>,
    pub outputs: crate::node::NodeChildren<'v>,
}
