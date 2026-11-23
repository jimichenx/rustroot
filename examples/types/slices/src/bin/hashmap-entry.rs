use std::collections::HashMap;

fn main() {
    // ANCHOR: naive_count
    let words = ["rust", "types", "rust", "ownership", "rust"];
    let mut naive_counts: HashMap<&str, usize> = HashMap::new();
    for word in words {
        if !naive_counts.contains_key(word) {
            naive_counts.insert(word, 0);
        }
        let next = naive_counts[word] + 1;
        naive_counts.insert(word, next);
    }
    // ANCHOR_END: naive_count

    // ANCHOR: entry_count
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for word in words {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!(
        "rust: {:?}, types: {:?} (same as naive: {})",
        counts.get("rust"),
        counts.get("types"),
        counts == naive_counts
    );
    // ANCHOR_END: entry_count
}
