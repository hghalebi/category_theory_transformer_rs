//! Rust models for the seven applied-category-theory sketches.
//!
//! This module is a study companion for *Seven Sketches in Compositionality*.
//! It does not try to encode all of category theory. Instead, each section
//! gives one small Rust model for the main structure of a sketch: orders,
//! resources, databases, co-design, signal flow, circuits, and logic of
//! behavior.

use crate::error::{CtError, CtResult};

/// A finite order used for the first sketch: observations can be refined into
/// features, scores, and decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InformationLevel {
    Observation,
    Feature,
    Score,
    Decision,
}

impl InformationLevel {
    /// Returns true when information at this level can flow into `target`.
    pub fn can_flow_to(self, target: Self) -> bool {
        self <= target
    }

    /// Least upper bound in this small total order.
    pub fn join(self, other: Self) -> Self {
        self.max(other)
    }
}

const INFORMATION_LEVELS: [InformationLevel; 4] = [
    InformationLevel::Observation,
    InformationLevel::Feature,
    InformationLevel::Score,
    InformationLevel::Decision,
];

/// Checks reflexivity and transitivity for the finite information order.
pub fn information_order_obeys_preorder_laws() -> bool {
    for level in INFORMATION_LEVELS {
        if !level.can_flow_to(level) {
            return false;
        }
    }

    for first in INFORMATION_LEVELS {
        for second in INFORMATION_LEVELS {
            for third in INFORMATION_LEVELS {
                let premise = first.can_flow_to(second) && second.can_flow_to(third);
                if premise && !first.can_flow_to(third) {
                    return false;
                }
            }
        }
    }

    true
}

/// Number of concrete features in a tiny model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FeatureCount(usize);

