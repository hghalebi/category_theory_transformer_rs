# Seven Sketches Through Rust

This chapter is a Rust companion to Brendan Fong and David I. Spivak's
*Seven Sketches in Compositionality: An Invitation to Applied Category
Theory*. The paper is book-length, so this chapter does not reproduce it. It
turns every main sketch into a small typed Rust model you can run, inspect, and
test.

The paper's repeated pattern is:

```text
structure + coherence -> compositional software model
```

Rust gives that pattern a concrete form:

```text
newtypes + smart constructors + typed composition -> fewer invalid states
```

## How to Read the Companion

Use the paper for the mathematics and this chapter for executable intuition.
For each sketch, ask three questions:

1. What are the objects?
2. What are the arrows or relationships?
3. What law makes composition reliable?

In Rust, those questions usually become:

```rust,ignore
struct ObjectName(/* private representation */);

impl ObjectName {
    pub fn new(value: /* raw input */) -> CtResult<Self> {
        // validate the invariant at the boundary
    }
}
```

Then composition becomes an ordinary method, trait implementation, or function
whose type signature says what can connect to what.

## Paper Map to Rust

Use this map when moving between the paper and the repository.

| Paper area | Main content | Rust companion |
| --- | --- | --- |
| Preface and reading guide | structure, coherence, applications, exercises | course learning loop and tests |
| 1. Generative effects | preorders, monotone maps, meets, joins, Galois connections, closure | `InformationLevel`, `FeatureCount`, `LayerBudget` |
| 2. Resources | monoidal preorders, wiring diagrams, enriched categories, quantale-style matrix multiplication | `ResourceBundle`, `ResourceAmount` |
| 3. Databases | schemas as categories, instances as functors, natural transformations, data migration, limits and colimits | `CompanyInstance`, `EmployeeRecord`, `DepartmentId` |
| 4. Co-design | Bool-valued feasibility, enriched profunctors, profunctor composition, compact closed structure | `DesignRequirement`, `ImplementationOffer`, `FeasibilityRelation` |
| 5. Signal flow | props, presentations, rigs, matrix semantics, graphical linear algebra, feedback | `SignalMatrix`, `SignalCoefficient` |
| 6. Circuits | cospans, pushouts, hypergraph categories, decorated cospans, electric circuits, operads | `OpenCircuit`, `CircuitComponent`, `PortName` |
| 7. Logic of behavior | Set-like toposes, subobject classifiers, sheaves, predicates, quantification, temporal safety | `TruthValue`, `TimeInterval`, `LocalSafetyCheck`, `SafetyCover` |
| Exercise solutions | active checks for the laws introduced in the sketches | unit tests in `src/sketches.rs` |

## Sketch 1: Generative Effects, Orders, and Adjunctions

The first sketch studies order. A preorder is a relationship that is reflexive
and transitive. That is enough structure to talk about refinement, implication,
abstraction, closure, and Galois connections.

In this repository, `InformationLevel` is a small finite order:

```rust,ignore
Observation <= Feature <= Score <= Decision
```

The Rust lesson is that an order is not just a list. It is a contract:

```rust,ignore
InformationLevel::Observation.can_flow_to(InformationLevel::Decision)
```

The helper `information_order_obeys_preorder_laws()` checks the law directly.
That mirrors the paper's habit of making coherence explicit instead of hoping
the reader infers it.

The Galois connection appears as a pair of conversions:

```rust,ignore
FeatureCount -> LayerBudget
LayerBudget -> FeatureCount
```

`abstract_to_layer_budget` compresses concrete features into an abstract layer
budget. `concretize_layer_budget` expands a layer budget back to feature
capacity. The checked law is:

```text
abstract(features) <= layers
if and only if
features <= concretize(layers)
```

That is the engineering lesson behind adjunctions: two directions can be
different but still coordinated by one law.

## Sketch 2: Resources, Monoidal Preorders, and Enrichment

The second sketch studies resources. The practical question is whether one
bundle of resources can supply another, and whether independent resources can
be combined.

The Rust model is:

```rust,ignore
ResourceBundle {
    compute: ResourceAmount,
    memory: ResourceAmount,
}
```

The preorder is `can_supply`. The monoidal product is `tensor`, which combines
independent bundles component by component.

The important law is monotonicity:

```text
if large can supply small,
then large tensor fixed can supply small tensor fixed
```

That is the resource-theory version of "composition should preserve meaning."
It is also the same software rule you want when composing model components,
pipelines, deployment capacity, or data-processing steps.

The enrichment part of the paper generalizes the idea of a yes/no relationship.
Instead of asking only "is this reachable?", an enriched relationship can carry
cost, distance, latency, probability, or any other structured quantity. In Rust,
that usually means replacing `bool` with a semantic newtype or enum.

## Sketch 3: Databases, Categories, Functors, and Limits

The third sketch treats a database schema as a category. Tables are objects.
Foreign keys are arrows. A database instance is a functor from that schema into
sets.

The Rust model is deliberately small:

```rust,ignore
DepartmentId
EmployeeId
EmployeeRecord { id, department }
CompanyInstance
```

`EmployeeRecord.department` is the schema arrow:

```text
Employee -> Department
```

`CompanyInstance::new` validates that every employee points at an existing
department. That is the same boundary rule used throughout this repository:
invalid data should be rejected before it reaches the core model.

The paper then uses functors and adjunctions to explain data migration:

```text
source schema -> target schema
```

The Rust translation is a typed conversion boundary. Do not leak one external
schema shape through the whole program. Convert at the edge, validate the
foreign-key law, and keep the core model coherent.

