pub fn factorial(n: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 1..=n {
        // Use saturating multiplication to stop at the maximum value of u32
        // rather than overflowing and wrapping around
        result = result.saturating_mul(i);
    }
    result
}

pub fn to_32bit_binary_loop(num: u32) -> String {
    let mut binary_string = String::with_capacity(32);

    // Loop backwards from bit index 31 down to 0
    for i in (0..32).rev() {
        let bit = (num >> i) & 1;
        binary_string.push(if bit == 1 { '1' } else { '0' });
    }

    binary_string
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        assert_eq!(factorial(20), u32::MAX);
    }

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
