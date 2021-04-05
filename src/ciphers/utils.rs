/// Cleans up the input by removing all characters that are not alphanumeric
/// (Returns an uppercase String)
pub fn clean_input(input: &str) -> String {
    let cleaned = input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>();

    cleaned
        .split_whitespace()
        .collect::<String>()
        .to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_input() {
        let input = String::from("awesome_testing_functionality with spaces");
        let expected = String::from("AWESOMETESTINGFUNCTIONALITYWITHSPACES");
        assert_eq!(expected, clean_input(&input));

        let input = String::from("NoW@wITHéé˛Ånumb3rz00712");
        let expected = String::from("NOWWITHNUMB3RZ00712");
        assert_eq!(expected, clean_input(&input));

        assert_eq!("".to_string(), clean_input(""));
    }
}
