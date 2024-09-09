use super::HTMLError;
use std::{
    iter::Peekable,
    ops::{Deref, DerefMut},
    str::Chars,
};

/// Token
#[derive(Debug, PartialEq)]
pub enum Token {
    OpeningTag(Tag),
    ClosingTag(Tag),
    SelfClosing(Tag),
    Literal(String),
}

/// Tag
#[derive(Debug, PartialEq)]
pub struct Tag {
    tag_type: String,
}

pub struct Tokenizer<'a>(Peekable<Chars<'a>>);

impl<'a> Tokenizer<'a> {
    pub fn new(s: &'a str) -> Self {
        Tokenizer(s.trim().chars().peekable())
    }

    pub fn parse(&mut self) -> Result<Vec<Token>, HTMLError> {
        let mut tokens = Vec::new();

        while let Some(&c) = self.peek() {
            match c {
                '<' => {
                    self.next(); // Consume '<'
                    if self.peek() == Some(&'/') {
                        self.next(); // Consume '/'
                        tokens.push(self.parse_closing_tag()?);
                    } else {
                        tokens.push(self.parse_opening_tag()?);
                    }
                }
                _ => tokens.push(self.parse_literal()?),
            }
        }

        Ok(tokens)
    }

    fn parse_opening_tag(&mut self) -> Result<Token, HTMLError> {
        let tag_type = self.parse_tag_type()?;

        while let Some(&c) = self.peek() {
            match c {
                '>' => {
                    self.next(); // Consume '>'
                    return Ok(Token::OpeningTag(Tag { tag_type }));
                }
                '/' => {
                    self.next(); // Consume '/'
                    if self.next() == Some('>') {
                        return Ok(Token::SelfClosing(Tag { tag_type }));
                    } else {
                        return Err(HTMLError::InvalidSyntax);
                    }
                }
                _ => {
                    self.next();
                }
            }
        }

        Err(HTMLError::UnexpectedEndOfInput)
    }

    fn parse_closing_tag(&mut self) -> Result<Token, HTMLError> {
        let tag_type = self.parse_tag_type()?;

        if self.next() == Some('>') {
            Ok(Token::ClosingTag(Tag { tag_type }))
        } else {
            Err(HTMLError::InvalidSyntax)
        }
    }

    fn parse_tag_type(&mut self) -> Result<String, HTMLError> {
        let mut tag_type = String::new();

        while let Some(&c) = self.peek() {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                tag_type.push(self.next().unwrap());
            } else {
                break;
            }
        }

        if tag_type.is_empty() {
            Err(HTMLError::InvalidTagName)
        } else {
            Ok(tag_type)
        }
    }

    fn parse_literal(&mut self) -> Result<Token, HTMLError> {
        let mut literal = String::new();

        while let Some(&c) = self.peek() {
            if c != '<' {
                literal.push(self.next().unwrap());
            } else {
                break;
            }
        }

        Ok(Token::Literal(literal))
    }
}

// Allow direct access to the Peekable
impl<'a> Deref for Tokenizer<'a> {
    type Target = Peekable<Chars<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Tokenizer<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_string() {
        let s = "Hello World";
        let mut tokenizer = Tokenizer::new(s);
        let tokens = tokenizer.parse();
        let expected = Token::Literal(s.to_string());

        // Tests
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        assert_eq!(1, tokens.len());
        assert_eq!(expected, tokens[0]);
    }

    #[test]
    fn test_parse_tag() {
        let s = "<h1></h1>";
        let mut tokenizer = Tokenizer::new(s);
        let tokens = tokenizer.parse();
        let expected = vec![
            Token::OpeningTag(Tag {
                tag_type: "h1".to_string(),
            }),
            Token::ClosingTag(Tag {
                tag_type: "h1".to_string(),
            }),
        ];

        // Tests
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        // assert_eq!(2, tokens.len());
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_self_closing_tag() {
        let s = "<img />";
        let mut tokenizer = Tokenizer::new(s);
        let tokens = tokenizer.parse();
        let expected = vec![Token::SelfClosing(Tag {
            tag_type: "img".to_string(),
        })];

        // Tests
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        assert_eq!(1, tokens.len());
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_tag_with_children() {
        let s = "<h1>Hello World</h1>";
        let mut tokenizer = Tokenizer::new(s);
        let tokens = tokenizer.parse();
        let expected = vec![
            Token::OpeningTag(Tag {
                tag_type: "h1".to_string(),
            }),
            Token::Literal("Hello World".to_string()),
            Token::ClosingTag(Tag {
                tag_type: "h1".to_string(),
            }),
        ];

        // Tests
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        // assert_eq!(2, tokens.len());
        assert_eq!(expected, tokens);
    }
}
