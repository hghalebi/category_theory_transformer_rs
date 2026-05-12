# Educator Path

Use this path if you are reviewing the project as a technical educator,
workshop facilitator, study-group host, or curriculum designer.

The review question is:

```text
Does the learner always know what to run, read, explain, or try next?
```

This is a public learning-path guide for checking learner movement through the
material.

## Thirty-Minute Review

Review these public entry points in order:

1. [README.md](../README.md)
2. [START_HERE.md](../START_HERE.md)
3. [book/src/welcome.md](../book/src/welcome.md)
4. [book/src/00-map.md](../book/src/00-map.md)
5. [book/src/exercises.md](../book/src/exercises.md)

At each stop, ask whether the next action is explicit.

## Learner Action Audit

| Surface | What the learner should know next | Evidence to check |
| --- | --- | --- |
| `README.md` | what the project is and which first command to run | `cargo run --bin category_ml` and `cargo run --example 01_token_sequence` are visible early |
| `START_HERE.md` | which first chapter sequence to follow | Domain Objects, Morphism and Composition, Tiny ML Pipeline, Training, Exercises |
| `book/src/welcome.md` | what the first command proves | text becomes token sequence and training pairs |
| `book/src/00-map.md` | how the source files, objects, and arrows fit together | pipeline diagram plus module map |
| `book/src/exercises.md` | how to practice and check transfer | exercises name a command, expected evidence, and a boundary to inspect |

The audit is about learner movement, not polish. A section can be beautifully
written and still fail if the learner cannot tell what to do next.

## Five-Minute Learner Simulation

Run this sequence as if you were a first-session learner:

```bash
cargo run --example 01_token_sequence
```

Then answer:

```text
What object did the raw text become?
What training examples were created?
Which chapter should I read next?
Which exercise checks the same idea?
```

If any answer requires guessing, that is a useful clarity issue.

## Thirty-Minute Learner Simulation

Use the first three examples:

```bash
cargo run --example 01_token_sequence
cargo run --example 01_domain_objects
cargo run --example 02_morphism_composition
```

Then check whether the learner can explain:

```text
TokenId is not a raw usize.
TokenSequence is not an unvalidated Vec.
A morphism has a source object and target object.
Composition requires the first target to match the second source.
```

The expected assessment is not a formal proof. The expected assessment is a
clear explanation tied to one printed output line or one Rust type.

## Exercise Review

For each exercise, look for four things:

| Requirement | Question |
| --- | --- |
| command | Does the exercise say what to run? |
| concept | Does it name the idea being practiced? |
| evidence | Does it say what output, test, or code boundary proves progress? |
| transfer | Does it ask the learner to explain the idea in a new small case? |

Exercises are strongest when they produce evidence, not only agreement.

## Report Useful Friction

Open the [chapter clarity feedback form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml)
when the learner path loses a next action.

Use this shape:

```text
Perspective: technical educator
Surface reviewed:
First place without a clear next action:
Last action that was clear:
What the learner should be told to run, read, explain, or try next:
Smallest useful fix:
```

Good educator feedback names a heading, command, table row, exercise, or link.
Broad comments are less useful than one concrete blocked learning step.
