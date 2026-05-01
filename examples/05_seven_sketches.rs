use category_theory_transformer_rs::{
    CircuitComponent, CompanyInstance, CtResult, DepartmentId, DesignRequirement, EmployeeId,
    EmployeeRecord, FeasibilityRelation, FeatureCount, ImplementationOffer, InformationLevel,
    LatencyMs, LayerBudget, LocalSafetyCheck, MatrixCols, MatrixRows, OpenCircuit, PortName,
    ResistanceOhms, ResourceAmount, ResourceBundle, SafetyCover, SignalCoefficient, SignalMatrix,
    Throughput, TimeInterval, TimeTick, TruthValue, abstract_to_layer_budget,
    concretize_layer_budget, feature_layer_galois_law_holds, information_order_obeys_preorder_laws,
    resource_tensor_is_monotone,
};

fn main() -> CtResult<()> {
    println!(
        "orders obey preorder laws: {}",
        information_order_obeys_preorder_laws()
    );
    println!(
        "join(feature, decision): {:?}",
        InformationLevel::Feature.join(InformationLevel::Decision)
    );

    let features = FeatureCount::new(9)?;
    let layers = LayerBudget::new(3)?;
    println!(
        "feature/layer Galois law: {}",
        feature_layer_galois_law_holds(features, layers)?
    );
    println!(
        "abstract 9 features to {} layers; concretize 3 layers to {} features",
        abstract_to_layer_budget(features)?.value(),
        concretize_layer_budget(layers).value()
    );

    let encoder = ResourceBundle::new(ResourceAmount::new(2), ResourceAmount::new(8));
    let decoder = ResourceBundle::new(ResourceAmount::new(3), ResourceAmount::new(10));
    println!("combined resource bundle: {:?}", encoder.tensor(&decoder));
    println!(
        "resource tensor monotone: {}",
        resource_tensor_is_monotone()
    );

    let research = DepartmentId::new(1);
    let platform = DepartmentId::new(2);
    let ada = EmployeeId::new(7);
    let instance =
        CompanyInstance::new([research, platform], [EmployeeRecord::new(ada, research)])?;
    println!(
        "employee {:?} belongs to department {:?}",
        ada,
        instance.department_of(ada)
    );

    let requirement = DesignRequirement::new(Throughput::new(100)?, LatencyMs::new(80)?);
    let offer = ImplementationOffer::new(Throughput::new(120)?, LatencyMs::new(50)?);
    println!(
        "co-design offer feasible: {}",
        FeasibilityRelation::relates(requirement, offer)
    );

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
    println!(
        "signal-flow matrix semantics: {:?}",
        composed.coefficients()
    );

    let input = PortName::new("input")?;
    let middle = PortName::new("middle")?;
    let output = PortName::new("output")?;
    let first_circuit = OpenCircuit::new(
        [input],
        [middle],
        [CircuitComponent::resistor(
            input,
            middle,
            ResistanceOhms::new(10)?,
        )],
    )?;
    let second_circuit = OpenCircuit::new(
        [middle],
        [output],
        [CircuitComponent::resistor(
            middle,
            output,
            ResistanceOhms::new(20)?,
        )],
    )?;
    println!(
        "serial circuit component count: {}",
        first_circuit.then(&second_circuit)?.component_count()
    );

    let safety = SafetyCover::new([
        LocalSafetyCheck::new(
            TimeInterval::new(TimeTick::new(0), TimeTick::new(5))?,
            TruthValue::True,
        ),
        LocalSafetyCheck::new(
            TimeInterval::new(TimeTick::new(5), TimeTick::new(10))?,
            TruthValue::True,
        ),
    ])?;
    println!("global behavior truth: {:?}", safety.global_truth());

    Ok(())
}
