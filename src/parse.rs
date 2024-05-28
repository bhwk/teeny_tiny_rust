use crate::emit::Emitter;
use crate::lex::{Lexer, Token, TokenType};
use std::collections::HashSet;
use std::fmt::format;

pub struct Parser {
    lexer: Lexer,
    pub emitter: Emitter,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    symbols: HashSet<String>,
    labels_declared: HashSet<String>,
    labels_gotoed: HashSet<String>,
}

impl Parser {
    pub fn new(lexer: Lexer, emitter: Emitter) -> Parser {
        let mut parser = Parser {
            lexer,
            emitter,
            current_token: None,
            peek_token: None,
            symbols: HashSet::new(),
            labels_declared: HashSet::new(),
            labels_gotoed: HashSet::new(),
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

    // program::={statement}
    pub fn program(&mut self) {
        self.emitter.header_line("#include <stdio.h>".into());
        self.emitter.header_line("int main(void){".into());

        while self.check_token(TokenType::NEWLINE) {
            self.next_token();
        }

        while !self.check_token(TokenType::EOF) {
            self.statement();
        }

        //return on program end
        self.emitter.emit_line("return 0;".into());
        self.emitter.emit_line("}".into());

        for label in &self.labels_gotoed {
            if !self.labels_declared.contains(label) {
                self.abort_operation(format!("Attempting to GOTO to undeclared label: {}", label))
            }
        }
    }

    fn statement(&mut self) {
        //check first token

        if self.check_token(TokenType::PRINT) {
            // PRINT (expression | string)
            self.next_token();

            if self.check_token(TokenType::STRING) {
                // string, print string
                self.emitter.emit_line(format!(
                    "printf(\"{}\\n\");",
                    self.current_token.as_ref().unwrap().text,
                ));

                self.next_token();
            } else {
                self.emitter.emit(format!("printf(\"%.2f\\n\", (float)("));
                self.expression();
                self.emitter.emit_line("));".into());
            }
        } else if self.check_token(TokenType::IF) {
            // IF comparison THEN statement ENDIF
            self.next_token();
            self.emitter.emit("if(".into());
            self.comparison();

            self.match_token(TokenType::THEN);
            self.nl();
            self.emitter.emit_line(") {".into());

            while !self.check_token(TokenType::ENDIF) {
                self.statement();
            }
            self.match_token(TokenType::ENDIF);
            self.emitter.emit_line("}".into());
        } else if self.check_token(TokenType::WHILE) {
            // WHILE comparison REPEAT statement ENDWHILE
            self.next_token();
            self.emitter.emit("while(".into());
            self.comparison();

            self.match_token(TokenType::REPEAT);
            self.nl();
            self.emitter.emit_line("){".into());

            while !self.check_token(TokenType::ENDWHILE) {
                self.statement();
            }
            self.match_token(TokenType::ENDWHILE);
            self.emitter.emit_line("}".into());
        } else if self.check_token(TokenType::LABEL) {
            //LABEL ident
            self.next_token();

            //if insert returns false, it means that the value
            // already exists in the set. abort operation if so.
            // clone because insert consumes the value
            if !self
                .labels_declared
                .insert(self.current_token.clone().unwrap().text)
            {
                self.abort_operation(format!(
                    "Label already exists {}",
                    self.current_token.as_ref().unwrap().text
                ))
            }

            self.emitter
                .emit_line(format!("{}:", self.current_token.as_ref().unwrap().text));
            self.match_token(TokenType::IDENT);
        } else if self.check_token(TokenType::GOTO) {
            //GOTO ident
            self.next_token();
            self.labels_gotoed
                .insert(self.current_token.clone().unwrap().text);
            self.emitter.emit_line(format!(
                "goto {};",
                self.current_token.as_ref().unwrap().text
            ));
            self.match_token(TokenType::IDENT);
        } else if self.check_token(TokenType::LET) {
            // LET ident = expression
            self.next_token();

            if !self
                .symbols
                .contains(&self.current_token.as_ref().unwrap().text)
            {
                self.symbols
                    .insert(self.current_token.clone().unwrap().text);
                self.emitter.header_line(format!(
                    "float {};",
                    self.current_token.as_ref().unwrap().text
                ));
            }
            self.emitter
                .emit(format!("{} = ", self.current_token.as_ref().unwrap().text));
            self.match_token(TokenType::IDENT);
            self.match_token(TokenType::EQ);

            self.expression();
            self.emitter.emit_line(";".into());
        } else if self.check_token(TokenType::INPUT) {
            self.next_token();

            //if variable doesn't exist declare in symbols set
            if !self
                .symbols
                .contains(&self.current_token.as_ref().unwrap().text)
            {
                self.symbols
                    .insert(self.current_token.clone().unwrap().text);
                self.emitter.header_line(format!(
                    "float {};",
                    self.current_token.as_ref().unwrap().text
                ));
            }

            //emit scanf but also validate input.
            //if invalid set the variable to 0 and clear input
            self.emitter.emit_line(format!(
                "if(0 == scanf(\"%f\", &{})) {}",
                self.current_token.as_ref().unwrap().text,
                "{"
            ));
            self.emitter.emit_line(format!(
                "{} = 0;",
                self.current_token.as_ref().unwrap().text
            ));
            self.emitter.emit("scanf(\"%".into());
            self.emitter.emit_line("*s\");".into());
            self.emitter.emit_line("}".into());
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
        // require at least 1 newline
        self.match_token(TokenType::NEWLINE);
        while self.check_token(TokenType::NEWLINE) {
            self.next_token();
        }
    }

    fn expression(&mut self) {
        // expression ::= term OPERATOR term

        self.term();

        // can have 0 or more OPERATOR and expressions
        while self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
            self.emitter.emit(self.current_token.clone().unwrap().text);
            self.next_token();
            self.term();
        }
    }

    fn term(&mut self) {
        self.unary();
        // can have 0 or more expressions

        while self.check_token(TokenType::ASTERISK) || self.check_token(TokenType::SLASH) {
            self.emitter.emit(self.current_token.clone().unwrap().text);
            self.next_token();
            self.unary()
        }
    }

    fn unary(&mut self) {
        // unary::= + - primary
        //optional unary
        if self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
            self.emitter.emit(self.current_token.clone().unwrap().text);
            self.next_token();
        }
        self.primary();
    }

    fn primary(&mut self) {
        //primary ::= number |ident
        if self.check_token(TokenType::NUMBER) {
            self.emitter.emit(self.current_token.clone().unwrap().text);
            self.next_token();
        } else if self.check_token(TokenType::IDENT) {
            if !self
                .symbols
                .contains(&self.current_token.as_ref().unwrap().text)
            {
                self.abort_operation(format!(
                    "Referencing variable before assignment: {} ",
                    self.current_token.clone().unwrap().text
                ))
            }
            self.emitter.emit(self.current_token.clone().unwrap().text);
            self.next_token();
        } else {
            self.abort_operation(format!(
                "Unexpected token at {}",
                self.current_token.as_ref().unwrap().text
            ))
        }
    }

    fn comparison(&mut self) {
        //comparison ::= expression

        self.expression();

        if self.is_comparison_operator() {
            self.emitter.emit(self.current_token.clone().unwrap().text);
            self.next_token();
            self.expression();
        } else {
            self.abort_operation(format!(
                "Expected comparison operator at: {}",
                self.current_token.as_ref().unwrap().text
            ))
        }

        while self.is_comparison_operator() {
            self.emitter.emit(self.current_token.clone().unwrap().text);
            self.next_token();
            self.expression();
        }
    }

    fn is_comparison_operator(&mut self) -> bool {
        return self.check_token(TokenType::GT)
            || self.check_token(TokenType::GTEQ)
            || self.check_token(TokenType::LT)
            || self.check_token(TokenType::LTEQ)
            || self.check_token(TokenType::EQEQ)
            || self.check_token(TokenType::NOTEQ);
    }
}
