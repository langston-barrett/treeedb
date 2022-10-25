use tree_sitter::Node;

use super::consumer::FactConsumer;

pub struct CountConsumer {
    count: usize,
}

impl CountConsumer {
    pub fn new() -> Self {
        CountConsumer { count: 0 }
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

impl Default for CountConsumer {
    fn default() -> Self {
        Self::new()
    }
}

impl FactConsumer for CountConsumer {
    type Err = ();

    fn node(&mut self, _node: &Node) -> Result<(), Self::Err> {
        self.count += 1;
        Ok(())
    }

    fn field(
        &mut self,
        _parent: &Node,
        _name: &'static str,
        _child: &Node,
    ) -> Result<(), Self::Err> {
        Ok(())
    }
}
