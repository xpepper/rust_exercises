pub(crate) fn pig_latin(original_word: &str) -> String {
    if original_word.is_empty() {
        return String::new();
    }

    let first_char = original_word.chars().next().unwrap();
    if !first_char.is_alphabetic() {
        return original_word.to_string();
    }

    if is_vowel(&first_char) {
        format!("{original_word}-hay")
    } else {
        format!("{}-{first_char}ay", &original_word[1..])
    }
}

fn is_vowel(char: &char) -> bool {
    matches!(char.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
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

    #[test]
    fn words_starting_with_uppercase_vowel_have_hay_added_at_the_end() {
        assert_eq!(pig_latin("Apple"), "Apple-hay");
        assert_eq!(pig_latin("Elephant"), "Elephant-hay");
    }

    #[test]
    fn words_starting_with_uppercase_consonant_have_that_char_moved_to_the_end_and_ay_added() {
        assert_eq!(pig_latin("First"), "irst-Fay");
        assert_eq!(pig_latin("Bob"), "ob-Bay");
    }

    #[test]
    fn words_starting_with_non_alphabetic_characters_are_unchanged() {
        assert_eq!(pig_latin("123abc"), "123abc");
        assert_eq!(pig_latin("!hello"), "!hello");
        assert_eq!(pig_latin("@rust"), "@rust");
    }
}
