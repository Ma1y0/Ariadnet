use super::dom::{Node, Tag};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    iter::Peekable,
    str::Chars,
};

struct Parser<'a> {
    buffer: Peekable<Chars<'a>>,
    queue: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            buffer: s.chars().peekable(),
            queue: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();

        while self.buffer.peek().is_some() {
            self.consume_while(char::is_whitespace);

            if self.buffer.peek().map_or(false, |x| *x == '<') {
                // It's a tag node
                self.buffer.next();
                if self.buffer.peek().map_or(false, |x| *x == '/') {
                    // It's a closing tag
                    self.buffer.next();
                    self.consume_while(char::is_whitespace); // </      h1>

                    let tag_name = self.consume_while(Self::is_valid_tag_name_char);

                    self.consume_while(|x| x != '>'); // white space
                    self.buffer.next();

                    self.queue.push(tag_name);
                } else if self.buffer.peek().map_or(false, |x| *x == '!') {
                    // It's a comment
                    unimplemented!("Comment");
                } else {
                    // It's opening tag
                }
            } else {
                // It's a text node
                nodes.push(self.parse_text_node())
            }
        }

        nodes
    }

    fn parse_node(&mut self) -> Node {
        let name = self.consume_while(Self::is_valid_tag_name_char);
        let attr = self.parse_attributes();

        let children = self.parse();
        Node::new_element(name, attr, children)
    }

    fn parse_attributes(&mut self) -> HashMap<String, String> {
        let mut attr = HashMap::new();

        while self.buffer.peek().map_or(false, |x| *x != '>') {
            self.consume_while(char::is_whitespace);
            let attr_name = self
                .consume_while(Self::is_valid_attributes_name_char)
                .to_lowercase();
            self.consume_while(char::is_whitespace);

            let value = if self.buffer.peek().map_or(false, |x| *x == '=') {
                self.buffer.next();
                self.consume_while(char::is_whitespace);
                let val = self.parse_attr_value();
                self.consume_while(|x| !x.is_whitespace() && x != '>');
                self.consume_while(char::is_whitespace);

                val
            } else {
                "".to_string()
            };

            attr.insert(attr_name, value);
        }

        self.buffer.next();

        attr
    }

    fn parse_attr_value(&mut self) -> String {
        self.consume_while(char::is_whitespace);

        match self.buffer.peek() {
            Some(&a) if a == '"' || a == '\'' => {
                self.buffer.next();
                let ret = self.consume_while(|x| x != a);
                self.buffer.next();
                ret
            }
            _ => self.consume_while(Self::is_valid_attr_value),
        }
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

    fn is_valid_tag_name_char(ch: char) -> bool {
        ch.is_digit(36)
    }

    fn is_valid_attributes_name_char(ch: char) -> bool {
        !Self::is_excluded_name_char(ch) && !Self::is_control_char(ch)
    }

    fn is_control_char(ch: char) -> bool {
        match ch {
            '\u{007F}' => true,
            c if c >= '\u{0000}' && c <= '\u{001F}' => true,
            c if c >= '\u{0080}' && c <= '\u{009F}' => true,
            _ => false,
        }
    }

    fn is_excluded_name_char(ch: char) -> bool {
        match ch {
            ' ' | '"' | '\'' | '>' | '/' | '=' => true,
            _ => false,
        }
    }

    fn is_valid_attr_value(ch: char) -> bool {
        match ch {
            ' ' | '"' | '\'' | '=' | '<' | '>' | '`' => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    //{{{
    use crate::engine::dom::NodeType;

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
