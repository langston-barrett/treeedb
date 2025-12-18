// TODO(#14)

use std::fs;
use std::fs::File;
use std::io;
use std::path::PathBuf;

use tree_sitter::Node;

use super::consumer::FactConsumer;

#[derive(Debug)]
pub struct NarrowCsvConsumer {
    node_id: csv::Writer<File>,
}

impl NarrowCsvConsumer {
    pub fn new(dir: PathBuf) -> Result<Self, io::Error> {
        fs::create_dir_all(&dir)?;
        Ok(NarrowCsvConsumer {
            node_id: csv::Writer::from_writer(File::create(dir.join("node_id.csv"))?),
        })
    }
}

impl FactConsumer for NarrowCsvConsumer {
    type Err = csv::Error;

    fn field(
        &mut self,
        _parent: &Node<'_>,
        _name: &'static str,
        _child: &Node<'_>,
    ) -> Result<(), Self::Err> {
        Ok(())
    }

    fn child(
        &mut self,
        _parent: &Node<'_>,
        _index: u32,
        _child: &Node<'_>,
    ) -> Result<(), Self::Err> {
        Ok(())
    }

    fn node(&mut self, node: &Node<'_>, _source: &[u8]) -> Result<(), Self::Err> {
        let id = node.id();
        self.node_id
            .write_record([&id.to_string(), &id.to_string()])?;
        Ok(())
    }
}
