Variable renaming
=================

Modify your solver to rename variables correctly. Start by writing a helper
function to rename all the variables in a term. I suggest:

    fn rename_vars(term: &Term, depth: usize) -> Term { ... }

Use `apply_subs` for inspiration, but `rename_vars` should be simpler. For every
variable that it finds in the term, it should rename it by appending a hash sign
and the depth to the end, For example:

    X at depth 1 => X#1
    Head at depth 5 => Head#5

So it returns a clone of the term but with all variables renamed in this way.

Next, update solve to take a `depth` parameter:

    pub fn solve(program: &[Clause], goals: &[Term], query: &Term, depth: usize) { ... }

Every time it considers a clause in the program, it should rename all of its
variables before calling `mgu` or `resolution`. When `solve` calls itself
recursively, it should increment the depth.

No other changes should be needed.
