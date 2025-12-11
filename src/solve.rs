use crate::types::{Clause, Term};
use std::fmt;

struct Sub {
    input_variable: String,
    output_term: Term,
}

impl fmt::Display for Sub {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} -> {})", self.input_variable, self.output_term)
    }
}

fn apply_subs(subs: &[Sub], mut term: Term) -> Term {
    for Sub { input_variable, output_term } in subs {
        term = match term {
            Term::Var(name) if name == *input_variable => output_term.clone(),
            Term::Compound { head_atom: atom_name, termlist: input_list } => {
                let head_atom = atom_name.clone();
                let mut termlist = Vec::new();
                for elt in input_list {
                    termlist.push(apply_subs(subs, elt));
                }
                Term::Compound { head_atom, termlist }
            }
            _ => term.clone(),
        };
    }
    term
}

fn mgu(a: &Term, b: &Term) -> Result<Vec<Sub>, ()> {
    let mut unifier = Vec::new();
    let mut a_list = vec![a.clone()];
    let mut b_list = vec![b.clone()];
    while !a_list.is_empty() && !b_list.is_empty() {
        match (a_list.remove(0), b_list.remove(0)) {
            (Term::Var(var_a), Term::Var(var_b)) => {
                if var_a > var_b {
                    unifier.push(Sub {
                        input_variable: var_a,
                        output_term: Term::Var(var_b),
                    });
                } else {
                    unifier.push(Sub {
                        input_variable: var_b,
                        output_term: Term::Var(var_a),
                    });
                }
                for term in std::mem::take(&mut a_list) {
                    a_list.push(apply_subs(&unifier, term));
                }
                for term in std::mem::take(&mut b_list) {
                    b_list.push(apply_subs(&unifier, term));
                }
            }
            (output_term, Term::Var(input_variable)) => {
                unifier.push(Sub { input_variable, output_term });
                for term in std::mem::take(&mut a_list) {
                    a_list.push(apply_subs(&unifier, term));
                }
                for term in std::mem::take(&mut b_list) {
                    b_list.push(apply_subs(&unifier, term));
                }
            }
            (Term::Var(input_variable), output_term) => {
                unifier.push(Sub { input_variable, output_term });
                for term in std::mem::take(&mut a_list) {
                    a_list.push(apply_subs(&unifier, term));
                }
                for term in std::mem::take(&mut b_list) {
                    b_list.push(apply_subs(&unifier, term));
                }
            }
            (Term::Atom(a), Term::Atom(b)) => {
                if a != b {
                    return Err(());
                }
            }
            (
                Term::Compound { head_atom: head_a, termlist: lst_a },
                Term::Compound { head_atom: head_b, termlist: lst_b },
            ) => {
                if head_a != head_b || lst_a.len() != lst_b.len() {
                    return Err(());
                }
                a_list.extend_from_slice(&lst_a);
                b_list.extend_from_slice(&lst_b);
            }
            _ => return Err(()),
        }
    }
    Ok(unifier)
}

fn succeed(term: &Term) {
    println!("SUCCESS: {term}");
}

fn resolution(
    subs: &[Sub],
    clause: &Clause,
    goals: &[Term],
    query: &Term,
) -> (Vec<Term>, Term) {
    let clause_body = match clause {
        Clause::Fact(_) => &[],
        Clause::Rule(_, body) => body.as_slice(),
    };

    let remaining_goals = &goals[1..];

    let mut new_goals = Vec::new();
    for term in clause_body {
        new_goals.push(apply_subs(subs, term.clone()));
    }
    for term in remaining_goals {
        new_goals.push(apply_subs(subs, term.clone()));
    }

    let new_query = apply_subs(subs, query.clone());

    (new_goals, new_query)
}

pub fn solve(program: &[Clause], goals: &[Term], query: &Term) {
    if goals.is_empty() {
        succeed(query);
        return;
    }

    let head_goal = &goals[0];

    for clause in program {
        let clause_head = match clause {
            Clause::Fact(head) => head,
            Clause::Rule(head, _) => head,
        };

        if let Ok(subs) = mgu(clause_head, head_goal) {
            let (new_goals, new_query) =
                resolution(&subs, clause, goals, query);

            solve(program, &new_goals, &new_query);
        }
    }
}
