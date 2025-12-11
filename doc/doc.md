Parser
======

Write a parser for a simple Prolog interpreter. You should base the structure of
your parser on the parsers from other assignments.

Here is the grammar you should implement:

    term             -> variable | atom ( termlist ) | atom
    termlist         -> term | term , termlist
    clause           -> fact | rule
    fact             -> term .
    rule             -> term :- termlist .

A few relevant details:

*   Implement the parser in `parse.rs`.

*   I have provided a complete tokenizer. See `tokenize.rs` for its definition.

*   The other types (Clause and Term) are defined in `types.rs`.

*   The public interface (used by `main`) is the functions `parse_clause` and
    `parse_query`. They each create a `Parser` object and expect to call a
    method on it. Use these to figure out what is required.

*   `inputs/tests.input` gives a bunch of examples of clauses that will need to
    parse. Make sure you understand how the grammar relates to the inputs before
    you begin.

You may also find the slides for Prolog to be helpful for review.
