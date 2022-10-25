use tree_sitter::Node;

pub trait FactConsumer {
    type Err;

    fn field(&mut self, parent: &Node, name: &'static str, child: &Node) -> Result<(), Self::Err>;

    fn node(&mut self, node: &Node) -> Result<(), Self::Err>;
}