impl FeatureCount {
    pub fn new(value: usize) -> CtResult<Self> {
        if value == 0 {
            return Err(CtError::EmptyInput("feature count"));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// Number of abstract layers used to summarize feature capacity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LayerBudget(usize);

impl LayerBudget {
    pub fn new(value: usize) -> CtResult<Self> {
        if value == 0 {
            return Err(CtError::EmptyInput("layer budget"));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

const FEATURES_PER_LAYER: usize = 4;

/// Abstracts concrete features to the minimum layer budget that can hold them.
pub fn abstract_to_layer_budget(features: FeatureCount) -> CtResult<LayerBudget> {
    LayerBudget::new(features.value().div_ceil(FEATURES_PER_LAYER))
}

/// Concretizes an abstract layer budget back to feature capacity.
pub fn concretize_layer_budget(layers: LayerBudget) -> FeatureCount {
    FeatureCount(layers.value() * FEATURES_PER_LAYER)
}

/// Checks the Galois-connection law for the feature/layer abstraction.
pub fn feature_layer_galois_law_holds(
    features: FeatureCount,
    layers: LayerBudget,
) -> CtResult<bool> {
    let abstracted_fits = abstract_to_layer_budget(features)?.value() <= layers.value();
    let concrete_fits = features.value() <= concretize_layer_budget(layers).value();

    Ok(abstracted_fits == concrete_fits)
}

/// A non-negative amount in a resource bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ResourceAmount(usize);

impl ResourceAmount {
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// Compute and memory resources composed by component-wise addition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceBundle {
    compute: ResourceAmount,
    memory: ResourceAmount,
}

impl ResourceBundle {
    pub fn new(compute: ResourceAmount, memory: ResourceAmount) -> Self {
        Self { compute, memory }
    }

    pub fn compute(&self) -> ResourceAmount {
        self.compute
    }

    pub fn memory(&self) -> ResourceAmount {
        self.memory
    }

    /// Monoidal product for independent resources.
    pub fn tensor(&self, other: &Self) -> Self {
        Self {
            compute: ResourceAmount::new(self.compute.value() + other.compute.value()),
            memory: ResourceAmount::new(self.memory.value() + other.memory.value()),
        }
    }

    /// Resource preorder: supply can cover demand when every component is large
    /// enough.
    pub fn can_supply(&self, demand: &Self) -> bool {
        self.compute >= demand.compute && self.memory >= demand.memory
    }
}

/// Demonstrates monotonicity of the monoidal resource operation.
pub fn resource_tensor_is_monotone() -> bool {
    let small_supply = ResourceBundle::new(ResourceAmount::new(2), ResourceAmount::new(8));
    let large_supply = ResourceBundle::new(ResourceAmount::new(4), ResourceAmount::new(16));
    let fixed = ResourceBundle::new(ResourceAmount::new(1), ResourceAmount::new(2));

    large_supply.can_supply(&small_supply)
        && large_supply
            .tensor(&fixed)
            .can_supply(&small_supply.tensor(&fixed))
}

/// Identifier for a department row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DepartmentId(usize);

impl DepartmentId {
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// Identifier for an employee row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EmployeeId(usize);

impl EmployeeId {
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// A row in the employee table. The department field is the schema arrow
/// `Employee -> Department`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmployeeRecord {
    id: EmployeeId,
    department: DepartmentId,
}

impl EmployeeRecord {
    pub fn new(id: EmployeeId, department: DepartmentId) -> Self {
        Self { id, department }
    }

    pub fn id(&self) -> EmployeeId {
        self.id
    }

    pub fn department(&self) -> DepartmentId {
        self.department
    }
}

/// A tiny database instance: schema objects become sets of ids, and schema
/// arrows become functions between those sets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanyInstance {
    departments: Vec<DepartmentId>,
    employees: Vec<EmployeeRecord>,
}

impl CompanyInstance {
    pub fn new(
        departments: impl IntoIterator<Item = DepartmentId>,
        employees: impl IntoIterator<Item = EmployeeRecord>,
    ) -> CtResult<Self> {
        let departments = departments.into_iter().collect::<Vec<_>>();
        let employees = employees.into_iter().collect::<Vec<_>>();

        for employee in &employees {
            if !departments.contains(&employee.department()) {
                return Err(CtError::ShapeMismatch {
                    op: "database instance",
                    expected: String::from("employee departments exist in Department"),
                    got: format!("missing department {}", employee.department().value()),
                });
            }
        }

        Ok(Self {
            departments,
            employees,
        })
    }

    pub fn departments(&self) -> &[DepartmentId] {
        &self.departments
    }

    pub fn employees(&self) -> &[EmployeeRecord] {
        &self.employees
    }

    pub fn department_of(&self, employee_id: EmployeeId) -> Option<DepartmentId> {
        self.employees
            .iter()
            .find(|employee| employee.id() == employee_id)
            .map(EmployeeRecord::department)
    }

    pub fn foreign_keys_resolve(&self) -> bool {
        self.employees
            .iter()
            .all(|employee| self.departments.contains(&employee.department()))
    }
}

/// Requests per second needed by a design.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Throughput(usize);

impl Throughput {
    pub fn new(value: usize) -> CtResult<Self> {
        if value == 0 {
            return Err(CtError::EmptyInput("throughput"));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// Millisecond latency boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LatencyMs(usize);

impl LatencyMs {
    pub fn new(value: usize) -> CtResult<Self> {
        if value == 0 {
            return Err(CtError::EmptyInput("latency"));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// Functional need in a co-design problem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DesignRequirement {
    minimum_throughput: Throughput,
    maximum_latency: LatencyMs,
}

impl DesignRequirement {
    pub fn new(minimum_throughput: Throughput, maximum_latency: LatencyMs) -> Self {
        Self {
            minimum_throughput,
            maximum_latency,
        }
    }
}

/// Implementation offered by a candidate component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImplementationOffer {
    throughput: Throughput,
    latency: LatencyMs,
}

impl ImplementationOffer {
    pub fn new(throughput: Throughput, latency: LatencyMs) -> Self {
        Self {
            throughput,
            latency,
        }
    }
}

/// A Bool-valued feasibility relation between requirements and offers.
#[derive(Debug, Clone, Copy)]
pub struct FeasibilityRelation;

impl FeasibilityRelation {
    pub fn relates(requirement: DesignRequirement, offer: ImplementationOffer) -> bool {
        offer.throughput >= requirement.minimum_throughput
            && offer.latency <= requirement.maximum_latency
    }
}

/// A scalar coefficient in a signal-flow matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalCoefficient(i32);

impl SignalCoefficient {
    pub fn new(value: i32) -> Self {
        Self(value)
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    fn add(self, other: Self) -> Self {
        Self(self.value() + other.value())
    }

    fn multiply(self, other: Self) -> Self {
        Self(self.value() * other.value())
    }
}

/// Number of rows in a signal-flow matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatrixRows(usize);

impl MatrixRows {
    pub fn new(value: usize) -> CtResult<Self> {
        if value == 0 {
            return Err(CtError::EmptyInput("matrix rows"));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// Number of columns in a signal-flow matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatrixCols(usize);

impl MatrixCols {
    pub fn new(value: usize) -> CtResult<Self> {
        if value == 0 {
            return Err(CtError::EmptyInput("matrix columns"));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// Matrix semantics for a signal-flow graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalMatrix {
    rows: MatrixRows,
    cols: MatrixCols,
    coefficients: Vec<Vec<SignalCoefficient>>,
}

impl SignalMatrix {
    pub fn new(
        rows: MatrixRows,
        cols: MatrixCols,
        coefficients: Vec<Vec<SignalCoefficient>>,
    ) -> CtResult<Self> {
        if coefficients.len() != rows.value() {
            return Err(CtError::ShapeMismatch {
                op: "signal matrix",
                expected: format!("{} rows", rows.value()),
                got: format!("{} rows", coefficients.len()),
            });
        }

        for row in &coefficients {
            if row.len() != cols.value() {
                return Err(CtError::ShapeMismatch {
                    op: "signal matrix",
                    expected: format!("{} columns", cols.value()),
                    got: format!("{} columns", row.len()),
                });
            }
        }

        Ok(Self {
            rows,
            cols,
            coefficients,
        })
    }

    pub fn rows(&self) -> MatrixRows {
        self.rows
    }

    pub fn cols(&self) -> MatrixCols {
        self.cols
    }

    pub fn coefficients(&self) -> &[Vec<SignalCoefficient>] {
        &self.coefficients
    }

    /// Matrix composition. If `previous` is `A -> B` and `self` is `B -> C`,
    /// the result is `A -> C`.
    pub fn compose_after(&self, previous: &Self) -> CtResult<Self> {
        if previous.rows.value() != self.cols.value() {
            return Err(CtError::ShapeMismatch {
                op: "signal matrix composition",
                expected: format!("middle dimension {}", self.cols.value()),
                got: format!("middle dimension {}", previous.rows.value()),
            });
        }

        let mut coefficients =
            vec![vec![SignalCoefficient::zero(); previous.cols.value()]; self.rows.value()];

        for (output_row, output_coefficients) in coefficients.iter_mut().enumerate() {
            for (input_col, coefficient) in output_coefficients.iter_mut().enumerate() {
                let mut total = SignalCoefficient::zero();

                for middle in 0..self.cols.value() {
                    total = total.add(
                        self.coefficients[output_row][middle]
                            .multiply(previous.coefficients[middle][input_col]),
                    );
                }

                *coefficient = total;
            }
        }

        Self::new(self.rows, previous.cols, coefficients)
    }
}

/// A named boundary port in an open circuit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PortName(&'static str);

impl PortName {
    pub fn new(value: &'static str) -> CtResult<Self> {
        if value.trim().is_empty() {
            return Err(CtError::EmptyInput("port name"));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

/// Positive resistance value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResistanceOhms(usize);

impl ResistanceOhms {
    pub fn new(value: usize) -> CtResult<Self> {
        if value == 0 {
            return Err(CtError::InvalidQuantity {
                kind: "resistance",
                value: value as i64,
            });
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// A simple resistor component between two ports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CircuitComponent {
    from: PortName,
    to: PortName,
    resistance: ResistanceOhms,
}

impl CircuitComponent {
    pub fn resistor(from: PortName, to: PortName, resistance: ResistanceOhms) -> Self {
        Self {
            from,
            to,
            resistance,
        }
    }

    pub fn from(&self) -> PortName {
        self.from
    }

    pub fn to(&self) -> PortName {
        self.to
    }

    pub fn resistance(&self) -> ResistanceOhms {
        self.resistance
    }
}

/// An open circuit with input ports, output ports, and internal components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenCircuit {
    inputs: Vec<PortName>,
    outputs: Vec<PortName>,
    components: Vec<CircuitComponent>,
}

impl OpenCircuit {
    pub fn new(
        inputs: impl IntoIterator<Item = PortName>,
        outputs: impl IntoIterator<Item = PortName>,
        components: impl IntoIterator<Item = CircuitComponent>,
    ) -> CtResult<Self> {
        let inputs = inputs.into_iter().collect::<Vec<_>>();
        let outputs = outputs.into_iter().collect::<Vec<_>>();
        let components = components.into_iter().collect::<Vec<_>>();

        if inputs.is_empty() {
            return Err(CtError::EmptyInput("circuit inputs"));
        }

        if outputs.is_empty() {
            return Err(CtError::EmptyInput("circuit outputs"));
        }

        Ok(Self {
            inputs,
            outputs,
            components,
        })
    }

    pub fn input_count(&self) -> usize {
        self.inputs.len()
    }

    pub fn output_count(&self) -> usize {
        self.outputs.len()
    }

    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    /// Serial composition wires this circuit's outputs into the next circuit's
    /// inputs.
    pub fn then(&self, next: &Self) -> CtResult<Self> {
        if self.output_count() != next.input_count() {
            return Err(CtError::ShapeMismatch {
                op: "open circuit serial composition",
                expected: format!("{} next inputs", self.output_count()),
                got: format!("{} next inputs", next.input_count()),
            });
        }

        let mut components = self.components.clone();
        components.extend_from_slice(&next.components);

        Self::new(self.inputs.clone(), next.outputs.clone(), components)
    }

    /// Parallel composition keeps the two open interfaces side by side.
    pub fn parallel(&self, other: &Self) -> CtResult<Self> {
        let mut inputs = self.inputs.clone();
        inputs.extend_from_slice(&other.inputs);

        let mut outputs = self.outputs.clone();
        outputs.extend_from_slice(&other.outputs);

        let mut components = self.components.clone();
        components.extend_from_slice(&other.components);

        Self::new(inputs, outputs, components)
    }
}

/// Truth values used for behavior classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TruthValue {
    False,
    True,
}

impl TruthValue {
    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (TruthValue::True, TruthValue::True) => TruthValue::True,
            _ => TruthValue::False,
        }
    }

    pub fn implies(self, other: Self) -> Self {
        match (self, other) {
            (TruthValue::True, TruthValue::False) => TruthValue::False,
            _ => TruthValue::True,
        }
    }
}

/// Discrete time point in a behavior trace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeTick(usize);

impl TimeTick {
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// Closed interval of time ticks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeInterval {
    start: TimeTick,
    end: TimeTick,
}

impl TimeInterval {
    pub fn new(start: TimeTick, end: TimeTick) -> CtResult<Self> {
        if start > end {
            return Err(CtError::InvalidInterval {
                start: start.value(),
                end: end.value(),
            });
        }

        Ok(Self { start, end })
    }

    pub fn start(&self) -> TimeTick {
        self.start
    }

    pub fn end(&self) -> TimeTick {
        self.end
    }
}

/// Local safety result on one interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalSafetyCheck {
    interval: TimeInterval,
    truth: TruthValue,
}

impl LocalSafetyCheck {
    pub fn new(interval: TimeInterval, truth: TruthValue) -> Self {
        Self { interval, truth }
    }

    pub fn interval(&self) -> TimeInterval {
        self.interval
    }

    pub fn truth(&self) -> TruthValue {
        self.truth
    }
}

/// A cover of local behavior checks. The global result is true only when all
/// local checks are true.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafetyCover(Vec<LocalSafetyCheck>);

impl SafetyCover {
    pub fn new(checks: impl IntoIterator<Item = LocalSafetyCheck>) -> CtResult<Self> {
        let checks = checks.into_iter().collect::<Vec<_>>();

        if checks.is_empty() {
            return Err(CtError::EmptyInput("safety cover"));
        }

        Ok(Self(checks))
    }

    pub fn global_truth(&self) -> TruthValue {
        self.0
            .iter()
            .fold(TruthValue::True, |truth, check| truth.and(check.truth()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn information_order_has_preorder_laws() {
        assert!(information_order_obeys_preorder_laws());
        assert_eq!(
            InformationLevel::Feature.join(InformationLevel::Decision),
            InformationLevel::Decision
        );
    }

    #[test]
    fn feature_layer_abstraction_obeys_galois_law() -> CtResult<()> {
        let features = FeatureCount::new(9)?;
        let layers = LayerBudget::new(3)?;

        assert!(feature_layer_galois_law_holds(features, layers)?);
        assert_eq!(abstract_to_layer_budget(features)?.value(), 3);
        assert_eq!(concretize_layer_budget(layers).value(), 12);
        Ok(())
    }

    #[test]
    fn resource_tensor_is_order_preserving() {
        assert!(resource_tensor_is_monotone());
    }

    #[test]
    fn database_instance_resolves_schema_arrow() -> CtResult<()> {
        let research = DepartmentId::new(1);
        let platform = DepartmentId::new(2);
        let ada = EmployeeId::new(7);
        let instance =
            CompanyInstance::new([research, platform], [EmployeeRecord::new(ada, research)])?;

        assert!(instance.foreign_keys_resolve());
        assert_eq!(instance.department_of(ada), Some(research));
        Ok(())
    }

    #[test]
    fn feasibility_relation_matches_requirement_to_offer() -> CtResult<()> {
        let requirement = DesignRequirement::new(Throughput::new(100)?, LatencyMs::new(80)?);
        let offer = ImplementationOffer::new(Throughput::new(120)?, LatencyMs::new(50)?);

        assert!(FeasibilityRelation::relates(requirement, offer));
        Ok(())
    }

    #[test]
    fn signal_matrices_compose_like_flow_graph_semantics() -> CtResult<()> {
        let duplicate = SignalMatrix::new(
            MatrixRows::new(2)?,
            MatrixCols::new(1)?,
            vec![
                vec![SignalCoefficient::new(1)],
                vec![SignalCoefficient::new(1)],
            ],
        )?;
        let add_weighted = SignalMatrix::new(
            MatrixRows::new(1)?,
            MatrixCols::new(2)?,
            vec![vec![SignalCoefficient::new(2), SignalCoefficient::new(3)]],
        )?;

        let composed = add_weighted.compose_after(&duplicate)?;

        assert_eq!(composed.coefficients(), &[vec![SignalCoefficient::new(5)]]);
        Ok(())
    }

    #[test]
    fn open_circuits_compose_in_series_and_parallel() -> CtResult<()> {
        let input = PortName::new("input")?;
        let middle = PortName::new("middle")?;
        let output = PortName::new("output")?;
        let first = OpenCircuit::new(
            [input],
            [middle],
            [CircuitComponent::resistor(
                input,
                middle,
                ResistanceOhms::new(10)?,
            )],
        )?;
        let second = OpenCircuit::new(
            [middle],
            [output],
            [CircuitComponent::resistor(
                middle,
                output,
                ResistanceOhms::new(20)?,
            )],
        )?;

        let serial = first.then(&second)?;
        let parallel = first.parallel(&second)?;

        assert_eq!(serial.input_count(), 1);
        assert_eq!(serial.output_count(), 1);
        assert_eq!(serial.component_count(), 2);
        assert_eq!(parallel.input_count(), 2);
        assert_eq!(parallel.output_count(), 2);
        Ok(())
    }

    #[test]
    fn local_behavior_truth_glues_to_global_truth() -> CtResult<()> {
        let first = LocalSafetyCheck::new(
            TimeInterval::new(TimeTick::new(0), TimeTick::new(5))?,
            TruthValue::True,
        );
        let second = LocalSafetyCheck::new(
            TimeInterval::new(TimeTick::new(5), TimeTick::new(10))?,
            TruthValue::True,
        );
        let cover = SafetyCover::new([first, second])?;

        assert_eq!(cover.global_truth(), TruthValue::True);
        assert_eq!(
            TruthValue::True.implies(TruthValue::False),
            TruthValue::False
        );
        Ok(())
    }
}
