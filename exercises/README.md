# Exercises

This directory contains the compact practice ladder for the book.

Use the files in this order:

1. `beginner/README.md`
2. `intermediate/README.md`
3. `advanced/README.md`

After attempting an exercise, use `ANSWER_KEY.md` to compare reasoning. The
answer key is written as facilitator guidance, so it focuses on expected
conceptual shape rather than exact phrasing.

If an exercise is unclear, fill out the attempt-record template in the book's
Exercises chapter before opening feedback. A useful report names the exercise,
the command run, the first failure signal, the confusing line or concept, and
the answer-key mismatch.

Exercise 15 is the mixed transfer check. Use it after the chapter-local
exercises to practice diagnosing invariant, composition, endomorphism, shape,
and local-to-global boundaries without chapter hints.

The advanced track now includes a finite-difference gradient-checking exercise
that matches the tiny Transformer training tests in `src/attention.rs`.

Run the full validation gate before submitting exercise changes:

```bash
bash scripts/check.sh
```
