use std::collections::{HashMap, HashSet};

struct Tag {
    name: String,
    attributes: HashMap<String, String>,
}

impl Tag {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(a) => a.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

enum NodeType {
    Element(Tag),
    Text(String),
}

struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

impl Node {
    pub fn new_text(s: String) -> Node {
        Node {
            children: Vec::new(),
            node_type: NodeType::Text(s),
        }
    }

    pub fn new_element(
        name: String, attributes: HashMap<String, String>, children: Vec<Node>,
    ) -> Node {
        Node {
            children,
            node_type: NodeType::Element(Tag { name, attributes }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO
}
