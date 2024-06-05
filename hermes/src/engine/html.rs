use super::dom::Node;

struct Parser {
    ch: u8,
    next_ch: u8,
    buffer: Vec<u8>,
    index: usize,
}

impl Parser {
    pub fn new(s: String) -> Self {
        let buffer = s.as_bytes();
        Self {
            ch: buffer[0],
            next_ch: buffer[1],
            buffer: buffer.to_vec(),
            index: 0,
        }
    }

    pub fn parse(&mut self) -> Node {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn parse_tag() {
        let s_tag = "<p>Hello World</p>";
        let text_node = Node::new_text("Hello World".to_string());
        let tag_node = Node::new_element("p".to_string(), HashMap::new(), vec![text_node]);

        let mut parser = Parser::new(s_tag.to_string());
        let dom = parser.parse();

        assert_eq!(tag_node, dom);
    }
}
