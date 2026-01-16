/// Build a Prolog query to check for required predicates.
pub fn build_required_predicates_query(predicates: &[&str]) -> String {
    let checks = predicates
        .iter()
        .map(|p| format!("current_predicate({})", p))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{checks}.")
}
