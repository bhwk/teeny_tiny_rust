use std::str::FromStr;

use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

pub struct Lexer {
    pub source: Vec<char>,
    pub current_char: Option<char>,
    pub current_pos: isize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub kind: TokenType,
}

#[derive(Debug, PartialEq, Eq, EnumIter, EnumString, Clone)]
pub enum TokenType {
    EOF,
    NEWLINE,
    NUMBER,
    IDENT,
    STRING,
    LABEL,
    GOTO,
    PRINT,
    INPUT,
    LET,
    IF,
    THEN,
    ENDIF,
    WHILE,
    REPEAT,
    ENDWHILE,
    //operators
    EQ,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    EQEQ,
    NOTEQ,
    LT,
    LTEQ,
    GT,
    GTEQ,
}

impl Token {
    fn new(text: String, kind: TokenType) -> Token {
        Token { text, kind }
    }
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        //turn source string into Vec<char> for easier indexing

        let mut source: Vec<char> = source.chars().collect();
        source.push('\n');

        //create lexer struct
        let mut lexer = Lexer {
            source,
            current_char: None,
            current_pos: -1,
        };

        lexer.next_char();

        lexer
    }

    pub fn next_char(&mut self) {
        self.current_pos += 1;

        if self.current_pos as usize >= self.source.len() {
            //EOF
            self.current_char = Some('\0');
        } else {
            self.current_char = Some(self.source[self.current_pos as usize]);
        }
    }

    pub fn peek(&mut self) -> char {
        let next_pos = self.current_pos + 1;
        if next_pos as usize >= self.source.len() {
            return '\0';
        }

        return self.source[next_pos as usize];
    }

    pub fn get_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        self.skip_comments();

        let mut token: Option<Token> = None;

        if let Some(current_char) = self.current_char {
            match current_char {
                //operator pattern matching
                '+' => token = Some(Token::new(current_char.into(), TokenType::PLUS)),
                '-' => token = Some(Token::new(current_char.into(), TokenType::MINUS)),
                '*' => token = Some(Token::new(current_char.into(), TokenType::ASTERISK)),
                '/' => token = Some(Token::new(current_char.into(), TokenType::SLASH)),
                '=' => {
                    if self.peek() == '=' {
                        let mut text = current_char.to_string();
                        self.next_char();
                        text.push(self.current_char.unwrap());

                        token = Some(Token::new(text, TokenType::EQEQ));
                    } else {
                        token = Some(Token::new(current_char.into(), TokenType::EQ));
                    }
                }
                '>' => {
                    if self.peek() == '=' {
                        let mut text = current_char.to_string();
                        self.next_char();
                        text.push(self.current_char.unwrap());

                        token = Some(Token::new(text, TokenType::GTEQ));
                    } else {
                        token = Some(Token::new(current_char.into(), TokenType::GT));
                    }
                }
                '<' => {
                    if self.peek() == '=' {
                        let mut text = current_char.to_string();
                        self.next_char();
                        text.push(self.current_char.unwrap());

                        token = Some(Token::new(text, TokenType::LTEQ));
                    } else {
                        token = Some(Token::new(current_char.into(), TokenType::LT));
                    }
                }
                '!' => {
                    if self.peek() == '=' {
                        let mut text = current_char.to_string();
                        self.next_char();
                        text.push(self.current_char.unwrap());

                        token = Some(Token::new(text, TokenType::NOTEQ));
                    } else {
                        let next_char = self.peek();
                        self.abort_operation(format!("Expected !=, got ! {}", next_char));
                    }
                }
                '\"' => {
                    // get characters
                    self.next_char();
                    let start_pos = self.current_pos;

                    while self.current_char != Some('\"') {
                        // Do not allow special characters
                        if self.current_char == Some('\r')
                            || self.current_char == Some('\t')
                            || self.current_char == Some('\n')
                            || self.current_char == Some('\\')
                            || self.current_char == Some('%')
                        {
                            self.abort_operation(format!(
                                "Illegal character in string: {}",
                                self.current_char.unwrap()
                            ))
                        }
                        self.next_char();
                    }

                    // slice Vec<char> from start to current
                    let text = self.source[start_pos as usize..self.current_pos as usize]
                        .iter()
                        .collect();

                    token = Some(Token::new(text, TokenType::STRING));
                }
                '\n' => token = Some(Token::new(current_char.into(), TokenType::NEWLINE)),
                '\0' => token = Some(Token::new(current_char.into(), TokenType::EOF)),
                _ => {
                    //handle number, identifier, other operators
                    //digit token
                    if current_char.is_digit(10) {
                        let start_pos = self.current_pos;

                        while self.peek().is_digit(10) {
                            self.next_char();
                        }

                        //Decimal value
                        if self.peek() == '.' {
                            self.next_char();

                            if !self.peek().is_digit(10) {
                                self.abort_operation("Illegal character in number".into());
                            }

                            while self.peek().is_digit(10) {
                                self.next_char();
                            }
                        }

                        let text: String = self.source
                            [start_pos as usize..=self.current_pos as usize]
                            .iter()
                            .collect();

                        token = Some(Token::new(text, TokenType::NUMBER));
                    } else if current_char.is_alphabetic() {
                        //identifier token
                        //leading character is alphabetic, means identifier
                        let start_pos = self.current_pos;
                        while self.peek().is_alphanumeric() {
                            self.next_char();
                        }

                        let text: String = self.source
                            [start_pos as usize..=self.current_pos as usize]
                            .iter()
                            .collect();

                        //matching of keyword
                        if let Some(keyword) = self.check_if_keyword(text.clone()) {
                            token = Some(Token::new(text.into(), keyword));
                        } else {
                            token = Some(Token::new(text.into(), TokenType::IDENT));
                        }

                        //check if token text exists in token types
                    } else {
                        self.abort_operation(format!("Unknown token:  {}", current_char))
                    }
                }
            }
        };

        self.next_char();

        return token;
    }

    fn abort_operation(&self, message: String) {
        panic!("Lexing error. {}", message);
    }

    fn skip_whitespace(&mut self) {
        while self.current_char == Some(' ')
            || self.current_char == Some('\t')
            || self.current_char == Some('\r')
        {
            self.next_char();
        }
    }

    fn skip_comments(&mut self) {
        if self.current_char == Some('#') {
            while self.current_char != Some('\n') {
                self.next_char();
            }
        }
    }

    fn check_if_keyword(&mut self, text: String) -> Option<TokenType> {
        match TokenType::from_str(text.as_str()) {
            Ok(value) => {
                for kind in TokenType::iter() {
                    if kind == value {
                        return Some(kind);
                    }
                }
                //return None if no values found
                //i don't know why this is a thing
                None
            }
            //parsing error, return None
            Err(_) => return None,
        }
    }
}
