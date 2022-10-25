use std::fs::File;
use std::io;
use std::path::PathBuf;

use csv::Writer;
use tree_sitter::Node;

use super::consumer::FactConsumer;

pub struct WideCsvConsumer {
    node: csv::Writer<File>,
    field: csv::Writer<File>,
}

impl WideCsvConsumer {
    pub fn new(node_file_path: PathBuf, field_file_path: PathBuf) -> Result<Self, io::Error> {
        Ok(WideCsvConsumer {
            node: Writer::from_writer(File::create(node_file_path)?),
            field: Writer::from_writer(File::create(field_file_path)?),
        })
    }
}

impl FactConsumer for WideCsvConsumer {
    type Err = csv::Error;

    fn field(&mut self, parent: &Node, name: &'static str, child: &Node) -> Result<(), Self::Err> {
        self.field
            .write_record(&[&parent.id().to_string(), name, &child.id().to_string()])?;
        Ok(())
    }

    fn node(&mut self, node: &Node) -> Result<(), Self::Err> {
        let start = node.start_position();
        let end = node.end_position();
        self.node.write_record(&[
            &node.id().to_string(),
            node.kind(),
            &node.is_named().to_string(),
            &node.is_extra().to_string(),
            &node.is_error().to_string(),
            &node.is_missing().to_string(),
            &node.start_byte().to_string(),
            &node.end_byte().to_string(),
            &start.row.to_string(),
            &start.column.to_string(),
            &end.row.to_string(),
            &end.column.to_string(),
            // TODO(lb): Text!
        ])?;
        Ok(())
    }
}