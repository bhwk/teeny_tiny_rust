use crate::lex::{Lexer, Token, TokenType};

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            current_token: None,
            peek_token: None,
        };

        //initialize current and peek token
        parser.next_token();
        parser.next_token();

        return parser;
    }

    fn check_token(&mut self, kind: TokenType) -> bool {
        return kind == self.current_token.clone().unwrap().kind;
    }

    fn check_peek(&mut self, kind: TokenType) -> bool {
        return kind == self.peek_token.clone().unwrap().kind;
    }

    fn match_token(&mut self, kind: TokenType) {
        //matches current token, then advances
        if !self.check_token(kind.clone()) {
            self.abort_operation(format!(
                "Expected {:?}, got {:?}",
                kind,
                self.current_token.as_ref().unwrap().kind
            ))
        }
        self.next_token();
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.get_token();
    }

    fn abort_operation(&self, message: String) {
        panic!("Error. {message}")
    }

    pub fn program(&mut self) {
        println!("PROGRAM");

        while self.check_token(TokenType::NEWLINE) {
            self.next_token();
        }

        while !self.check_token(TokenType::EOF) {
            self.statement();
        }
    }

    fn statement(&mut self) {
        //check first token

        if self.check_token(TokenType::PRINT) {
            // PRINT (expression | string)
            println!("STATEMENT-PRINT");
            self.next_token();

            if self.check_token(TokenType::STRING) {
                // string
                self.next_token();
            } else {
                self.expression()
            }
        } else if self.check_token(TokenType::IF) {
            // IF comparison THEN statement ENDIF
            println!("STATEMENT-IF");
            self.next_token();
            self.comparison();

            self.match_token(TokenType::THEN);
            self.nl();

            while !self.check_token(TokenType::ENDIF) {
                self.statement();
            }
            self.match_token(TokenType::ENDIF);
        } else if self.check_token(TokenType::WHILE) {
            // WHILE comparison REPEAT statement ENDWHILE
            println!("STATEMENT-WHILE");
            self.next_token();
            self.comparison();

            self.match_token(TokenType::REPEAT);
            self.nl();

            while !self.check_token(TokenType::ENDWHILE) {
                self.statement();
            }
            self.match_token(TokenType::ENDWHILE);
        } else if self.check_token(TokenType::LABEL) {
            //LABEL ident
            println!("STATEMENT-LABEL");
            self.next_token();
            self.match_token(TokenType::IDENT);
        } else if self.check_token(TokenType::GOTO) {
            //GOTO ident
            println!("STATEMENT-GOTO");
            self.next_token();
            self.match_token(TokenType::IDENT);
        } else if self.check_token(TokenType::LET) {
            // LET ident = expression
            println!("STATEMENT-LET");
            self.next_token();
            self.match_token(TokenType::IDENT);
            self.match_token(TokenType::EQ);

            self.expression();
        } else if self.check_token(TokenType::INPUT) {
            println!("STATEMENT-INPUT");
            self.next_token();
            self.match_token(TokenType::IDENT);
        } else {
            self.abort_operation(format!(
                "Invalid statement at {} {:?}",
                self.current_token.as_ref().unwrap().text,
                self.current_token.as_ref().unwrap().kind
            ))
        }
        self.nl();
    }
    fn nl(&mut self) {
        println!("NEWLINE");

        // require at least 1 newline
        self.match_token(TokenType::NEWLINE);
        while self.check_token(TokenType::NEWLINE) {
            self.next_token();
        }
    }

    fn expression(&mut self) {
        todo!()
    }

    fn comparison(&mut self) {
        todo!()
    }
}
