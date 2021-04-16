/// Pad a string until its length is a multiple of n (the closest multiple
/// of n with respect to the string's initial length)
///
/// If n > data.len() then data will be padded until it's of length n.
pub fn pad(data: &str, n: u32, pad_char: char) -> String {
    let mut ret = String::from(data);

    // usually n < data.len() so this while should be pretty fast
    while ret.len() % (n as usize) != 0 {
        ret.push(pad_char);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad() {
        let input = String::from("testing");
        let n = 4;
        let expected = String::from("testing.");
        assert_eq!(expected, pad(&input, n, '.'));

        let input = String::from("lololo");
        let n = 3;
        assert_eq!(input, pad(&input, n, ')'));

        let input = String::from("te");
        let n = 12;
        let expected = String::from("teXXXXXXXXXX");
        assert_eq!(expected, pad(&input, n, 'X'));
    }
}
