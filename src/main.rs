mod parse;
mod solve;
mod tokenize;
mod types;

use crate::parse::{parse_clause, parse_query};
use crate::solve::solve;
use crate::tokenize::tokenize;
use crate::types::Token;
use std::io;

fn main() {
    // print a prompt
    println!("\nPlease enter clauses (blank line to end the database):");

    let mut program = Vec::new();
    loop {
        // get one clause
        let mut tokens = Vec::new();
        loop {
            // read a line of input, quit on ctrl-d and skip empty lines
            let mut input = String::new();
            let len =
                io::stdin().read_line(&mut input).expect("Failed to read line");
            if len == 0 {
                return;
            }
            if input.trim().is_empty() {
                break;
            }

            // tokenize
            match tokenize(input.trim()) {
                Ok(new_tokens) => {
                    tokens.extend(new_tokens);
                }
                Err(msg) => {
                    println!("Tokenizer error: {msg}");
                    return;
                }
            };

            // finish if the last token was a period
            if let Some(Token::Period) = tokens.last() {
                break;
            }
        }
        if tokens.is_empty() {
            break;
        }

        print!("tokens: [");
        let mut sep = "";
        for t in &tokens {
            print!("{sep}{t}");
            sep = " ";
        }
        println!("]");

        // parse
        let clause = match parse_clause(&tokens) {
            Ok(clause) => clause,
            Err(msg) => {
                println!("Parse error: {msg}");
                return;
            }
        };
        println!("clause: {clause}");
        program.push(clause);
    }

    loop {
        // get one query
        println!("Please enter a query (blank line to quit):");
        let mut tokens = Vec::new();
        loop {
            // read a line of input, quit on ctrl-d and skip empty lines
            let mut input = String::new();
            let len =
                io::stdin().read_line(&mut input).expect("Failed to read line");
            if len == 0 {
                return;
            }
            if input.trim().is_empty() {
                break;
            }

            // tokenize
            match tokenize(input.trim()) {
                Ok(new_tokens) => {
                    tokens.extend(new_tokens);
                }
                Err(msg) => {
                    println!("Tokenizer error: {msg}");
                    return;
                }
            };

            // finish if the last token was a period
            if let Some(Token::Period) = tokens.last() {
                break;
            }
        }
        if tokens.is_empty() {
            break;
        }

        print!("tokens: [");
        let mut sep = "";
        for t in &tokens {
            print!("{sep}{t}");
            sep = " ";
        }
        println!("]");

        // parse
        let query = match parse_query(&tokens) {
            Ok(term) => term,
            Err(msg) => {
                println!("Parse error: {msg}");
                return;
            }
        };
        println!("query: {query}.");

        // solve
        solve(&program, std::slice::from_ref(&query), &query);
        println!();
    }
}
