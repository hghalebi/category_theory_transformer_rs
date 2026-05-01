# Seven Sketches Through Rust

The problem this chapter solves is:

> Applied category theory can feel too large to connect to code. This chapter
> turns the seven major themes of *Seven Sketches in Compositionality* into
> small Rust blocks with tests.

This chapter does not reproduce the paper.

It gives executable handles for the ideas.

The repeated pattern is:

```text
mathematical structure
  -> Rust type
  -> constructor or method
  -> law check
```

The Rust lesson is:

```text
newtypes + private fields + validation + explicit composition
```

The category-theory lesson is:

```text
objects + relationships + composition + laws
```

## Source Snapshots

The main module:

<details>
<summary>Source snapshot: src/sketches.rs</summary>

```rust,ignore
{{#include ../../src/sketches.rs}}
```

</details>

The runnable companion:

<details>
<summary>Source snapshot: examples/05_seven_sketches.rs</summary>

```rust,ignore
{{#include ../../examples/05_seven_sketches.rs}}
```

</details>

## Paper Map To Rust

Use this table as the navigation layer.

| Paper area | Main content | Rust companion |
| --- | --- | --- |
| Generative effects | preorders, monotone maps, Galois connections | `InformationLevel`, `FeatureCount`, `LayerBudget` |
| Resources | monoidal preorders, resource composition, enrichment | `ResourceBundle`, `ResourceAmount` |
| Databases | schemas as categories, instances as functors | `CompanyInstance`, `EmployeeRecord`, `DepartmentId` |
| Co-design | feasibility relations and profunctor-like reasoning | `DesignRequirement`, `ImplementationOffer`, `FeasibilityRelation` |
| Signal flow | syntax, semantics, matrices, composition | `SignalMatrix`, `SignalCoefficient` |
| Circuits | open systems, ports, serial and parallel composition | `OpenCircuit`, `CircuitComponent`, `PortName` |
| Logic of behavior | truth values, intervals, local-to-global checks | `TruthValue`, `TimeInterval`, `SafetyCover` |

Each section below uses the same three-part lens:

```text
Rust syntax
ML or software concept
Category theory concept
```

## Sketch 1: Information Order

The problem this block solves is:

> Some concepts are ordered by refinement. An observation can be refined into a
> feature, a feature into a score, and a score into a decision.

The block begins:

```rust,ignore
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InformationLevel {
    Observation,
    Feature,
    Score,
    Decision,
}
```

## Rust Syntax

This is an enum.

The variants are ordered because the enum derives:

```rust,ignore
PartialOrd, Ord
```

That means Rust can compare:

```rust,ignore
InformationLevel::Observation <= InformationLevel::Decision
```

The methods:

```rust,ignore
pub fn can_flow_to(self, target: Self) -> bool {
    self <= target
}

pub fn join(self, other: Self) -> Self {
    self.max(other)
}
```

reuse that ordering.

`can_flow_to` checks whether information can move upward.

`join` returns the more informative of two levels in this total order.

## ML Or Software Concept

ML systems often move through levels of processed information:

```text
raw observation
  -> extracted feature
  -> model score
  -> final decision
```

The order prevents treating a low-level observation as if it were already a
decision.

## Category Theory Concept

This is a preorder-shaped example.

A preorder needs:

```text
reflexivity: a <= a
transitivity: if a <= b and b <= c, then a <= c
```

The law check:

```rust,ignore
information_order_obeys_preorder_laws()
```

iterates over the finite set and verifies those rules.

## Sketch 1 Continued: Feature And Layer Galois Law

The problem this block solves is:

> A concrete feature count and an abstract layer budget are different worlds,
> but they can be coordinated by a law.

The key types:

```rust,ignore
pub struct FeatureCount(usize);
pub struct LayerBudget(usize);
```

The conversion functions:

```rust,ignore
pub fn abstract_to_layer_budget(features: FeatureCount) -> CtResult<LayerBudget>

pub fn concretize_layer_budget(layers: LayerBudget) -> FeatureCount
```

## Rust Syntax

Both `FeatureCount` and `LayerBudget` are newtypes around `usize`.

Their constructors reject zero because neither a zero feature count nor a zero
layer budget is useful in this model.

`abstract_to_layer_budget` divides feature count by `FEATURES_PER_LAYER` and
rounds up:

```rust,ignore
features.value().div_ceil(FEATURES_PER_LAYER)
```

`concretize_layer_budget` multiplies layers by features per layer.

## ML Or Software Concept

This models capacity planning.

Concrete features might be:

```text
9 measured feature channels
```

The abstract layer budget might be:

```text
3 layers
```

The law says the two planning views agree about what fits.

## Category Theory Concept

The checked law is:

```text
abstract(features) <= layers
if and only if
features <= concretize(layers)
```

That is the shape of a Galois connection.

The two directions are not inverses.

They are coordinated by an order law.

## Sketch 2: Resources

The problem this block solves is:

