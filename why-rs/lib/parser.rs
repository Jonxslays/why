use super::Node;
use super::NodeType;
use super::Operator;
use super::Token;
use super::TokenType;

#[derive(Clone, Debug)]
pub struct Parser {
    pub root: Node,
    pub tokens: Vec<Token>,
    pub idx: usize,
    pub current_node: Node,
}

impl Parser {
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        let root = Node::new(NodeType::Entrypoint);

        Self {
            root: root.clone(),
            tokens,
            idx: 0,
            current_node: root,
        }
    }

    pub fn can_advance(&self) -> bool {
        self.idx < self.tokens.len()
    }

    pub fn peek_token(&self) -> &Token {
        &self.tokens[self.idx + 1]
    }

    pub fn current_token(&self) -> &Token {
        &self.tokens[self.idx]
    }

    pub fn next(parser: &mut Parser) {
        parser.idx += 1;
    }

    pub fn convert_tok_to_op(token: &Token) -> Operator {
        match token.typ {
            TokenType::Plus => Operator::Plus,
            TokenType::PlusPlus => Operator::PlusPlus,
            TokenType::PlusEq => Operator::PlusEq,
            TokenType::Minus => Operator::Minus,
            TokenType::MinusMinus => Operator::MinusMinus,
            TokenType::MinusEq => Operator::MinusEq,
            TokenType::Eq => Operator::Eq,
            TokenType::EqEq => Operator::EqEq,
            TokenType::Slash => Operator::Slash,
            TokenType::SlashEq => Operator::SlashEq,
            TokenType::Star => Operator::Star,
            TokenType::StarEq => Operator::StarEq,
            TokenType::StarStar => Operator::StarStar,
            _ => {
                panic!("Syntax error: {}", token.loc);
            }
        }
    }

    pub fn generate_number_node(token: &Token) -> Node {
        match token.typ {
            TokenType::NumLiteral(true) => {
                // This is a float
                if let Ok(value) = token.value.parse::<f64>() {
                    Node::new(NodeType::FloatExpr(value))
                } else {
                    panic!("Type error: {}\nFailed to convert {} to float", token.loc, token.value);
                }
            }
            TokenType::NumLiteral(false) => {
                // This is an integer
                if let Ok(value) = token.value.parse::<usize>() {
                    Node::new(NodeType::IntExpr(value))
                } else {
                    panic!("Type error: {}\nFailed to convert {} to int", token.loc, token.value);
                }
            }
            _ => {
                panic!("Syntax error: {}", token.loc)
            }
        }
    }

    pub fn parse_binary_expr(parser: &mut Parser, left_token: Token, op_token: &Token) {
        Parser::next(parser);
        let right_token = parser.peek_token();

        // println!("Parsing binary");
        match right_token.typ {
            TokenType::NumLiteral(_) => {
                let op = Parser::convert_tok_to_op(&op_token);
                let mut binary_expr = Node::new_op(NodeType::BinaryExpr, op);

                let left_node = Parser::generate_number_node(&left_token);
                let right_node = Parser::generate_number_node(right_token);

                binary_expr.push_child("left", left_node);
                binary_expr.push_child("right", right_node);

                parser.root.push_child("root-child", binary_expr);
            }
            _ => (),
        }
    }

    pub fn parse(&mut self) {
        println!("");
        println!("Entering parse loop...");
        println!("");

        while self.can_advance() {
            println!("{}", self.current_token());
            let current = self.current_token().clone();

            match current.typ {
                TokenType::NumLiteral(_) => {
                    let next = self.peek_token().clone();

                    match next.typ {
                        TokenType::Plus
                        | TokenType::Minus
                        | TokenType::Star
                        | TokenType::Slash => Parser::parse_binary_expr(self, current, &next),
                        TokenType::Semi => Parser::next(self),
                        TokenType::Eof => {
                            eprintln!("\nInvalid syntax: {}\nExpected ';' after {:?}", next.loc, current.typ);
                            break;
                        }
                        _ => {
                            eprintln!("\nInvalid syntax: {}\nNo rules expect {} after {:?}", next.loc, next, current.typ);
                            break;
                        }
                    }
                }
                TokenType::Eof => break,
                _ => {
                    eprintln!("Syntax error: Unexpected token at {}", current.loc);
                    break;
                },
            }

            Parser::next(self);
        }

        println!("");

        for (_, node) in &self.root.children {
            println!("Parsed {:?}: {:?}", node.typ, node.op.clone().unwrap());

            for (name, child) in &node.children {
                println!("{}: {:?}", name, child);
            }
        }
    }
}
