pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        let mut lexer = Tokenizer::new(new_expr);
        let current_token =  match lexer.next_token() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };

        Ok(Parser {
            tokenizer: lexer,
            current_token: current_token,
        })
    }

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let ast = self.generate_ast(OperPrec::DefaultZero);

        match ast {
            Ok(ast) => Ok(ast),
            Err(err) => Err(err),
        }
    }

    fn generate_ast(&mut self, oper_prec: OperPrec) -> Result<Node, ParseError> {
        let mut left_expr = self.parse_number()?;

        while oper_prec < self.current_token.get_oper_prec() {
            if self.current_token == Token::EOF {
                break;
            }
            if right_expr = self.convert_token_to_node(left_expr.clone())?;

            left_expr = right_expr;
        }

        Ok(left_expr);
    }

    fn get_next_token(&mut self) -> Result<(), ParseError> {
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        }
    }

    // 检测括号对
    fn check_paren(&mut self, expected: Token) -> Result<(), ParseError> {
        if expected == self.current_token {
            self.get_next_token()?;
            Ok(())
        } else {
            Err(ParseError::InvalidOperator(format!("Expected {:?}, found {:?}", expected, self.current_token)))
        }
    }

    fn parse_number(&mut self) -> Result<Node, ParseError> {
        let token = self.current_token.clone();

        match token {
            Token::Num(num) => {
                self.get_next_token()?;
                Ok(Node::Num(num))
            },
            Token::Substract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            },
            Token::LeftParen => {
                self.get_next_token()?;
                let ast = self.generate_ast(OperPrec::DefaultZero)?;
                self.check_paren(Token::RightParen)?;
                if self.current_token == Token::LeftParen {
                    let right = self.generate_ast(OperPrec::MulDiv)?;
                    return Ok(Node::Multiply(Box::new(ast), Box::new(right)));
                }
                Ok(ast)
            },
            _ => Err(ParseError::UnableToParse("Unable to parse".to_string())),
        }
    }

    fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParseError> {
        let token = self.current_token.clone();

        match token {
            Token::Add => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSubstract)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            },
            Token::Substract => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSubstract)?;
                Ok(Node::Substract(Box::new(left_expr), Box::new(right_expr)))
            },
            Token::Multiply => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
            },
            Token::Divide => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
            },
            Token::Caret => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::Power)?;
                Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
            },
            _ => Err(ParseError::InvalidOperator("Invalid operator".into())),
        }
    }
}
