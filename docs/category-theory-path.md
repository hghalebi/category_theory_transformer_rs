# Category Theory Path

Use this path if you are curious about category theory but want executable
examples first.

## Order

1. Read [book/src/00-map.md](../book/src/00-map.md).
2. Read [book/src/02-morphisms-composition.md](../book/src/02-morphisms-composition.md).
3. Run `cargo run --example 02_morphism_composition`.
4. Read [book/src/05-structure-and-calculus.md](../book/src/05-structure-and-calculus.md).
5. Run `cargo run --example 05_seven_sketches`.

## Core Questions

- What is an object in this tiny system?
- What is a morphism?
- Why does composition require matching types?
- Why is a training step an endomorphism?
- What law does each example check?

## Done Criteria

You can explain category-theory vocabulary without leaving the Rust code:

```text
object -> morphism -> composition -> law
```
