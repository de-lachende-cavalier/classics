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

// XXX this is actually pretty useless, may have to delete it
/// Pad a string until it's length is a multiple of n (the closest multiple
/// of n with respect to the string's initial length)
pub fn pad(data: &str, n: u32) -> String {
    let mut ret = String::from(data);

    // usually n < data.len() so this while will be pretty fast
    while ret.len() % (n as usize) != 0 {
        ret.push('.');
    }

    ret
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

    #[test]
    fn test_pad() {
        let input = String::from("testing");
        let n = 4;
        let expected = String::from("testing.");
        assert_eq!(expected, pad(&input, n));

        let input = String::from("lololo");
        let n = 3;
        assert_eq!(input, pad(&input, n));

        let input = String::from("te");
        let n = 12;
        let expected = String::from("te..........");
        assert_eq!(expected, pad(&input, n));
    }
}
