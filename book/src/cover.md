<div align="center">

<h1>Category Theory for Tiny ML in Rust</h1>

<p>A practical bridge between compositional mathematics, Rust types, and tiny
machine-learning systems</p>

<p><strong>Working Draft · Public Feedback Edition</strong></p>

<p>Coauthored by<br>
<a href="https://www.linkedin.com/in/hamze/">Hamze Ghalebi</a><br>
<a href="https://www.linkedin.com/in/farzad-jafarranmani-5a1b4768/">Farzad Jafarranmani</a></p>

<p><a href="https://github.com/hghalebi/category_theory_transformer_rs">GitHub repository</a></p>

</div>

## About This Book

Category Theory for Tiny ML in Rust is a working draft that develops a small,
explicit machine-learning system through the lens of category theory and Rust.

The book is designed for readers who want to understand machine learning not
only as numerical computation, but as a structured pipeline of objects,
transformations, composition, and constraints.

Rather than treating category theory as decorative abstraction, this book uses
it as an engineering tool:

- domain objects become Rust types,
- morphisms become typed transformations,
- composition becomes executable program structure,
- training becomes repeated transformation of model state,
- and tiny ML systems become a way to make mathematical structure concrete.

This is not a finalized edition. Chapters, examples, terminology, diagrams,
code, and references may evolve as the work continues.

The current public edition is still worth publishing: readers can use the
existing chapters, run the Rust examples, and send evidence-shaped feedback
while later completion passes continue.

