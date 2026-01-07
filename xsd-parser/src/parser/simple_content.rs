use roxmltree::Node;

use crate::parser::{
    node_parser::parse_node,
    types::RsEntity,
    xsd_elements::{ElementType, XsdNode},
};

pub fn parse_simple_content(node: &Node) -> RsEntity {
    let content = node
        .children()
        .rfind(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
        .expect("Content in simpleContent required");

    parse_node(&content, node)
}
