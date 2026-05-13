# FAQ

## Is this a production ML framework?

No. This is a teaching crate and public book. The implementation is deliberately
small so the whole system can be inspected.

## Why Rust?

Rust makes data shape, ownership, errors, and invariants explicit. That is a
good fit for teaching category-theory ideas as engineering structure.

## Why category theory?

Category theory gives names to typed transformations, composition, products,
endomorphisms, functors, and laws. Those names help explain why the tiny ML
pipeline is organized the way it is.

## Where should I start?

Run:

```bash
cargo run --example 01_token_sequence
```

Then read [START_HERE.md](../START_HERE.md).

## How can I help?

Open a specific issue. The best first issues point to one unclear sentence, one
missing diagram, one Rust example, or one exercise idea.

If you are reporting reader confusion, use the closest public route:

| Perspective | Report link |
| --- | --- |
| Rust engineer | [Open Rust engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+Rust+engineer+brief&location=docs%2Frust-path.md&command=cargo+run+--example+01_domain_objects%0Acargo+run+--example+02_morphism_composition%0Acargo+test+domain%3A%3Atests+--lib%0Acargo+test+category%3A%3Atests+--lib) |
| ML engineer or learner | [Open ML engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+ML+engineer+brief&location=docs%2Fml-path.md&command=cargo+run+--example+01_token_sequence%0Acargo+run+--bin+category_ml%0Acargo+run+--example+03_training_endomorphism%0Acargo+run+--example+07_transformer_training_state) |
| Category-theory reader | [Open category-theory reader report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+category-theory+reader+brief&location=book%2Fsrc%2Froadmap.md+-%3E+Category+Shape+Diagnostic+-%3E+Reader+Evidence+Handoff&command=cargo+run+--example+06_attention_scores) |
| Technical educator | [Open technical educator report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+technical+educator+brief&location=README.md%2C+START_HERE.md%2C+docs%2Feducator-path.md%2C+book%2Fsrc%2Fwelcome.md%2C+book%2Fsrc%2F00-map.md%2C+or+book%2Fsrc%2Fexercises.md&command=public+book+review+path+or+local+file+review) |
| Beginner-adjacent learner | [Open beginner-adjacent learner report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+beginner-adjacent+reader+brief&location=Welcome%2C+Course+Map%2C+Domain+Objects%2C+or+Morphism+and+Composition&command=cargo+run+--example+01_token_sequence+or+public+book+path) |

The link fills the route, not the evidence. The evidence signal should come
from what you personally read, ran, or attempted.

If you want examples before opening an issue, read
[review-examples.md](review-examples.md).

## How can a reading club or workshop report feedback?

Use one report per participant confusion point. The strongest report still
comes from the participant who read a page, ran a command, or attempted an
exercise.

If a participant cannot open GitHub during the session, a facilitator can
transcribe one report into the quick or detailed form. Use `reading-session
report` or `workshop report` in the command or page field, name the public
page, chapter, exercise, command, or output line the participant used, and
preserve the participant's last clear idea and first unclear point.

Do not include names, email addresses, private messages, or contact details.
Do not merge several people's comments into one report. A summary that only
says the group was confused is not direct reader evidence.

## Can I use this in a company workshop?

Not without written permission if you reproduce, adapt, distribute, or teach
material from the book or repository in a commercial or organizational setting
that involves more than one person, except for short quotation, linking, review,
and individual-study allowances.

That includes company workshops, company-sponsored workshops, internal team
workshops, paid courses, company reading groups based on copied or adapted
material, corporate course packs, adapted slide decks, handouts, labs, workshop
packets, and other group training uses.

If the material is reused by or for a company, team, class, workshop, cohort,
course, or training program with more than one person, request written
permission first.

Individual readers may link to the public book, cite the project, clone the
repository, and run the examples for personal study. You may also use short
quotations for commentary or review. See [LICENSE.md](../LICENSE.md) for the
full reuse terms.

Plain rule: one reader may study, cite, link, clone, and run the project for
personal learning. A company, team, class, cohort, workshop, course, or other
commercial or organizational group with more than one person needs written
permission before reproducing, adapting, distributing, or teaching substantial
material from the book or repository.

The source code is published so readers can inspect, run, test, and contribute
to the examples. Substantial reproduced code, prose, exercises, diagrams, or
adapted teaching material used in a commercial or organizational group setting
follows the same written-permission rule.

Permission requests should start through the source repository. Opening an
issue or sending a request does not itself grant permission. Only an explicit
written approval from a project owner or maintainer grants permission for the
requested commercial or organizational group use.

Source repository:
<https://github.com/hghalebi/category_theory_transformer_rs>

Citation is still required, but citation alone is not permission and does not
replace written permission for commercial or organizational group reuse.

Company workshops, internal team workshops, paid workshops, and workshop
packets count as commercial or organizational group reuse when they reproduce,
adapt, distribute, or teach substantial material from the book or repository.

## How do I cite it?

Use this citation. Include both the public book URL and the source repository
URL:

```text
Ghalebi, H., & Jafarranmani, F.
Category Theory for Tiny ML in Rust.
Open-access working draft.
Book: https://hghalebi.github.io/category_theory_transformer_rs/
Source: https://github.com/hghalebi/category_theory_transformer_rs
```

The public book URL and the source repository URL should both appear in the
citation.

Keep both URLs in public references. The book page is the open-access reading
surface; the source repository is the executable Rust source for the examples.

Use this citation in permitted reuse contexts such as papers, posts, slides,
course notes, workshop pages, repositories, and public references.

## How can I support the project?

The online book will always remain open access at
<https://hghalebi.github.io/category_theory_transformer_rs/>. When Kindle or
hard copy editions are available, buying the Kindle version or a hard copy is a
way to support continued public work. Paid editions are support editions, not
access gates. Paid editions will not remove free public access to the online
book.

## What does this project unlock?

Framework-only tutorials often teach the call:

```text
model(input)
```

This project teaches the structure that call compresses.

After the first path, a reader should be able to:

- see `TokenId`, `TokenSequence`, `TrainingSet`, `Parameters`,
  `Distribution`, and `Loss` as different ML objects, not loose values
- explain a tiny pipeline as typed transformations:

  ```text
  Text -> TokenSequence -> TrainingPairs -> ModelState -> Prediction -> Loss
  ```

- recognize composition as the rule that lets one stage feed the next
- describe training as a controlled update of model state
- read a larger framework API with better questions about inputs, outputs,
  hidden state, shape, loss, and update steps

The unlock is not a production-scale model. The unlock is an inspectable mental
model for what larger frameworks automate.

Run the first concrete example:

```bash
cargo run --example 01_token_sequence
```
