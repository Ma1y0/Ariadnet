use std::fmt::Display;

use super::dom::Node;

pub struct Parser {
    ch: u8,
    next_ch: u8,
    buffer: Vec<u8>,
    index: usize,
}

impl Parser {
    pub fn new(s: String) -> Self {
        let buffer = s.trim().as_bytes();
        Self {
            ch: buffer[0],
            next_ch: buffer[1],
            buffer: buffer.to_vec(),
            index: 0,
        }
    }

    pub fn parse(&mut self) -> ParserResult {
        Err(ParseError::ParseError())
    }

    fn parse_node() -> Node {
        unimplemented!()
    }
}

#[derive(Debug)]
enum ParseError {
    ParseError(),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError() => writeln!(f, "Failed to parse"),
        }
    }
}

impl std::error::Error for ParseError {}

type ParserResult = Result<Node, ParseError>;

#[cfg(test)]
mod tests {
    use std::collections::HashMap; //{{{

    use super::*;

    #[test]
    fn parse_tag() {
        let s_tag = "<p>Hello World</p>";
        let text_node = Node::new_text("Hello World".to_string());
        let tag_node = Node::new_element("p".to_string(), HashMap::new(), vec![text_node]);

        let mut parser = Parser::new(s_tag.to_string());
        let dom = parser.parse();

        match dom {
            Ok(a) => assert_eq!(tag_node, a),
            Err(e) => panic!("{e}"),
        }
    } //}}}
}
