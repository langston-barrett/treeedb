use std::fs;
use std::fs::File;
use std::io;
use std::path::PathBuf;

use csv::Writer;
use tree_sitter::Node;

use super::consumer::FactConsumer;

pub struct NarrowCsvConsumer {
    node_id: csv::Writer<File>,
}

impl NarrowCsvConsumer {
    pub fn new(dir: PathBuf) -> Result<Self, io::Error> {
        fs::create_dir_all(&dir)?;
        Ok(NarrowCsvConsumer {
            node_id: Writer::from_writer(File::create(dir.join("node_id.csv"))?),
        })
    }
}

impl FactConsumer for NarrowCsvConsumer {
    type Err = csv::Error;

    fn field(
        &mut self,
        _parent: &Node,
        _name: &'static str,
        _child: &Node,
    ) -> Result<(), Self::Err> {
        // TODO(lb)
        Ok(())
    }

    fn node(&mut self, node: &Node, _source: &[u8]) -> Result<(), Self::Err> {
        let id = node.id();
        self.node_id
            .write_record(&[&id.to_string(), &id.to_string()])?;
        Ok(())
    }
}
