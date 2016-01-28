use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn is_vowel(c: char) -> bool {
    return "aeiou".chars().any(|x| c == x);
}

fn number_of_vowels(word: &str) -> usize {
    return word.chars().filter(|c| is_vowel(*c)).count();
}

fn vowels_condition(word: &str) -> bool {
    return number_of_vowels(word) >= 3;
}

fn double_letter_condition(word: &str) -> bool {
    let mut double_letter = false;
    let _ = word.chars().fold('1', |acc, c| {
        if acc == c {
            double_letter = true;
        }
        c
    });
    return double_letter;
}

fn no_bad_strings_condition(word: &str) -> bool {
    let bad_strings = vec!["ab", "cd", "pq", "xy"];

    for bad_string in bad_strings.iter() {
        let found = match word.find(bad_string) {
            Some(x) => true,
            None => false,
        };
        if found {
            return false;
        }
    }
    return true;
}

fn nice_word(word: &str) -> bool {
    vowels_condition(word) && double_letter_condition(word) && no_bad_strings_condition(word)
}


fn has_two_pairs(word: &str) -> bool {

    let vec_chars: Vec<char> = word.chars().collect();
    let len = vec_chars.len();

    for (i, c) in vec_chars.iter().take(len - 1).enumerate() {
        let mut string = String::new();
        string.push(*c);
        string.push(vec_chars[i + 1]);

        if word[i + 2..].contains(&string) {
            return true;
        }
    }

    return false;
}

fn has_alternate(word: &str) -> bool {
    let sliced = word.as_bytes();

    for win in sliced.windows(3) {
        let x = win.iter().nth(0).unwrap();
        let y = win.iter().nth(2).unwrap();

        if x == y {
            return true;
        }
    }

    return false;
}

fn nice_word_v2(word: &str) -> bool {
    return has_two_pairs(word) && has_alternate(word);
}


pub fn solve() {
    let path = Path::new("input-5.txt");

    let mut file = match File::open(&path) {
        Err(why) => panic!("could not open file because: {}", Error::description(&why)),
        Ok(file) => file,
    };

    let mut total_nice = 0;
    let mut total_nice_v2 = 0;

    let file = BufReader::new(file);

    let mut accum: i32 = 0;
    let mut bow_accum: i32 = 0;

    for line in file.lines() {
        match line {
            Err(why) => panic!("could not read line because: {}", Error::description(&why)),
            Ok(word) => {
                if nice_word(&word) {
                    total_nice += 1;
                }
                if nice_word_v2(&word) {
                    total_nice_v2 += 1;
                }
            }
        }
    }

    println!("Total number of nice words: {}", total_nice);
    println!("Total number of nice words, second version: {}",
             total_nice_v2);

}


#[test]
fn test_five() {
    assert_eq!(is_vowel('a'), true);
    assert_eq!(is_vowel('b'), false);

    assert_eq!(number_of_vowels("aaa"), 3);
    assert_eq!(number_of_vowels("aba"), 2);

    assert_eq!(vowels_condition("ugknbfddgicrmopn"), true);
    assert_eq!(vowels_condition("dvszwmarrgswjxmb"), false);

    assert_eq!(double_letter_condition("aaa"), true);
    assert_eq!(double_letter_condition("jchzalrnumimnmhp"), false);
    assert_eq!(double_letter_condition("haegwjzuvuyypxyu"), true);

    assert_eq!(no_bad_strings_condition("aaa"), true);
    assert_eq!(no_bad_strings_condition("haegwjzuvuyypxyu"), false);

    assert_eq!(nice_word("aaa"), true);
    assert_eq!(nice_word("jchzalrnumimnmhp"), false);
    assert_eq!(nice_word("haegwjzuvuyypxyu"), false);
    assert_eq!(nice_word("dvszwmarrgswjxmb"), false);

    assert_eq!(has_two_pairs("xyxy"), true);
    assert_eq!(has_two_pairs("aabcdefgaa"), true);
    assert_eq!(has_two_pairs("aaa"), false);

    assert_eq!(has_alternate("xyx"), true);
    assert_eq!(has_alternate("abcdefeghi"), true);
    assert_eq!(has_alternate("aaa"), true);
    assert_eq!(has_alternate("bbca"), false);
}
