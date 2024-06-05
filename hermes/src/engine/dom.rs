use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
enum NodeType {
    Element(Tag),
    Text(String),
}

#[derive(Debug, PartialEq, Clone)]
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
    use super::*; //{{{

    fn build_dom() -> Node {
        let mut att = HashMap::new();
        att.insert("id".to_string(), "my_btn".to_string());
        att.insert("class".to_string(), "btn p-6".to_string());

        let text1 = Node::new_text("Hello World".to_string());
        let text2 = Node::new_text("Button".to_string());
        let button = Node::new_element("button".to_string(), att, vec![text2]);

        Node::new_element("body".to_string(), HashMap::new(), vec![text1, button])
    }

    #[test]
    fn test_tag_id() {
        let mut attributes = HashMap::new();
        attributes.insert("id".to_string(), "unique_id".to_string());
        let tag = Tag {
            name: "div".to_string(),
            attributes,
        };

        assert_eq!(tag.id(), Some(&"unique_id".to_string()));
    }

    #[test]
    fn test_tag_id_none() {
        let tag = Tag {
            name: "div".to_string(),
            attributes: HashMap::new(),
        };

        assert_eq!(tag.id(), None);
    }

    #[test]
    fn test_tag_classes() {
        let mut attributes = HashMap::new();
        attributes.insert("class".to_string(), "class1 class2".to_string());
        let tag = Tag {
            name: "div".to_string(),
            attributes,
        };

        let classes: HashSet<&str> = ["class1", "class2"].iter().cloned().collect();
        assert_eq!(tag.classes(), classes);
    }

    #[test]
    fn test_tag_classes_empty() {
        let tag = Tag {
            name: "div".to_string(),
            attributes: HashMap::new(),
        };

        let classes: HashSet<&str> = HashSet::new();
        assert_eq!(tag.classes(), classes);
    }

    #[test]
    fn test_node_new_text() {
        let node = Node::new_text("Sample text".to_string());

        match node.node_type {
            NodeType::Text(ref text) => assert_eq!(text, "Sample text"),
            _ => panic!("Expected text node"),
        }
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_node_new_element() {
        let mut attributes = HashMap::new();
        attributes.insert("key".to_string(), "value".to_string());
        let children = vec![Node::new_text("Child".to_string())];
        let node = Node::new_element("div".to_string(), attributes.clone(), children.clone());

        match node.node_type {
            NodeType::Element(ref tag) => {
                assert_eq!(tag.name, "div");
                assert_eq!(tag.attributes, attributes);
            }
            _ => panic!("Expected element node"),
        }
        assert_eq!(node.children, children);
    }

    #[test]
    fn test_build_dom() {
        let dom = build_dom();

        match dom.node_type {
            NodeType::Element(ref tag) => {
                assert_eq!(tag.name, "body");
            }
            _ => panic!("Expected body element"),
        }

        assert_eq!(dom.children.len(), 2);

        match &dom.children[0].node_type {
            NodeType::Text(ref text) => assert_eq!(text, "Hello World"),
            _ => panic!("Expected text node"),
        }

        match &dom.children[1].node_type {
            NodeType::Element(ref tag) => {
                assert_eq!(tag.name, "button");
                assert_eq!(tag.id(), Some(&"my_btn".to_string()));

                let classes: HashSet<&str> = ["btn", "p-6"].iter().cloned().collect();
                assert_eq!(tag.classes(), classes);
            }
            _ => panic!("Expected button element"),
        }

        assert_eq!(dom.children[1].children.len(), 1);
        match &dom.children[1].children[0].node_type {
            NodeType::Text(ref text) => assert_eq!(text, "Button"),
            _ => panic!("Expected text node"),
        }
    } //}}}
}