Limits and colimits explain universal ways to merge, project, summarize, or
join structured data. The programming lesson is that a good schema API should
make common migration and joining patterns explicit instead of leaving them as
untyped string manipulation.

## Sketch 4: Co-Design, Profunctors, and Monoidal Categories

The fourth sketch asks a design question:

```text
Can this implementation satisfy this requirement?
```

That relationship is not a normal function. One requirement may be satisfied by
many offers, and one offer may satisfy many requirements. The paper models that
shape with profunctors.

The Rust model is:

```rust,ignore
DesignRequirement {
    minimum_throughput: Throughput,
    maximum_latency: LatencyMs,
}

ImplementationOffer {
    throughput: Throughput,
    latency: LatencyMs,
}
```

`FeasibilityRelation::relates(requirement, offer)` is a Bool-valued
relationship. It returns true only when the offer provides enough throughput
and stays under the latency boundary.

The software lesson is that not every relationship should become `fn A -> B`.
Sometimes the correct model is a relation, a search space, a constraint, or a
feasibility check.

The monoidal part matters because designs compose. If a feature, service, and
deployment each satisfy their local contracts, the larger design can be checked
by composing those relationships rather than restarting the analysis from
scratch.

## Sketch 5: Signal Flow Graphs, Props, Presentations, and Proofs

The fifth sketch separates syntax from semantics.

Signal-flow graphs are syntax:

```text
copy this signal
multiply it by a gain
add two signals
wire boxes together
```

Matrices are semantics:

```text
the whole graph computes this linear map
```

The Rust model uses `SignalMatrix` as the semantic side. Matrix composition is
the executable version of "wire these two signal processors in series":

```rust,ignore
let composed = add_weighted.compose_after(&duplicate)?;
```

The example duplicates one signal and then applies weights `2` and `3`, so the
resulting one-input, one-output behavior has coefficient `5`.

The deeper lesson is functorial semantics:

```text
big graph meaning
=
meaning of small pieces, composed in the same shape as the graph
```

That is exactly what you want from a typed ML pipeline. The code path and the
mathematical meaning should compose in the same order.

## Sketch 6: Circuits, Hypergraph Categories, and Operads

The sixth sketch studies open systems. A circuit is not only a closed object.
It has boundary ports where it can connect to other circuits.

The Rust model is:

```rust,ignore
OpenCircuit {
    inputs,
    outputs,
    components,
}
```

Serial composition wires outputs into inputs:

```rust,ignore
let serial = first_circuit.then(&second_circuit)?;
```

Parallel composition keeps interfaces side by side:

```rust,ignore
let parallel = first_circuit.parallel(&second_circuit)?;
```

Hypergraph categories give the algebra of sharing, merging, copying, and
discarding wires. Decorated cospans give a disciplined way to keep the boundary
separate from the internal decoration. Operads describe the valid wiring
patterns.

In software architecture, the same idea shows up whenever a component has:

- an external interface
- internal implementation details
- valid composition rules

The boundary should be typed. The implementation details should stay internal.
Composition should fail early if the interfaces do not match.

## Sketch 7: Logic of Behavior, Sheaves, Toposes, and Languages

The seventh sketch studies behavior and proof. The practical question is:

```text
How can we prove a system is safe over time?
```

The Rust model uses:

```rust,ignore
TimeInterval
LocalSafetyCheck
SafetyCover
TruthValue
```

Each `LocalSafetyCheck` is a truth value over one interval. `SafetyCover` glues
local checks into a global result:

```rust,ignore
let global = safety.global_truth();
```

This is a small programming analogue for the sheaf idea: local information
should combine into global information only when the overlap behavior is
coherent.

Toposes give a setting where objects, predicates, and logic live together. In
Rust terms, this means a behavior model should not keep proofs as comments or
informal promises. It should represent them as values with explicit
constructors and checkable composition.

## Appendix Role: Exercises and Solutions

The paper includes many exercises and an appendix of solutions. This repository
uses Rust tests for the same learning role.

Every law in `src/sketches.rs` has a small test:

- preorder laws
- Galois-connection law
- monoidal resource monotonicity
- database foreign-key resolution
- co-design feasibility
- signal-flow matrix composition
- open-circuit serial and parallel composition
- local-to-global behavior truth

The discipline is the same:

```text
do not only read the law; run it
```

## Run the Companion

```bash
cargo run --example 05_seven_sketches
```

For the full validation gate:

```bash
bash scripts/check.sh
```

## Source Snapshot: Seven Sketches Module

<details>
<summary>Source snapshot: src/sketches.rs</summary>

```rust,ignore
{{#include ../../src/sketches.rs}}
```

</details>

## Source Snapshot: Runnable Example

<details>
<summary>Source snapshot: examples/05_seven_sketches.rs</summary>

```rust,ignore
{{#include ../../examples/05_seven_sketches.rs}}
```

</details>

## What to Remember

The paper is about compositionality. The Rust version is about making
composition inspectable:

- orders become explicit comparison methods
- adjunctions become coordinated conversion pairs
- resources become monoidal bundles
- database schemas become typed references
- design feasibility becomes a relation
- graph syntax gets matrix semantics
- circuits get typed open interfaces
- behavior proofs become values that can be checked locally and globally

The common rule is simple: make structure visible, then make composition obey a
law.

## Further Reading

- [References](references.md): paper links and supporting Rust/materials
- [Glossary](glossary.md): terms used by the course
- [Repository Source Snapshots](source-snapshots.md): complete source files
