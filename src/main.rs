use std::collections::{HashMap, HashSet};

pub fn create_hashmap(word: &str) -> HashMap<char, i32> {
    let mut char_map: HashMap<char, i32> = HashMap::new();
    for char in word.chars() {
        *char_map.entry(char).or_insert(0) += 1;
    }
    char_map
}

pub fn compare_string_with_hashmap(map_1: HashMap<char, i32>, string: &str) -> bool {
    let mut char_map_copy: HashMap<char, i32> = map_1.clone();
    string.chars().for_each(|charcter: char| {
        *char_map_copy.entry(charcter).or_insert(0) -= 1;
    });

    char_map_copy
        .into_iter()
        .fold(true, |acc, (_key, value)| acc && value == 0)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let char_map: HashMap<char, i32> = create_hashmap(&word.to_lowercase());
    let result = possible_anagrams
        .iter()
        .copied()
        .filter(|anagram_word: &&str| -> bool {
            if anagram_word.to_lowercase() == word.to_lowercase() {
                false
            } else {
                compare_string_with_hashmap(char_map.to_owned(), &anagram_word.to_lowercase())
            }
        });
    return HashSet::from_iter(result);
}

fn main() {
    print!(
        "{:#?}",
        anagrams_for("Orchestra", &["cashregister", "Carthorse", "radishes"])
    );
}
