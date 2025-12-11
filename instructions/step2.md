Solver
======

In this step you should write the Prolog interpreter/solver as described in the
slides:

    function resolution(clause, goals, query):
        let sub = the MGU of head(clause) and head(goals)
        return (sub(tail(clause) concatenated with tail(goals)), sub(query))

    function solve(goals, query)
        if goals is empty then succeed(query)
        else for each clause c in the program, in order
            if head(c) does not unify with head(goals) then do nothing
            else solve(resolution(c, goals, query))

We will change this a little and use the following Rust functions:

    fn resolution(subs: &[Sub], clause: &Clause, goals: &[Term], query: &Term) -> (Vec<Term>, Term) {}
    pub fn solve(program: &[Clause], goals: &[Term], query: &Term) {}

Note that only `solve` needs to be public as it is called from `main`.

`subs` is a list of substitutions to be applied in order, i.e., as new
substitutions are found they are added to the end of the list. This is built for
you by the `mgu` function, which is provided. You can use it by calling the
`apply_subs` function, which takes a list of substitutions and a `Term` and
applies the substitutions to the term.

Note that the structure is a little different than the slides: `solve` checks to
see if the head of the goal list and the head of clause can be unified, and if
so it passes the MGU (a substitution list) to `resolution`, rather than having
`resolution` re-create it as in the pseudocode.

So here are the paramaters and return values for `resolution`:

*   `subs`: a list of substitutions as provided by the `mgu` function and used
    be `apply_subs`
*   `clause`: a single clause from the program/database that unifies with the
    head of the goal list
*   `goals`: the list of goals that `resolution` must update
*   `query`: the query term used to collect substitions and present results back
    to the user

Return values:

*   The updated goal list
*   The query with any new substitutions applied

And here are the parameters to `solve`:

*   `program`: the complete list of clauses that make up the program database.
*   `goals`: the current list of goals that need to be solved.
*   `query`: the original query term, with substitutions applied along the way.

You can use `make run` to invoke the interpreter. It will start by having you
type in the program database clauses, then when you enter a blank line it will
switch to accepting queries and calling `solve` on them.
