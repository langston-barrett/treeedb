use tree_sitter::Node;

// TODO(lb): Bounds error for source text, also check bounds
// TODO(lb): Conflicting definitions for 'from' :-(
// use thiserror::Error;
// #[derive(Error, Debug)]
// pub enum ConsumerError<FC: FactConsumer<Err = E>, E> {
//     #[error("UTF-8 error")]
//     Utf8(#[from] core::str::Utf8Error),
//     #[error("FactConsumer error")]
//     Err(#[from] E),
// }

// TODO(lb): Take in a source file name as well
pub trait FactConsumer {
    type Err;

    fn field(&mut self, parent: &Node, name: &'static str, child: &Node) -> Result<(), Self::Err>;

    fn node(&mut self, node: &Node, source: &[u8]) -> Result<(), Self::Err>;
}
