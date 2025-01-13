
pub struct Parser {
    tokenizer: Tokenizer,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(expr: &'a str) -> Result<Self, Parser> {
        let mut lexer = Tokenizer::new(expr);
        let cur_token = match lexer.next() {
            Some(token) => token,
            None => return Err(ParserError::InvalidOperator("Invalid character in expression".into())),
        };
        Ok(Parser {
            tokenizer: lexer,
            current_token: cur_token,
        })
    }

    pub fn parse(&mut self) -> Result<Node, ParserError> {
        let ast = self.generate_ast(OperPrec::DefaultZero);
        match ast {
            Ok(ast) => Ok(ast),
            Err(e) => Err(e),
        }
    }

    fn get_next_token(&mut self) -> Result<(), ParserError> {
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParserError::InvalidOperator("Invalid character".into())),
        };
        self.current_token = next_token;
        Ok(());
    }

    fn check_paren(&mut self, expected: Token) -> Result<(), ParserError> {
        if self.current_token != expected {
            return Err(ParserError::InvalidOperator(format!("Expected {:?} but got {:?}", expected, self.current_token)));
        }
        self.get_next_token()?;
        Ok(())
    }

    fn parse_number(&mut self) -> Result<Node, ParserError> {
        let token = self.current_token.clone();
        match token {
            Token::Subtract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::Num(i) => {
                self.get_next_token()?;
                Ok(Node::Number(i))
            }
            Token::LeftParen => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                self.check_paren(Token::RightParen)?;
                if self.current_token == Token::LeftParen {
                    let right = self.generate_ast(OperPrec::MulDiv)?;
                    return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
                }

                Ok(expr)
            }
            _ => Err(ParseError::UnableToParse("Unable to parse".to_string())),
        }
    }

    fn generate_ast(&mut self, precedence: OperPrec) -> Result<Node, ParserError> {
        let mut left = self.parse_number()?;
        while let Some(op) = self.current_token.clone().to_operator() {
            if op.precedence() < precedence {
                return Ok(left);
            }
            self.get_next_token()?;
            let mut right = self.parse_number()?;
            while let Some(next_op) = self.current_token.clone().to_operator() {
                if next_op.precedence() > op.precedence() {
                    right = self.generate_ast(next_op.precedence())?;
                } else {
                    break;
                }
            }
            left = op.create_node(left, right);
        }
        Ok(left)
    }
}