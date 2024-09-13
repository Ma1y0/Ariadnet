use super::{token::Token, HTMLError};
use std::{
    iter::Peekable,
    ops::{Deref, DerefMut},
    vec::IntoIter,
};

#[derive(Debug, PartialEq)]
pub enum TagType {
    Header1,
    Image,
    Division,
}

impl TryFrom<&str> for TagType {
    type Error = HTMLError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "h1" => Ok(Self::Header1),
            "img" => Ok(Self::Image),
            "div" => Ok(Self::Division),
            _ => Err(HTMLError::InvalidTag),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Tag {
    tag_type: TagType,
    body: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Tag(Tag),
    Literal(String),
}

// Maybe use Iter instead ?
struct Parser(Peekable<IntoIter<Token>>);

impl Parser {
    pub fn new(buf: Vec<Token>) -> Parser {
        Parser(buf.into_iter().peekable())
    }

    pub fn parse(&mut self) -> Result<Vec<Node>, HTMLError> {
        let mut nodes: Vec<Node> = Vec::new();

        while let Some(token) = self.next() {
            nodes.push(self.parse_node(token)?);
        }

        Ok(nodes)
    }

    fn parse_node(&mut self, token: Token) -> Result<Node, HTMLError> {
        let node = match token {
            Token::Literal(a) => Node::Literal(a),
            Token::SelfClosing(a) => Self::parse_self_closing(&a)?,
            Token::OpeningTag(a) => self.parse_tag(&a)?,
            Token::ClosingTag(_) => unreachable!(":("),
        };

        Ok(node)
    }

    fn parse_self_closing(tag_type: &str) -> Result<Node, HTMLError> {
        let tag_type = TagType::try_from(tag_type)?;
        Ok(Node::Tag(Tag {
            tag_type,
            body: Vec::new(),
        }))
    }

    fn parse_tag(&mut self, tag_type: &str) -> Result<Node, HTMLError> {
        let mut body = Vec::new();

        while let Some(token) = self.next() {
            match token {
                Token::ClosingTag(closing_tag) if closing_tag == tag_type => {
                    return Ok(Node::Tag(Tag {
                        tag_type: TagType::try_from(tag_type)?,
                        body,
                    }));
                }
                _ => body.push(self.parse_node(token)?),
            }
        }

        Err(HTMLError::UnexpectedEndOfInput)
    }
}

// Allow direct access to the Peekable
impl Deref for Parser {
    type Target = Peekable<IntoIter<Token>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Parser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use std::io::Lines;

    use super::*;
    use crate::engine::html::token::Tokenizer;

    fn parse(s: &str) -> Vec<Node> {
        let mut tokenizer = Tokenizer::new(s);
        let tokens = tokenizer.parse().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse().unwrap()
    }

    #[test]
    fn test_parse_literal() {
        let nodes = parse("Hello World");
        let expected = vec![Node::Literal("Hello World".to_string())];

        // Tests
        assert_eq!(expected, nodes);
    }

    #[test]
    fn test_parser_self_closing() {
        let nodes = parse("<img />");
        let expected = vec![Node::Tag(Tag {
            tag_type: TagType::Image,
            body: Vec::new(),
        })];

        // Tests
        assert_eq!(expected, nodes);
    }

    #[test]
    fn test_parse_series_of_literals_and_self_closing_tags() {
        let nodes = parse("Hello<img /> Bye<img /> How are you?<img />");
        let expected = vec![
            Node::Literal("Hello".to_string()),
            Node::Tag(Tag {
                tag_type: TagType::Image,
                body: Vec::new(),
            }),
            Node::Literal("Bye".to_string()),
            Node::Tag(Tag {
                tag_type: TagType::Image,
                body: Vec::new(),
            }),
            Node::Literal("How are you?".to_string()),
            Node::Tag(Tag {
                tag_type: TagType::Image,
                body: Vec::new(),
            }),
        ];

        // Tests
        assert_eq!(expected, nodes);
    }

    #[test]
    fn test_tag_with_body() {
        let nodes = parse("<h1>Hello World</h1>");
        let expected = vec![Node::Tag(Tag {
            tag_type: TagType::Header1,
            body: vec![Node::Literal("Hello World".to_string())],
        })];

        // Tests
        assert_eq!(expected, nodes);
    }

    #[test]
    fn test_nested_tag() {
        let nodes = parse("<h1><img /></h1>");
        let expected = vec![Node::Tag(Tag {
            tag_type: TagType::Header1,
            body: vec![Node::Tag(Tag {
                tag_type: TagType::Image,
                body: Vec::new(),
            })],
        })];

        // Tests
        assert_eq!(expected, nodes);
    }

    #[test]
    fn test_nested_tags() {
        let nodes = parse("<div><h1>Hello World</h1></div>");
        let expected = vec![Node::Tag(Tag {
            tag_type: TagType::Division,
            body: vec![Node::Tag(Tag {
                tag_type: TagType::Header1,
                body: vec![Node::Literal("Hello World".to_string())],
            })],
        })];

        // Tests
        assert_eq!(expected, nodes);
    }

    #[test]
    fn test_parse_multiline() {
        let nodes = parse(
            r#"
            <div>
                <div>
                    <h1>Hello World</h1>
                </div>
                <div>
                    <img />
                </div>
            </div>
            "#
            .lines()
            .map(|x| x.trim())
            // .filter(|ch| !ch.is_whitespace())
            .collect::<String>()
            .as_str(),
        );
        let expected = vec![Node::Tag(Tag {
            tag_type: TagType::Division,
            body: vec![
                Node::Tag(Tag {
                    tag_type: TagType::Division,
                    body: vec![Node::Tag(Tag {
                        tag_type: TagType::Header1,
                        body: vec![Node::Literal("Hello World".to_string())],
                    })],
                }),
                Node::Tag(Tag {
                    tag_type: TagType::Division,
                    body: vec![Node::Tag(Tag {
                        tag_type: TagType::Image,
                        body: Vec::new(),
                    })],
                }),
            ],
        })];

        // Tests
        assert_eq!(expected, nodes);
    }
}
