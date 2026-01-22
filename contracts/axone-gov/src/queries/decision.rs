use crate::domain::Case;

/// Build a Prolog decide query without motivation.
pub fn build_decide_query(case: &Case) -> String {
    format!("decide({}, Verdict).", case)
}

/// Build a Prolog decide query with motivation.
pub fn build_decide_query_with_motivation(case: &Case) -> String {
    format!("decide({}, Verdict, Motivation).", case)
}
