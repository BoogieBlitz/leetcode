use std::collections::HashMap;

/// Word Break II
/// https://leetcode.com/problems/word-break-ii/description/
pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut result_cache: Vec<String> = Vec::new();
    let mut words_processed: Vec<String> = Vec::new();
    let mut word_dict_cache: Vec<String> = word_dict.clone();

    loop {
        let mut input_str: String = s.clone();
        let mut temp_cache: Vec<String> = Vec::new();
        let mut word_index: HashMap<usize, &String> = HashMap::new();

        for word in word_dict_cache.iter() {
            let indices: Vec<usize> = input_str.match_indices(word).map(|(i, _)|i).collect();

            if indices.len() > 0 {
                for idx in indices {
                    word_index.insert(idx, word);
                }
                input_str = input_str.replace(word, "_".repeat(word.len()).as_str());
                //println!("{} - {:?} - {}", word, word_index, input_str);

                if !words_processed.contains(word) {
                    words_processed.push(word.to_owned());
                }
            } else {
                temp_cache.push(word.to_owned());
            }
        }

        let remaining_str: String = input_str.replace("_", "");
        if remaining_str.len() == 0 {
            let mut temp_vec: Vec<&str> = Vec::new();
            let mut keys: Vec<&usize> = word_index.keys().collect();
            keys.sort();
            for key in keys.iter() {
                temp_vec.push(word_index.get(key).unwrap().as_str());
            }
            let result_str: String = temp_vec.join(" ");
            if !result.contains(&result_str) {
                result.push(result_str);
            }
        }

        temp_cache.extend(words_processed.clone());
        word_dict_cache = temp_cache.clone();
        //println!("Result: {:?} | Result Cache: {:?}", result, result_cache);

        if (words_processed.len() == 0) | ((result.len() > 0) & (result == result_cache)) {
            break;
        }
        
        result_cache = result.clone();
        //break;
    }
    result
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
