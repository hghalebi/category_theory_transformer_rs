# 06 - Seven Sketches Through Rust

## Mental Model

Applied category theory is useful when it helps you model real structure:
orders, resources, database references, feasibility relations, signal flow,
open systems, and local-to-global checks.

This lesson uses one method:

```text
engineering problem
  -> named Rust values
  -> validated construction
  -> relation or composition
  -> law or boundary test
```

## Read This File

Open `src/sketches.rs`.

Read the tests first. They show the contract each small model is supposed to
protect.

Focus on these examples:

- `InformationLevel::can_flow_to`
- `ResourceBundle::tensor`
- `CompanyInstance::department_for`
- `SignalMatrix::compose_after`
- `OpenCircuit::then`
- `SafetyCover::global_truth`

## Run the Example

```bash
cargo run --example 05_seven_sketches
```

Expected shape:

```text
orders obey preorder laws: true
feature/layer Galois law: true
resource tensor monotone: true
co-design offer feasible: true
global behavior truth: True
```

The exact debug formatting is less important than the contracts. Each output
line is a small executable handle for a larger applied-category idea.

## What to Notice

The negative tests are as important as the positive examples.

They show that the code rejects invalid structure:

- a missing database reference,
- mismatched signal-matrix dimensions,
- an open-circuit boundary mismatch.

That is the same discipline as the tiny ML pipeline. A type or constructor is
valuable when it prevents a wrong connection from becoming ordinary runtime
state.

## Checkpoint

Pick one sketch and answer:

```text
What invalid composition or relationship does this model prevent?
```

A strong answer names the Rust type, the software problem, and the
category-theory shape.
