use crate::domain::Case;

/// Build a Prolog `decide/2` query without motivation.
pub fn build_decide_query(case: &Case) -> String {
    format!("decide({}, Verdict).", case)
}

/// Build a Prolog `decide/3` query with motivation.
pub fn build_decide_query_with_motivation(case: &Case) -> String {
    format!("decide({}, Verdict, Motivation).", case)
}

#[cfg(test)]
mod tests {
    use super::{build_decide_query, build_decide_query_with_motivation};
    use crate::domain::Case;

    #[test]
    fn builds_decide_predicates() {
        let case = Case::new("ctx{intent: read}").expect("case is valid");

        assert_eq!(
            build_decide_query(&case),
            "decide(ctx{intent: read}, Verdict)."
        );
        assert_eq!(
            build_decide_query_with_motivation(&case),
            "decide(ctx{intent: read}, Verdict, Motivation)."
        );
    }
}
