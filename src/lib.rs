//! A small, modular Rust tutorial for category-theory ideas in tiny ML.
//!
//! The crate is intentionally split into small learning modules:
//! - [`domain`] defines the nouns: tokens, vectors, probabilities, losses, and parameters.
//! - [`category`] defines the arrows: morphisms, identity, composition, and endomorphisms.
//! - [`ml`] implements concrete ML morphisms.
//! - [`training`] turns one optimizer step into an endomorphism on parameters.
//! - [`structure`] covers functors, natural transformations, and monoids.
//! - [`calculus`] shows the chain rule as a local backward pass.
//! - [`sketches`] gives Rust models for the seven applied-category-theory sketches.
//! - [`demo`] connects the pieces into the terminal walkthrough.

pub mod calculus;
pub mod category;
pub mod demo;
pub mod domain;
pub mod error;
pub mod ml;
pub mod sketches;
pub mod structure;
pub mod training;

pub use calculus::{LocalGradient, MulOp, Scalar};
pub use category::{
    Compose, Endomorphism, Identity, Morphism, StepCount, apply_endomorphism_n_times,
};
pub use demo::run_demo;
pub use domain::{
    Distribution, LearningRate, Logits, Loss, ModelDimension, Parameters, Product, TokenId,
    TokenSequence, TrainingExample, TrainingSet, Vector, VocabSize,
};
pub use error::{CtError, CtResult};
pub use ml::{
    CrossEntropy, DatasetWindowing, DirectPredict, Embedding, LinearToLogits, Softmax,
    average_loss, composed_prediction_matches_direct_prediction,
};
pub use sketches::{
    CircuitComponent, CompanyInstance, DepartmentId, DesignRequirement, EmployeeId, EmployeeRecord,
    FeasibilityRelation, FeatureCount, ImplementationOffer, InformationLevel, LatencyMs,
    LayerBudget, LocalSafetyCheck, MatrixCols, MatrixRows, OpenCircuit, PortName, ResistanceOhms,
    ResourceAmount, ResourceBundle, SafetyCover, SignalCoefficient, SignalMatrix, Throughput,
    TimeInterval, TimeTick, TruthValue, abstract_to_layer_budget, concretize_layer_budget,
    feature_layer_galois_law_holds, information_order_obeys_preorder_laws,
    resource_tensor_is_monotone,
};
pub use structure::{
    Functor, Monoid, NaturalTransformation, OptionFunctor, PipelineTrace, TraceStep, VecFunctor,
    VecToFirstOption, monoid_laws_hold_for_pipeline_trace,
    naturality_square_holds_for_first_option,
};
pub use training::TrainStep;
