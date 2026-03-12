//! Code for generating probabilities.

/// Returns a string of 100 probabilities, each labelled `labeli`, where
/// `i` is a number in `0..100`.
pub fn generate_probs_100(label: &str) -> Vec<String> {
    let mut lines = Vec::with_capacity(102);
    lines.push("start_random".to_string());
    for i in 0..100 {
        lines.push(format!("percent_chance 1 #define {label}{i}"));
    }
    lines.push("end_random".to_string());
    lines
}