The public source repository is available at
[github.com/hghalebi/category_theory_transformer_rs](https://github.com/hghalebi/category_theory_transformer_rs).

## Public Workshop

The first public workshop for this book and Rust lab is hosted through AI
Reading Club. It introduces the tiny ML pipeline as typed Rust structure and
uses the working draft as the shared study material.

<a href="https://luma.com/event/evt-Pb1kYMQvzs8JrQq" class="luma-checkout--button" data-luma-action="checkout" data-luma-event-id="evt-Pb1kYMQvzs8JrQq">Register for Event</a>

<script id="luma-checkout" src="https://embed.lu.ma/checkout-button.js"></script>

## Coauthors

### Hamze Ghalebi

Hamze Ghalebi is a Paris-based AI architect, CTO, and software builder
associated with Remo Lab. His work focuses on production GenAI, regulated AI
systems, auditable AI products, Rust systems, and the transition from AI
prototypes to reliable production architectures.

His background includes advanced study at Institut Polytechnique de Paris across
statistics, optimization, machine learning, artificial intelligence, distributed
systems, cloud computing, and data science.

Hamze brings the engineering and product perspective of the book: how to turn
mathematical and machine-learning ideas into understandable, typed,
maintainable systems. His current work is especially concerned with AI systems
that can be evaluated, monitored, audited, and kept under human accountability
in real operational environments.

In this book, his role is to connect tiny ML, Rust implementation, and
production-minded software architecture — because apparently making category
theory executable was not ambitious enough already.

### Farzad Jafarranmani

Farzad Jafarranmani is a researcher and engineer in the Paris area, associated
with Huawei and the Lagrange Mathematics and Computing Research Center. His work
sits at the intersection of mathematics, computer science, logic, semantics,
proof theory, and category theory.

He holds a PhD in Mathematics and Computer Science from Université Paris Cité,
where his doctoral work focused on fixpoints of types in linear logic from a
Curry–Howard–Lambek perspective. He also studied Mathematics and Computer
Science at ENS Paris-Saclay, with work including induction in fibred
multicategories and denotational semantics of linear logic with least and
greatest fixpoints.

His previous research experience includes postdoctoral work at LIP6, Laboratoire
d'Informatique de Sorbonne Université / CNRS, as well as a visiting research
position at the University of Cambridge.

Farzad brings the mathematical and theoretical foundation of the book: category
theory, denotational semantics, proof theory, type-theoretic structure, and the
discipline required to keep abstractions precise instead of merely fashionable.

## Public Feedback

Public feedback is welcome while the book is still growing.

Useful feedback includes unclear explanations, broken examples, missing
references, awkward terminology, incorrect or overloaded mathematical language,
Rust examples that could be clearer or more idiomatic, and places where the
connection between Rust, machine learning, and category theory should be made
more explicit.

Feedback is easiest to act on when it is opened in the
[GitHub repository](https://github.com/hghalebi/category_theory_transformer_rs)
with a specific chapter, command, or source file.

Use the public
[review path](https://github.com/hghalebi/category_theory_transformer_rs/blob/main/docs/review-path.md)
if you want a short route for reviewing the book. If you want the shareable
public reviewer call, use
[Reviewers Needed](https://github.com/hghalebi/category_theory_transformer_rs/blob/main/REVIEWERS.md).
If several readers are reviewing together, use the
[public review sprint](https://github.com/hghalebi/category_theory_transformer_rs/blob/main/docs/review-sprint.md)
to split reports across Rust, ML, category-theory, educator, and beginner
perspectives. Use the
[chapter clarity form](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml)
when you can name the first unclear sentence, output line, table row, code
block, or exercise.

If you are reading the online book and cannot clone the repository right now,
you can still review one public page. Read Welcome, Course Map, Domain Objects,
or Morphism and Composition, then report the first visible sentence, heading,
diagram, table row, code block, or exercise prompt that becomes unclear. Put
`public book path` in the command or page field and include one evidence signal
from the page. A no-clone report is useful only when it names a public page and
one visible signal; broad praise or broad confusion is not enough.

This edition is intentionally public before it is final.

## Citation, Reuse, And Support

Short version:

- The public book will always remain open access at
  [hghalebi.github.io/category_theory_transformer_rs](https://hghalebi.github.io/category_theory_transformer_rs/).
- The source repository is available at
  [github.com/hghalebi/category_theory_transformer_rs](https://github.com/hghalebi/category_theory_transformer_rs).
- Cite both the public book URL and the source repository URL where reuse is
  allowed.
- Personal and individual study are allowed with clear citation.
- One reader may study, cite, link, clone, and run the project for personal
  learning.
- Commercial or organizational group reuse involving more than one person
  requires written permission before reproducing, adapting, distributing, or
  teaching material from the book or repository beyond short quotation, linking,
  review, and individual-study allowances. This includes company workshops,
  internal team workshops, classes, cohorts, courses, and training programs.
- Company workshops, internal team workshops, paid workshops, commercial
  training programs, course packs, adapted slide decks, handouts, labs, and
  workshop packets require written permission when they reuse substantial
  material from this project.
- When Kindle or hard copy editions are available, buying the Kindle version or
  a hard copy supports continued public work. Paid editions are support
  editions, not access gates.

The public book will always remain open access at
[hghalebi.github.io/category_theory_transformer_rs](https://hghalebi.github.io/category_theory_transformer_rs/).

The source repository is available at
[github.com/hghalebi/category_theory_transformer_rs](https://github.com/hghalebi/category_theory_transformer_rs).

These are custom citation-and-permission terms. Open access means the public
book remains free to read online; it does not mean unrestricted commercial
redistribution, commercial training use, company workshop use, or
organizational group reuse.

The source code is published so readers can inspect, run, test, and contribute
to the examples. Substantial reproduced code, prose, exercises, diagrams, or
adapted teaching material used in a commercial or organizational group setting
follows the same written-permission rule.

Original material from this book may be linked, quoted in short form, reviewed,
or discussed for personal, academic, or noncommercial educational use when the
source is clearly referenced and both coauthors are credited.

Suggested reference format. Use both the public book URL and the source
repository URL:

```text
Ghalebi, H., & Jafarranmani, F.
Category Theory for Tiny ML in Rust.
Open-access working draft.
Book: https://hghalebi.github.io/category_theory_transformer_rs/
Source: https://github.com/hghalebi/category_theory_transformer_rs
```

Use this citation in permitted reuse contexts such as papers, posts, slides,
course notes, workshop pages, repositories, and public references.

Keep both URLs in public references. The book page is the open-access reading
surface; the source repository is the executable Rust source for the examples.

Reproducing, adapting, distributing, or teaching material from this book or
repository in a commercial or organizational setting that involves more than one
person requires written permission from the project owners, except for short
quotation, linking, review, and individual-study allowances. This includes
company workshops, company-sponsored workshops, internal team workshops, company
reading groups based on copied or adapted material, paid training material,
course packs, adapted slide decks, handouts, labs, and workshop packets. If the
material is reused by or for a company, team, class, workshop, cohort, course,
or training program with more than one person, request written permission
first. Citation is required where reuse is allowed, but citation alone is not
permission and does not replace written permission for commercial or
organizational group reuse. See the repository
[license and reuse terms](https://github.com/hghalebi/category_theory_transformer_rs/blob/main/LICENSE.md).

Company workshops, internal team workshops, paid workshops, and workshop
packets count as commercial or organizational group reuse when they reproduce,
adapt, distribute, or teach substantial material from the book or repository.

Permission requests should start through the
[source repository](https://github.com/hghalebi/category_theory_transformer_rs),
for example by opening an issue or contacting the maintainers from the
repository page.

Opening an issue or sending a request does not itself grant permission. Only an
explicit written approval from a project owner or maintainer grants permission
for the requested commercial or organizational group use.

When Kindle or hard copy editions are available, buying the Kindle version or a
hard copy is a way to support continued work on the project. Paid editions are
support editions, not access gates. Paid editions will not remove free public
access to the online book.

External works cited or referenced by this book remain under their own licenses,
terms, and attribution requirements.
