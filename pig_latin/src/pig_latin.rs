pub(crate) fn pig_latin(original_word: &str) -> String {
    if original_word.is_empty() {
        return String::new();
    }

    let first_char = original_word.chars().next().unwrap();

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    if vowels.contains(&first_char) {
        format!("{original_word}-hay")
    } else {
        let string: String = original_word.chars().skip(1).collect();
        format!("{string}-{first_char}ay")
    }
}

#[cfg(test)]
mod tests {
    use super::pig_latin;

    #[test]
    fn does_not_apply_on_empty_strings() {
        assert_eq!(pig_latin(""), "");
    }

    #[test]
    fn words_starting_with_a_vowel_have_hay_added_at_the_end() {
        assert_eq!(pig_latin("apple"), "apple-hay");
        assert_eq!(pig_latin("elephant"), "elephant-hay");
        assert_eq!(pig_latin("iguana"), "iguana-hay");
        assert_eq!(pig_latin("orange"), "orange-hay");
        assert_eq!(pig_latin("umbrella"), "umbrella-hay");
    }

    #[test]
    fn words_starting_with_a_consonant_have_that_char_moved_to_the_end_of_the_word_and_ay_is_added()
    {
        assert_eq!(pig_latin("first"), "irst-fay");
        assert_eq!(pig_latin("second"), "econd-say");
        assert_eq!(pig_latin("third"), "hird-tay");
        assert_eq!(pig_latin("fourth"), "ourth-fay");
        assert_eq!(pig_latin("bob"), "ob-bay");
        assert_eq!(pig_latin("jim"), "im-jay");
    }
}
