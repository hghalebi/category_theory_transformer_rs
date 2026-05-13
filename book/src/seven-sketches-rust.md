# Seven Sketches Through Rust

The problem this chapter solves is:

> Applied category theory can feel too large to connect to code. This chapter
> turns the seven major themes of *Seven Sketches in Compositionality* into
> small Rust blocks with tests.

The previous chapter introduced reusable structures such as functors, natural
transformations, monoids, and local derivative rules. This chapter keeps the
same reading style but changes scale. Instead of following the tiny ML pipeline,
it takes seven broader ideas and asks how each one can be made concrete enough
to inspect in Rust.

This chapter does not reproduce the paper. It gives executable handles for the
ideas.

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

> Reader orientation:
> Treat each sketch as a small modeling exercise. You are not expected to know
> the full mathematical theory before reading the Rust. Start from the type,
> then read the constructor, then read the law check.

## Chapter Outcomes

By the end of this chapter, you should be able to:

- name the one law, relation, or boundary each Rust sketch preserves,
- explain which part of the source text each sketch deliberately does not
  implement,
- transfer one sketch to a small software or ML design problem without
  overclaiming the mathematics.

## What You Already Know

If you have modeled business rules, resource limits, database relationships, or
state machines in Rust, you already know the software version of this chapter.
The new idea is that applied category theory gives names to those compositional
structures and asks which laws make them trustworthy.

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

## Source Scope Contract

This chapter is a study companion, not a replacement for the source text. Each
Rust model preserves one inspectable idea and deliberately leaves the larger
mathematical development outside the tiny example.

| Paper area | What the Rust sketch preserves | What it does not claim |
| --- | --- | --- |
| Generative effects | order laws and one Galois-style capacity law | a complete treatment of generative effects |
| Resources | componentwise resource composition plus monotonicity | full enriched category theory |
| Databases | schema-like reference integrity through typed IDs | a general database semantics framework |
| Co-design | feasibility as a relation between requirements and offers | full profunctor theory |
| Signal flow | matrix composition and middle-dimension checks | a complete syntax-and-semantics account for signal-flow graphs |
| Circuits | open interfaces and serial boundary matching | a full circuit algebra |
| Logic of behavior | local interval truth combined into a global claim | sheaf theory or a full temporal logic |

Use the table as a precision guard. When the Rust code checks one law, say
which law it checks. When the source paper develops a larger theory, do not
pretend the small Rust model has implemented all of it.

## PDF-To-Rust Reading Contract

The arXiv record describes *Seven Sketches in Compositionality* as a long
invitation to applied category theory, built around concrete examples and
seven major sketches. That matters for how to use this chapter. The goal is
not to compress every page into a smaller page. The goal is to give every
major sketch a Rust handle that a reader can run, inspect, and test.

Complete coverage in this companion chapter means:

```text
every major sketch area has a Rust handle;
every Rust handle names one protected law, relation, or boundary;
every protected claim says what the source text still develops beyond the code;
every reader can run one command before arguing about the abstraction.
```

Use this ledger while reading the PDF beside the Rust:

| When the source text discusses | Ask in this chapter | Local evidence |
| --- | --- | --- |
| an order, relation, or refinement | Which enum, newtype, or method names the ordered world? | `InformationLevel::can_flow_to`, `FeatureCount`, `LayerBudget` |
| a resource or compositional quantity | Which operation combines independent pieces? | `ResourceBundle::tensor` |
| a schema, instance, or reference | Which constructor rejects invalid references? | `CompanyInstance::new` |
| a feasibility relation | Which requirement-offer pair is accepted or rejected? | `FeasibilityRelation::relates` |
| a signal-flow or matrix composition | Which middle dimension must match? | `SignalMatrix::compose_after` |
| an open system or circuit interface | Which boundary ports must agree? | `OpenCircuit::then` |
| a local-to-global behavior claim | Which local checks combine into a global result? | `SafetyCover::global_truth` |

This is also the limit of the chapter. If a PDF section develops a richer
construction than the Rust handle, record the richer construction as context,
not as something the code has proved. The safe sentence is:

```text
The source develops a larger theory here.
This Rust handle checks one executable boundary from that theory.
```

