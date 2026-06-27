# AGENTS.md

Guidelines for AI agents working in this repository.

## Commenting

Don't comment for the sake of commenting. Most code should describe itself.

**Don't add comments that restate the code.** This includes:
- Section headers like `// Drop column` above a block that checks `"dropcolumn"`.
- Trailing notes like `// clone the data` after a `.clone()`.
- Doc comments that just rephrase the function or field name (`/// Builds a column definition` on `define_column`).
- `# Arguments` / `# Parameters` / `# Returns` sections that list each item with an obvious restatement.

**Add a comment only when the code doesn't say it itself:**
- The *why*, not the *what* — a non-obvious business rule, a gotcha, a workaround.
- A constraint the reader can't see (e.g. "unwrap is safe because the key was validated above").
- Cross-references between modules or to an external spec.

**Keep:**
- License headers (the GPL block at the top of every file).
- `//!` module docs — keep them short; one or two lines is usually enough.
- `// TODO` markers.
- Doc comments that carry information the signature doesn't (e.g. an in-place mutation contract, a JSON-shape example).

**Style when you do comment:**
- Be direct. Shorter is better, as long as it's clear.
- No filler ("This function is used to...", "Note that..."). Start with the point.
- No AI-tells: no "leverage", "orchestration", "seamless", no em-dashes where a period works, no restating the task you were given.

## Scope

Do the task you were given, nothing more.

- Don't fix unrelated bugs, refactor unrelated code, or clean up style in code you weren't asked to touch. If you spot an unrelated problem, mention it — don't change it.
- Only change code the task requires. Leave everything else alone, even if it looks wrong.
- Exception: if the requested change breaks something else (e.g. you change a function signature), you may update the callers and anything else needed to keep the build working. That's part of the task, not unrelated.

## Build / verify

Before finishing a change, run from `src/alphadb`:

```
cargo build --features mysql
```

(Add `postgres`/`version-source` as needed.) Fix anything you broke. If `cargo clippy` is part of the workflow, run it too; don't introduce new warnings.

## General

- Match existing style in the file you're editing.
- Don't commit unless asked.
- Never run git commands (commit, push, branch, merge, rebase, tag, add, reset, etc.). Leave all version-control actions to the user.
- Don't add new dependencies without checking what's already used.
