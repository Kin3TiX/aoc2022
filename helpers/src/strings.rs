use std::collections::HashSet;

pub fn first_shared_character(a: &str, b: &str) -> char {

    let (shorter, longer) = if a.len() > b.len() {
        (b, a)
    } else {
        (a, b)
    };

    let set: HashSet<char> = shorter.chars().collect();

    longer.chars().find(|c| set.contains(&c))
        .expect("Could not find any shared characters")

}