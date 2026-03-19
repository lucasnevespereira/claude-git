pub const COMMIT_MSG: &str =
    "propose a conventional commit message for these changes. one line, max 72 chars. output only the message, nothing else.";

pub const REVIEW: &str =
    "review this diff. be concise. flag bugs, security issues, and logic errors. if it looks good, say so. skip style nitpicks.";

pub const EXPLAIN: &str =
    "explain what these changes do in 2-3 sentences. be specific about the behavior change, not just what files were touched.";

pub const PR: &str = "write a pull request description for these changes. format:

## What
one paragraph summary.

## Changes
- bullet points of key changes

output only the description, no title.";
