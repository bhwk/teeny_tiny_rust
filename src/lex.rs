pub struct Lexer {
    pub source: Vec<char>,
    pub current_char: Option<char>,
    pub current_pos: isize,
}

#[derive(Debug)]
pub struct Token {
    pub text: String,
    pub kind: TokenType,
}

#[derive(Debug, PartialEq, Eq)]
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
                _ => self.abort_operation(format!("Unknown token:  {}", current_char)),
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
}
