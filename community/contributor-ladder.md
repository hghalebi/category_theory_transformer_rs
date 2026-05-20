# Contributor Ladder

The project needs more than code. Readers can become contributors by making the
learning path clearer.

## Level 1: Reader Feedback

Open a chapter clarity report that points to one confusing sentence, command,
or example:

<https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml>

Use the closest report link if you already know your perspective. The link
fills the route, not the evidence; the evidence signal should come from what
you personally read, ran, or attempted.

| Perspective | Report link |
| --- | --- |
| Rust engineer | [Open Rust engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+Rust+engineer+brief&location=docs%2Frust-path.md&command=cargo+run+--example+01_domain_objects%0Acargo+run+--example+02_morphism_composition%0Acargo+test+domain%3A%3Atests+--lib%0Acargo+test+category%3A%3Atests+--lib) |
| ML engineer or learner | [Open ML engineer report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+ML+engineer+brief&location=docs%2Fml-path.md&command=cargo+run+--example+01_token_sequence%0Acargo+run+--bin+category_ml%0Acargo+run+--example+03_training_endomorphism%0Acargo+run+--example+07_transformer_training_state) |
| Category-theory reader | [Open category-theory reader report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+category-theory+reader+brief&location=docs%2Fcategory-theory-path.md+-%3E+Seven+Sketches+Transfer+Drill%3B+book%2Fsrc%2Froadmap.md+-%3E+Category+Shape+Diagnostic&command=cargo+run+--example+05_seven_sketches%0Acargo+run+--example+06_attention_scores) |
| Technical educator | [Open technical educator report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+technical+educator+brief&location=README.md%2C+START_HERE.md%2C+docs%2Feducator-path.md%2C+book%2Fsrc%2Fwelcome.md%2C+book%2Fsrc%2F00-map.md%2C+or+book%2Fsrc%2Fexercises.md&command=public+book+review+path+or+local+file+review) |
| Beginner-adjacent learner | [Open beginner-adjacent learner report](https://github.com/hghalebi/category_theory_transformer_rs/issues/new?template=chapter-clarity.yml&title=%5Bgood+first+feedback%5D+beginner-adjacent+reader+brief&location=Welcome%2C+Course+Map%2C+Domain+Objects%2C+or+Morphism+and+Composition&command=cargo+run+--example+01_token_sequence+or+public+book+path) |

The strongest first contribution names the command or page tried, the last
clear idea, the first unclear point, and the smallest fix that would help.

If the first contribution is about a reference, citation, source-backed claim,
or missing source, use the optional source-role field in the quick or detailed
report form. Name the source role, owned boundary, supported claim,
unsupported claim, and local Rust file, type, example, or chapter section. The
useful report is not "add more citations"; it points to one source boundary
that would make one local claim easier to verify. Use the source-role table in
`book/src/references.md` and the source-role example in
[docs/review-examples.md](../docs/review-examples.md).

## Level 2: Documentation Clarifier

Submit a small pull request that improves one paragraph, glossary entry, or
checkpoint.

## Level 3: Example Contributor

Add or improve one runnable Rust example and make sure `cargo test
--all-targets --all-features` passes.

## Level 4: Reviewer

Review pull requests for one lens:

- Rust idiom
- ML intuition
- category-theory precision
- learner clarity

## Level 5: Maintainer-Level Contributor

Own a roadmap milestone, keep the validation gate green, and help convert
reader questions into structured work.
