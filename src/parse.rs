use crate::types::{Clause, Term, Token};

pub fn parse_clause(tokens: &[Token]) -> Result<Clause, String> {
    let mut parser = Parser::new(tokens);
    let clause = parser.parse_clause()?;
    if parser.current_token().is_some() {
        return Err("Expected to find end of input".to_string());
    }
    Ok(clause)
}

pub fn parse_query(tokens: &[Token]) -> Result<Term, String> {
    let mut parser = Parser::new(tokens);
    let term = parser.parse_term()?;
    parser.expect_token(&Token::Period)?;
    if parser.current_token().is_some() {
        return Err("Expected to find end of input".to_string());
    }
    Ok(term)
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn expect_token(&mut self, expected: &Token) -> Result<(), String> {
        match self.current_token() {
            Some(token) if token == expected => {
                self.advance();
                Ok(())
            }
            Some(token) => {
                Err(format!("Expected {}, found {}", expected, token))
            }
            None => Err(format!("Expected {}, found end of input", expected)),
        }
    }

    // term -> variable | atom ( termlist ) | atom
    fn parse_term(&mut self) -> Result<Term, String> {
        match self.current_token() {
            Some(Token::Var(name)) => {
                let term = Term::Var(name.clone());
                self.advance();
                Ok(term)
            }
            Some(Token::Atom(name)) => {
                let name = name.clone();
                self.advance();
                // Check if it's a compound term: atom ( ... )
                if let Some(Token::LeftParen) = self.current_token() {
                    self.advance();
                    let args = self.parse_termlist()?;
                    self.expect_token(&Token::RightParen)?;
                    Ok(Term::Compound { head_atom: name, termlist: args })
                } else {
                    Ok(Term::Atom(name))
                }
            }
            Some(token) => Err(format!("Unexpected token in term: {}", token)),
            None => Err("Unexpected end of input in term".to_string()),
        }
    }

    // termlist -> term | term , termlist
    fn parse_termlist(&mut self) -> Result<Vec<Term>, String> {
        let mut terms = Vec::new();
        terms.push(self.parse_term()?);

        while let Some(Token::Comma) = self.current_token() {
            self.advance();
            terms.push(self.parse_term()?);
        }

        Ok(terms)
    }

    // clause -> fact | rule
    // fact -> term .
    // rule -> term :- termlist .
    fn parse_clause(&mut self) -> Result<Clause, String> {
        let head = self.parse_term()?;

        match self.current_token() {
            Some(Token::Period) => {
                self.advance();
                Ok(Clause::Fact(head))
            }
            Some(Token::ColonHyphen) => {
                self.advance();
                let body = self.parse_termlist()?;
                self.expect_token(&Token::Period)?;
                Ok(Clause::Rule(head, body))
            }
            Some(token) => {
                Err(format!("Expected . or :- after head, found {}", token))
            }
            None => Err("Unexpected end of input in clause".to_string()),
        }
    }
}