> Independent resources need a way to combine, and resource supply must respect
> demand ordering.

The key block:

```rust,ignore
pub struct ResourceBundle {
    compute: ResourceAmount,
    memory: ResourceAmount,
}
```

## Rust Syntax

`ResourceAmount` wraps a `usize`.

`ResourceBundle` stores two resource dimensions:

```text
compute
memory
```

The monoidal operation is:

```rust,ignore
pub fn tensor(&self, other: &Self) -> Self
```

It adds compute to compute and memory to memory.

The preorder check is:

```rust,ignore
pub fn can_supply(&self, demand: &Self) -> bool
```

It returns true only when every resource component is large enough.

## ML Or Software Concept

This is the shape of deployment capacity:

```text
encoder resources + decoder resources = combined resources
```

A machine can satisfy a demand only when it has enough compute and memory.

## Category Theory Concept

This is a monoidal preorder.

The preorder is `can_supply`.

The monoidal product is `tensor`.

The law check:

```rust,ignore
resource_tensor_is_monotone()
```

shows that adding the same fixed resource bundle to both sides preserves the
order.

## Sketch 3: Database Instance

The problem this block solves is:

> A database row with a foreign key should not point at a missing row.

The key types:

```rust,ignore
pub struct DepartmentId(usize);
pub struct EmployeeId(usize);

pub struct EmployeeRecord {
    id: EmployeeId,
    department: DepartmentId,
}

pub struct CompanyInstance {
    departments: Vec<DepartmentId>,
    employees: Vec<EmployeeRecord>,
}
```

## Rust Syntax

`DepartmentId` and `EmployeeId` are distinct newtypes.

That prevents mixing department IDs and employee IDs.

`EmployeeRecord` contains:

```text
employee id
department id
```

`CompanyInstance::new` collects departments and employees, then checks every
employee department exists:

```rust,ignore
if !departments.contains(&employee.department()) {
    return Err(CtError::ShapeMismatch { ... });
}
```

## ML Or Software Concept

Many ML systems depend on structured data.

If a row references missing data, downstream feature extraction or training can
fail later in a confusing place.

This code rejects invalid relational structure at construction time.

## Category Theory Concept

The schema can be read as:

```text
Employee -> Department
```

An instance assigns sets of rows to schema objects.

The foreign key is a function from employees to departments.

`CompanyInstance::new` checks that the function is defined for every employee.

## Sketch 4: Co-Design Feasibility

The problem this block solves is:

> Some relationships are not functions. A requirement and an implementation
> offer are related only when constraints are satisfied.

The key types:

```rust,ignore
pub struct DesignRequirement {
    minimum_throughput: Throughput,
    maximum_latency: LatencyMs,
}

pub struct ImplementationOffer {
    throughput: Throughput,
    latency: LatencyMs,
}

pub struct FeasibilityRelation;
```

## Rust Syntax

`Throughput` and `LatencyMs` are validated newtypes.

`DesignRequirement` stores the minimum acceptable throughput and maximum
acceptable latency.

`ImplementationOffer` stores what an implementation actually provides.

The relation is:

```rust,ignore
pub fn relates(requirement: DesignRequirement, offer: ImplementationOffer) -> bool {
    offer.throughput >= requirement.minimum_throughput
        && offer.latency <= requirement.maximum_latency
}
```

## ML Or Software Concept

This models feasibility:

```text
Can this model/service/deployment satisfy this requirement?
```

For example:

```text
required throughput: at least 100 requests/sec
required latency: at most 80 ms
offer: 120 requests/sec and 50 ms
```

The offer is feasible.

## Category Theory Concept

This is relation-shaped rather than function-shaped.

It is the small Bool-valued version of profunctor-like reasoning:

```text
Requirement x Offer -> Bool
```

Not every design problem should be forced into:

```text
A -> B
```

Some should be modeled as constraints or relations.

## Sketch 5: Signal Matrices

The problem this block solves is:

> Signal-flow diagrams need executable semantics. In this companion, matrices
> provide that meaning.

The key types:

```rust,ignore
pub struct SignalCoefficient(i32);
pub struct MatrixRows(usize);
pub struct MatrixCols(usize);

pub struct SignalMatrix {
    rows: MatrixRows,
    cols: MatrixCols,
    coefficients: Vec<Vec<SignalCoefficient>>,
}
```

## Rust Syntax

`MatrixRows` and `MatrixCols` reject zero.

`SignalMatrix::new` validates that the coefficient matrix has the promised
shape:

```text
number of rows matches MatrixRows
number of columns in every row matches MatrixCols
```

The composition method:

```rust,ignore
pub fn compose_after(&self, previous: &Self) -> CtResult<Self>
```

requires compatible middle dimensions.

Then it performs matrix multiplication using:

```text
add
multiply
sum over the middle dimension
```

## ML Or Software Concept

This is the same shape as composing linear layers or signal-processing stages.

If one stage maps:

```text
A -> B
```

and another maps:

