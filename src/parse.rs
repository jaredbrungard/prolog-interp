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

// grammar:
//
// term             -> variable | atom ( termlist ) | atom
// termlist         -> term | term , termlist
// clause           -> fact | rule
// fact             -> term .
// rule             -> term :- termlist .
