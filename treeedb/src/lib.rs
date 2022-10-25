use tree_sitter::Tree;

pub mod consumer;
pub mod count;
pub mod narrow;
pub mod wide;

pub fn facts<E>(fc: &mut impl consumer::FactConsumer<Err = E>, tree: Tree) -> Result<(), E> {
    let mut nodes = vec![tree.root_node()];
    let mut cursor = tree.walk();
    while let Some(node) = nodes.pop() {
        fc.node(&node)?;
        for (i, child) in node.children(&mut cursor).enumerate() {
            if let Some(name) = node.field_name_for_child(i as u32) {
                fc.field(&node, name, &child)?;
            }
        }
        for child in node.named_children(&mut cursor) {
            nodes.push(child);
        }
    }
    Ok(())
}
