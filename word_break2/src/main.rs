use std::collections::HashMap;

/// Word Break II
/// https://leetcode.com/problems/word-break-ii/description/
pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    // Map to store results of subproblems
    let mut dp: HashMap<usize, Vec<String>> = HashMap::new();

    // Iterate from the end of the string to the beginning
    for start_idx in (0..s.len()).rev() {
        // List to store valid sentences starting from start_idx
        let mut valid_sentences: Vec<String> = vec![];

        // Iterate from start_idx to the end of the string
        for end_idx in start_idx..s.len() {
            // Extract substring from start_idx to end_idx
            let current_word: String = s[start_idx..end_idx+1].to_string();

            // Check if the current substring is a valid word
            if word_dict.contains(&current_word) {
                // If it's the last word, add it as a valid sentence
                if end_idx == s.len() - 1 {
                    valid_sentences.push(current_word);
                } else {
                    // If it's not the last word, append it to each sentence formed by the remaining substring
                    for sentence in dp.get(&(end_idx+1)).unwrap() {
                        valid_sentences.push(format!("{} {}", current_word, sentence));
                    }
                }
            }
        }
        // Store the valid sentences in dp
        dp.insert(start_idx, valid_sentences);
    }
    // Return the sentences formed from the entire string
    return dp.get(&0).unwrap().to_vec();
}

pub fn main() {
    let s: String = "pineapplepenapple".to_string();
    let word_dict: Vec<String> = vec![
        "apple".to_string(),
        "pen".to_string(),
        "applepen".to_string(),
        "pine".to_string(),
        "pineapple".to_string()
    ];
    println!("Test Case 1: {:?}", word_break(s, word_dict));

    let s: String = "a".to_string();
    let word_dict: Vec<String> = vec![
        "b".to_string()
    ];
    println!("Test Case 2: {:?}", word_break(s, word_dict));

    let s: String = "bb".to_string();
    let word_dict: Vec<String> = vec![
        "a".to_string(),
        "b".to_string(),
        "bbb".to_string(),
        "bbbb".to_string()
    ];
    println!("Test Case 3: {:?}", word_break(s, word_dict));
    
    let s: String = "catsanddog".to_string();
    let word_dict: Vec<String> = vec![
        "cat".to_string(),
        "cats".to_string(),
        "and".to_string(),
        "sand".to_string(),
        "dog".to_string()
    ];
    println!("Test Case 4: {:?}", word_break(s, word_dict));

    let s: String = "aaaaaaa".to_string();
    let word_dict: Vec<String> = vec![
        "aaaa".to_string(),
        "aaa".to_string()
    ];
    println!("Test Case 5: {:?}", word_break(s, word_dict));
}
