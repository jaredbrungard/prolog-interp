use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {
    Var(String),
    Atom(String),
    LeftParen,
    RightParen,
    Period,
    Comma,
    ColonHyphen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Var(v) => write!(f, "{v}"),
            Token::Atom(a) => write!(f, "{a}"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::Period => write!(f, "."),
            Token::Comma => write!(f, ","),
            Token::ColonHyphen => write!(f, ":-"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Term {
    Var(String),
    Atom(String),
    Compound { head_atom: String, termlist: Vec<Term> },
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Compound { head_atom: cons, termlist }
                if cons == "cons" && termlist.len() == 2 =>
            {
                // pretty print lists
                write!(f, "[")?;
                let mut head = self;
                let mut sep = "";
                loop {
                    match head {
                        Term::Compound { head_atom: cons, termlist }
                            if cons == "cons" && termlist.len() == 2 =>
                        {
                            write!(f, "{sep}{}", termlist[0])?;
                            sep = ",";
                            head = &termlist[1];
                        }
                        Term::Atom(nil) if nil == "nil" => {
                            break;
                        }
                        _ => {
                            write!(f, "|{head}")?;
                            break;
                        }
                    }
                }
                write!(f, "]")
            }

            Term::Var(v) => write!(f, "{v}"),
            Term::Atom(a) => write!(f, "{a}"),
            Term::Compound { head_atom, termlist } => {
                write!(f, "{head_atom}(")?;
                let mut sep = "";
                for t in termlist {
                    write!(f, "{sep}{t}")?;
                    sep = ",";
                }
                write!(f, ")")
            }
        }
    }
}

pub enum Clause {
    Fact(Term),
    Rule(Term, Vec<Term>),
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Clause::Fact(t) => {
                write!(f, "{t}")?;
            }
            Clause::Rule(head, lst) => {
                write!(f, "{head} :- ")?;
                let mut sep = "";
                for elt in lst {
                    write!(f, "{sep}{elt}")?;
                    sep = ", ";
                }
            }
        }
        writeln!(f, ".")
    }
}
