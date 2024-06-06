use super::dom::Node;
use std::{iter::Peekable, str::Chars};

struct Parser<'a> {
    buffer: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            buffer: s.chars().peekable(),
        }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();

        while self.buffer.peek().is_some() {
            self.consume_while(char::is_whitespace);

            if self.buffer.peek().map_or(false, |x| *x == '<') {
                // Is tag
            } else {
                // Is text node
                nodes.push(self.parse_text_node())
            }
        }

        nodes
    }

    fn parse_text_node(&mut self) -> Node {
        let text = self.consume_while(|x| x != '<');

        Node::new_text(text)
    }

    fn consume_while<F>(&mut self, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut s = String::new();

        while self.buffer.peek().map_or(false, |x| condition(*x)) {
            s.push(self.buffer.next().unwrap());
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::dom::NodeType;

    //{{{
    use super::*;

    #[test]
    fn test_consume_while() {
        let s = "              Hello";
        let mut parser = Parser::new(s);
        let consumed = parser.consume_while(char::is_whitespace);

        assert_eq!(s.trim(), parser.buffer.collect::<String>());
        assert_eq!("              ".to_string(), consumed);
    }

    #[test]
    fn test_parse_text_node() {
        let s = "How are you today?";
        let mut parser = Parser::new(s);
        let dom = parser.parse();

        assert_eq!(1, dom.len(), "There should be only one node");
        assert_eq!(
            NodeType::Text(s.to_string()),
            dom[0].node_type,
            "The node should be a text node of {}, insed it is {:?}",
            s,
            dom[0].node_type
        );
    }
    //}}}
}