## Source-Backed Precision Rules

This chapter uses external sources as scope guards. Each source supports a
limited teaching claim, and each claim is tied to one local Rust boundary or
test. The chapter does not claim that `src/sketches.rs` implements the full
source text, a general categorical semantics library, or a production ML
architecture theory.

| Source | What the source supports | Local rule in this chapter | Rust evidence |
| --- | --- | --- | --- |
| [Seven Sketches](https://arxiv.org/abs/1803.05316) | Applied category theory can be introduced through concrete examples such as databases, circuits, dynamical systems, and other real-world structures. | Treat every sketch as one executable handle for one source idea, not as a replacement for the full mathematical development. | `InformationLevel`, `ResourceBundle`, `CompanyInstance`, `OpenCircuit`, `SafetyCover` |
| [MIT Applied Category Theory OCW](https://ocw.mit.edu/courses/18-s097-applied-category-theory-january-iap-2019/) | The seven topic areas can be studied as a course sequence: orders, resources, databases, co-design, signal flow, circuits, and logic of behavior. | Keep the chapter order and `Paper Map To Rust` aligned with that applied-category sequence. | `cargo run --example 05_seven_sketches` |
| [Category Theory for Programming](https://arxiv.org/abs/2209.01259) | Category-theory vocabulary can be taught through programming-shaped structures. | Explain the programming boundary before naming the category-theory pattern. | `information_order_obeys_preorder_laws`, `feature_layer_galois_law_holds`, `resource_tensor_is_monotone` |
| [Compositional Deep Learning](https://arxiv.org/abs/1907.08292) | Categorical schemas, functorial structure, and composition invariants can appear in neural-network settings under stated assumptions. | Use the database and co-design sketches as ML transfer analogies only; do not claim the crate learns functors or implements the thesis. | `CompanyInstance`, `FeasibilityRelation`, `database_instance_rejects_missing_department_reference` |
| [Categorical Deep Learning](https://proceedings.mlr.press/v235/gavranovic24a.html) | Architecture discussions can separate constraints a model should satisfy from implementations that realize them. | Use co-design as a tiny `Requirement x Offer -> Bool` boundary, not as a theory of all neural architectures. | `DesignRequirement`, `ImplementationOffer`, `FeasibilityRelation::relates` |
| [Rust Book: Enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) | Enums encode values that must be one variant from a finite set. | Use enums for finite state-like domains before adding laws around them. | `InformationLevel`, `TruthValue` |
| [Rust Book: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html) | Traits name shared behavior and make contracts explicit. | Treat laws and methods as contracts that tests must witness, not as prose-only claims. | `SignalMatrix::compose_after`, `OpenCircuit::then`, `SafetyCover::global_truth` |

The transfer pattern is:

```text
source idea -> local Rust model -> law or boundary check
```

For this chapter, that means reading `cargo run --example
05_seven_sketches` and `cargo test sketches::tests` as evidence for the tiny
models above, not as evidence that the full seven-sketch source text,
categorical deep-learning literature, or every applied-category construction
has been implemented.

## Choose A Sketch Without Losing The Tiny ML Thread

The source paper deliberately tours many application areas. This companion
chapter keeps that breadth, but the book still has one main learning path:
small typed systems that make ML structure inspectable.

Use this table to decide how deeply to read each sketch on a first pass.

| Sketch | Read deeply when you need | Tiny ML transfer | Safe first-pass treatment |
| --- | --- | --- | --- |
| Information order | staged representations or approval states | raw text, tokens, features, scores, and decisions form ordered levels of processed information | Core transfer |
| Feature/layer planning | two views of model capacity | feature counts and layer budgets are different descriptions of model size | Optional but useful |
| Resources | deployment or training constraints | compute and memory limits constrain model choices | Optional but practical |
| Database instance | structured training data | bad references in source data should fail before training | Core transfer |
| Co-design feasibility | requirements versus implementations | a model may satisfy some accuracy, latency, or memory requirements and fail others | Core transfer |
| Signal matrices | linear maps and shape compatibility | composed linear stages need matching middle dimensions | Core transfer |
| Open circuits | component interfaces | typed ML components compose only when output and input boundaries match | Core transfer |
| Logic of behavior | local checks and global claims | every batch or interval must satisfy the invariant before the global claim is trusted | Optional but useful |

On a first reading, focus on the rows marked core transfer. They connect most
directly to the tiny ML pipeline. The optional rows are not less important;
they are just farther from the first runnable model.

The chapter is successful if you can leave each sketch with one sentence:

```text
This Rust model prevents this invalid composition.
```

## Worked Example: Ordering Information Levels

The smallest first-principles version of this chapter is an ordered enum. Rust
can derive an order for enum variants, and that gives the code a concrete way to
ask whether one information level can safely flow into another:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Level {
    Observation,
    Feature,
    Decision,
}

assert!(Level::Observation <= Level::Decision);
assert!(Level::Feature <= Level::Feature);
```

The real `src/sketches.rs` module uses the same idea with named domain types,
validation, and law checks.

## Self-Check

Before reading the first sketch, explain why `Observation <= Decision` is a
modeling rule, not just a comparison between enum variants.

## One Method Across Seven Sketches

Do not read the chapter as seven unrelated theory notes. Each sketch follows
the same modeling method:

```text
engineering problem
  -> named Rust values
  -> validated construction
  -> composition operation or relation
  -> law or boundary test
```

That method is the same one used in the tiny ML pipeline. The difference is the
domain. Instead of tokens, logits, and parameters, this chapter uses resources,
database rows, signal matrices, open circuits, and local behavior checks.

When a sketch feels abstract, ask one question:

```text
What invalid composition should this model prevent?
```

The answer usually points to the real engineering value of the categorical
shape.

The chapter's applied models can be scanned by the structure they protect:

| Sketch | Rust handle | Valid structure | Rejected or checked boundary |
| --- | --- | --- | --- |
| Information order | `InformationLevel` | information flows upward through refinement levels | preorder laws check reflexivity and transitivity |
| Feature/layer planning | `FeatureCount`, `LayerBudget` | concrete and abstract capacity views agree | Galois law checks both directions of fit |
| Resources | `ResourceBundle` | compute and memory combine componentwise | monotonicity checks supply still respects demand |
| Databases | `CompanyInstance` | employee records refer to known departments | missing department references return `Err(...)` |
| Co-design | `FeasibilityRelation` | offers satisfy throughput and latency requirements | infeasible offers are not related |
| Signal flow | `SignalMatrix` | matrices compose when middle dimensions match | mismatched dimensions return `Err(...)` |
| Open circuits | `OpenCircuit` | serial composition connects matching port boundaries | boundary mismatch returns `Err(...)` |
| Behavior logic | `SafetyCover` | local interval checks combine into global truth | unknown local truth prevents a false global guarantee |

This table is the chapter's law-and-boundary index. The point is not to
memorize eight rows. The point is to see that each sketch earns its abstraction
by protecting one concrete relationship.

### Transfer Triage Card

Use this card when a source idea feels too large to turn into code. The goal is
not to shrink the source. The goal is to choose one local boundary that can be
inspected.

| If the transfer feels like... | Do this first | Ready when you can write... |
| --- | --- | --- |
| a broad theory claim | shrink to one law, relation, or boundary | `Source claim -> local Rust handle` |
| a vocabulary list | choose one constructor, method, example line, or test | `Rust handle -> protected relationship` |
| a passing example only | name the invalid case it rejects or the law it checks | `protected relationship -> rejected shortcut` |
| an ML analogy | name the ML object, constraint, or composition it maps to | `tiny ML transfer -> safe non-claim` |
| a research source | state what the source does not license this chapter to claim | `non-claim -> evidence command` |

The completed transfer card has seven fields:

```text
source idea:
local Rust handle:
protected law, relation, or boundary:
invalid shortcut rejected:
tiny ML transfer:
larger claim not implemented:
local evidence command or test:
```

Example:

```text
source idea: schemas and instances
local Rust handle: CompanyInstance::new
protected law, relation, or boundary: EmployeeRecord -> DepartmentId must resolve
invalid shortcut rejected: letting a missing department reach feature extraction
tiny ML transfer: validate structured training rows before training
larger claim not implemented: a general categorical database semantics
local evidence command or test: cargo test sketches::tests --lib
```

This follows the source discipline used above. *Seven Sketches* gives a broad
tour through concrete examples. MIT's course frames category theory as a way
to organize formal systems and transfer knowledge between them. Categorical
deep-learning work distinguishes architecture constraints from
implementations. The local job here is smaller: choose one implementable
handle, name one protected relationship, and state the non-claim.

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

### Transfer Task: Ordered States

Model a workflow from your own codebase as an ordered enum, such as:

```rust,ignore
enum ReviewState {
    Draft,
    Reviewed,
    Published,
}
```

Then write the Rust sentence you would want to be true:

```text
Draft can flow to Published
Published cannot flow to Draft
state can always flow to itself
```

The transfer is complete when you can name the invalid flow your order prevents.

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

### Transfer Task: Concrete And Abstract Capacity

Pick one concrete measure and one abstract budget from software work:

```text
concrete: batch size, feature count, request rate
abstract: GPU count, layer budget, service tier
```

Write the two functions:

```text
abstract(concrete) -> abstract budget
concretize(abstract budget) -> concrete capacity
```

The transfer is complete when you can state the law in both directions:

```text
abstract(concrete) fits budget
if and only if
concrete fits concretize(budget)
```

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

### Transfer Task: Resource Bundle

Extend the idea to three resource dimensions:

```text
compute
memory
disk
```

Name the constructor boundary and the law check:

```text
ResourceBundle::new(compute, memory, disk)
resource_tensor_is_monotone
```

The transfer is complete when you can explain why componentwise addition should
not turn an adequate supply into an inadequate supply.

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

### Transfer Task: Foreign Key Boundary

Translate the pattern to another schema arrow:

```text
Task -> Project
Order -> Customer
Comment -> Post
```

Name two newtypes and one record:

```text
ProjectId
TaskId
TaskRecord { id: TaskId, project: ProjectId }
```

The transfer is complete when you can write the constructor failure in plain
English: "reject a task whose project id is missing from the project table."

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

This also gives a small handle for reading newer categorical deep-learning
work. At architecture scale, a model can be discussed in terms of constraints
it should satisfy and implementations that realize those constraints. This Rust
sketch keeps only the first tiny version of that idea:

```text
DesignRequirement x ImplementationOffer -> bool
```

The important distinction is:

| Question | Co-design sketch answer |
| --- | --- |
| What should be true? | throughput is high enough and latency is low enough |
| What implementation evidence do we have? | one `ImplementationOffer` value with checked throughput and latency |
| What does the relation decide? | whether that offer satisfies that requirement |

That is not a full theory of neural architectures. It is the learner-sized
boundary that prevents a common overclaim: one implementation example is not
the same thing as the whole constraint space.

### Transfer Task: Feasibility Relation

Model a relation that is not a function:

```text
Requirement x Offer -> Bool
```

For example, use:

```text
minimum accuracy
maximum memory
maximum latency
```

Then write the architecture version:

```text
ArchitectureConstraint x CandidateImplementation -> Bool
```

The transfer is complete when you can explain why many offers may satisfy one
requirement, one offer may satisfy many requirements, and one passing offer is
not proof that every future implementation satisfies the architecture
constraint.

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

### Transfer Task: Shape-Safe Composition

Write the shape of two stages before writing any coefficients:

```text
A -> B
B -> C
```

Then write one invalid pair:

```text
A -> B
D -> C
```

The transfer is complete when you can name the exact middle dimension that must
match for matrix composition to make sense.

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

### Transfer Task: Interface Boundary

Model two components by boundary only:

```text
Tokenizer: Text -> TokenSequence
Embedder: TokenSequence -> HiddenSequence
```

Then write one invalid serial composition:

```text
Tokenizer: Text -> TokenSequence
Classifier: Logits -> Label
```

The transfer is complete when you can say which output boundary failed to match
which input boundary.

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

### Transfer Task: Local Checks To Global Claim

Pick one invariant over time:

```text
loss is finite
latency stays below the budget
no batch has an empty token sequence
```

Split it into local intervals and assign a truth value to each interval.

The transfer is complete when you can explain why one false local check must
make the global claim false.

## Tests As Exercise Solutions

The problem this block solves is:

> The laws should be runnable, not only described in prose.

The test module checks preorder laws, the feature/layer Galois law, resource
tensor monotonicity, database foreign-key resolution, feasibility relation
behavior, signal matrix composition, open circuit serial and parallel
composition, and local-to-global truth.

It also includes negative boundary tests. A database instance with a missing
department reference is rejected. Signal matrices with incompatible middle
dimensions cannot compose. Open circuits with mismatched serial boundaries do
not wire together. Those failures are part of the teaching point: the model is
useful because it rejects incoherent structure near the boundary.

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

The output gives one executable handle per sketch:

```text
orders obey preorder laws: true
feature/layer Galois law: true
resource tensor monotone: true
employee EmployeeId(7) belongs to department Some(DepartmentId(1))
co-design offer feasible: true
signal-flow matrix semantics: [[SignalCoefficient(5)]]
serial circuit component count: 2
global behavior truth: True
Typed transformation:
InformationLevel <= InformationLevel checks preorder
FeatureCount <-> LayerBudget checks Galois law
ResourceBundle x ResourceBundle -> ResourceBundle
EmployeeRecord -> DepartmentId must resolve in CompanyInstance
DesignRequirement x ImplementationOffer -> bool
SignalMatrix x SignalMatrix -> SignalMatrix when dimensions match
OpenCircuit x OpenCircuit -> OpenCircuit when ports match
SafetyCover -> TruthValue
```

## Example Output Transfer Checklist

Use the companion output as a boundary map. Each line should tell you which
structure is being protected.

| Example output | Rust handle | Protected structure | Shortcut to reject |
| --- | --- | --- | --- |
| `orders obey preorder laws: true` | `InformationLevel` | information can flow upward through an ordered refinement path | treating an enum order as arbitrary display order |
| `feature/layer Galois law: true` | `FeatureCount`, `LayerBudget` | concrete and abstract capacity views agree by a two-way fit law | treating abstraction and concretization as inverse functions |
| `resource tensor monotone: true` | `ResourceBundle::tensor` | adding resources componentwise preserves supply ordering | combining compute and memory as one raw number |
| `employee ... belongs to department ...` | `CompanyInstance::new` | every employee department reference resolves | letting a missing foreign key reach later feature extraction |
| `co-design offer feasible: true` | `FeasibilityRelation::relates` | requirements and offers form a relation, not a single function | forcing every design question into `A -> B` |
| `signal-flow matrix semantics: ...` | `SignalMatrix::compose_after` | matrix meanings compose only when middle dimensions match | multiplying stages before checking shape |
| `serial circuit component count: 2` | `OpenCircuit::then` | serial composition requires matching boundaries | wiring components by name alone |
| `global behavior truth: True` | `SafetyCover::global_truth` | local checks combine into a global claim | claiming global safety while one local interval is false |

The typed lines at the bottom of the output are not extra decoration. They name
the form each sketch protects: order, Galois law, monoidal resource
composition, database instance, feasibility relation, matrix composition, open
system composition, and local-to-global truth.

For the full validation gate:

```bash
cargo test --all-targets --all-features
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

## Where This Leaves Us

This chapter showed that the book's main discipline is not limited to language
modeling. Orders, resources, database instances, feasibility relations, signal
matrices, open circuits, and local behavior checks can all be read in the same
way: name the values, constrain construction, define composition, and test the
law that makes the composition trustworthy.

The remaining practice material in [Exercises](exercises.md) asks you to use
that reading method yourself. The exercises are not meant to test memorized
definitions. They are meant to train the habit of translating one Rust block
into its software role and its categorical shape.

## Further Reading

Do not treat these sources as a separate theory shelf. Use them to improve one
local Rust sentence at a time.

Start from this local evidence:

```text
cargo run --example 05_seven_sketches
cargo test sketches::tests --lib
src/sketches.rs
examples/05_seven_sketches.rs
```

Then read the sources in this order:

| Source | What to transfer back into this chapter | Local evidence to inspect |
| --- | --- | --- |
| [Seven Sketches](https://arxiv.org/abs/1803.05316) | Each sketch is one compositional structure: orders, resources, schemas, co-design, signal flow, open systems, or local-to-global behavior. | `Paper Map To Rust`, `Source Scope Contract`, `Example Output Transfer Checklist` |
| [MIT Applied Category Theory OCW](https://ocw.mit.edu/courses/18-s097-applied-category-theory-january-iap-2019/) | The seven topic areas form a study sequence, not a bag of unrelated examples. | `cargo run --example 05_seven_sketches` output order |
| [Category Theory for Programming](https://arxiv.org/abs/2209.01259) | Programming examples should carry the vocabulary before the formal name is emphasized. | `InformationLevel`, `ResourceBundle`, `SignalMatrix`, `OpenCircuit` |
| [Compositional Deep Learning](https://arxiv.org/abs/1907.08292) | Categorical schemas and composition invariants can appear in neural-network settings under explicit assumptions. | `CompanyInstance`, `FeasibilityRelation`, `database_instance_rejects_missing_department_reference` |
| [Categorical Deep Learning](https://proceedings.mlr.press/v235/gavranovic24a.html) | Architecture-level claims should separate constraints from implementations that realize them. | `DesignRequirement x ImplementationOffer -> bool` |
| [Rust Book: Enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) | Finite domains are often clearer as enums than as unstructured numbers. | `InformationLevel`, `TruthValue` |
| [Rust Book: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html) | Shared behavior should become an explicit contract before laws are tested around it. | `SignalMatrix::compose_after`, `OpenCircuit::then`, `SafetyCover::global_truth` |

After reading one source, answer four questions:

1. Which Rust handle did it clarify?
2. Which law, relation, or boundary did it support?
3. Which larger claim did the source not license?
4. Which command or test shows the local evidence?

For this chapter, the commands are:

```bash
cargo run --example 05_seven_sketches
cargo test sketches::tests --lib
```

For terminology recovery, use:

- [References](references.md): paper links and supporting Rust/materials
- [Glossary](glossary.md): terms used by the course
- [Repository Source Snapshots](source-snapshots.md): complete source files

If a source does not help you name a Rust handle, a law or boundary, a
non-claim, and an evidence command, it has not transferred back into this
chapter yet.

## Practice After This Chapter

Use [Exercise 10](exercises.md#exercise-10-test-one-sketch-law) to test one
sketch law or inspect one negative test. The goal is to choose a structure,
name the invalid state it rejects, and explain the result through Rust syntax,
software or ML meaning, and category-theory shape.

## Retrieval Practice

### Recall

Recover the chapter map before choosing a transfer example.

Name the sketch that models ordered refinement from observation to decision.

Name the sketch that rejects an employee row whose department is missing.

Name the sketch that rejects matrix composition when the middle dimensions do
not match.

Name the sketch that rejects serial component composition when output and input
boundaries do not match.

### Explain

Explain the protected boundary.

For `InformationLevel`, explain why `Observation` can flow to `Decision` but a
decision should not silently flow backward to an observation.

For `CompanyInstance`, explain why missing department references should be
rejected at construction time instead of during later feature extraction.

For `SignalMatrix`, explain why the middle dimension must match before matrix
composition can run.

For `OpenCircuit`, explain why a boundary mismatch is an interface error, not a
numeric error.

### Apply

Use the companion output as evidence.

The example prints:

```text
orders obey preorder laws: true
feature/layer Galois law: true
resource tensor monotone: true
signal-flow matrix semantics: [[SignalCoefficient(5)]]
serial circuit component count: 2
global behavior truth: True
```

Choose two printed lines. For each one, write:

```text
Rust value or function:
software meaning:
category-theory shape:
invalid structure prevented or law checked:
```

Then pick one engineering concept from your own work, such as permissions,
queues, schema references, resource budgets, or service interfaces. Describe
the objects, relationships, and one law or boundary test you would want the
code to check.

### Debug

For each broken model, name the missing structure:

```text
using raw usize for both EmployeeId and DepartmentId
letting a missing department reference enter the training data
composing signal matrices without checking the middle dimension
serially wiring components by name without checking boundary counts
claiming global safety when one local interval is false
```

A strong answer should use the chapter's repeated method:

```text
name the objects
name the relationships
control construction
define composition
check the law
```

The goal is not to memorize every sketch. The goal is to recognize what each
model refuses to let pass as valid structure.