```text
B -> C
```

then the composite maps:

```text
A -> C
```

The dimensions must line up.

## Category Theory Concept

Signal-flow syntax gets matrix semantics.

The important principle is functorial semantics:

```text
meaning(composed diagram)
=
composition of meanings
```

The code enforces the same middle-dimension law that ordinary morphism
composition enforces.

## Sketch 6: Open Circuits

The problem this block solves is:

> A circuit is not only internal components. It also has a boundary where it can
> connect to other circuits.

The key types:

```rust,ignore
pub struct PortName(&'static str);
pub struct ResistanceOhms(usize);

pub struct CircuitComponent {
    from: PortName,
    to: PortName,
    resistance: ResistanceOhms,
}

pub struct OpenCircuit {
    inputs: Vec<PortName>,
    outputs: Vec<PortName>,
    components: Vec<CircuitComponent>,
}
```

## Rust Syntax

`PortName::new` rejects empty names.

`ResistanceOhms::new` rejects zero resistance.

`OpenCircuit::new` rejects circuits with no inputs or no outputs.

Serial composition:

```rust,ignore
pub fn then(&self, next: &Self) -> CtResult<Self>
```

checks:

```text
self output count == next input count
```

Parallel composition:

```rust,ignore
pub fn parallel(&self, other: &Self) -> CtResult<Self>
```

puts the two boundaries side by side.

## ML Or Software Concept

This looks like component architecture:

```text
input interface
internal implementation
output interface
```

Composition should fail when interfaces do not match.

That rule applies to services, data pipelines, neural layers, and circuit-like
systems.

## Category Theory Concept

This is the open-system idea.

The boundary is part of the object.

Composition is controlled by boundary compatibility.

The paper develops this with cospans, hypergraph categories, decorated
cospans, and operads. The Rust code gives a small typed analogue.

## Sketch 7: Logic Of Behavior

The problem this block solves is:

> A system may be safe on local time intervals. The code needs a way to combine
> local safety checks into one global result.

The key types:

```rust,ignore
pub enum TruthValue {
    False,
    True,
}

pub struct TimeTick(usize);

pub struct TimeInterval {
    start: TimeTick,
    end: TimeTick,
}

pub struct LocalSafetyCheck {
    interval: TimeInterval,
    truth: TruthValue,
}

pub struct SafetyCover(Vec<LocalSafetyCheck>);
```

## Rust Syntax

`TruthValue` implements Boolean-style operations:

```rust,ignore
and
implies
```

`TimeInterval::new` rejects intervals where start is after end.

`SafetyCover::new` rejects an empty list of checks.

`global_truth` folds all local truths with `and`:

```rust,ignore
self.0
    .iter()
    .fold(TruthValue::True, |truth, check| truth.and(check.truth()))
```

## ML Or Software Concept

This models safety or behavior validation over time:

```text
check interval 0..5
check interval 5..10
combine into global result
```

If every local check is true, global truth is true.

If any local check is false, global truth is false.

## Category Theory Concept

This is a small analogue of local-to-global reasoning.

The sheaf-like idea is:

```text
local facts can determine a global fact when they glue coherently
```

The code uses a simple conjunction model, not full sheaf theory.

The important lesson is that proof-like information becomes explicit data, not
an informal comment.

## Tests As Exercise Solutions

The problem this block solves is:

> The laws should be runnable, not only described in prose.

The test module checks:

- preorder laws
- feature/layer Galois law
- resource tensor monotonicity
- database foreign-key resolution
- feasibility relation behavior
- signal matrix composition
- open circuit serial and parallel composition
- local-to-global truth

## Rust Syntax

Every law is a normal Rust test marked with:

```rust,ignore
#[test]
```

Tests that may fail through constructors return:

```rust,ignore
CtResult<()>
```

so they can use `?`.

## ML Or Software Concept

The tests act as executable learning checks.

If a future change breaks a law, the project should fail quickly.

## Category Theory Concept

The tests are small law checks.

They are not formal proofs, but they keep the implementation aligned with the
claimed structure.

## Run The Companion

Run:

```bash
cargo run --example 05_seven_sketches
```

For the full validation gate:

```bash
bash scripts/check.sh
```

## Core Mental Model

In Rust terms:

```text
each sketch becomes concrete types, constructors, methods, and tests
```

In ML or software terms:

```text
orders, resources, schemas, feasibility, signal flow, interfaces, and safety
are all engineering structures
```

In category-theory terms:

```text
the useful part is compositionality: make structure visible, then make
composition obey laws
```

## What To Remember

The seven sketches are not seven disconnected topics.

They repeat one engineering discipline:

```text
name the objects
name the relationships
control construction
define composition
check the law
```

That is also the discipline used by the tiny ML pipeline in the rest of the
course.

## Further Reading

- [References](references.md): paper links and supporting Rust/materials
- [Glossary](glossary.md): terms used by the course
- [Repository Source Snapshots](source-snapshots.md): complete source files
