pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[cfg_attr(not(rust_analyzer), cfg(kani))]
mod proofs {
    use super::*;

    #[cfg_attr(not(rust_analyzer), kani::proof)]
    fn verify_add() {
        let a: u64 = kani::any();
        let b: u64 = kani::any();
        let result = add(a, b);

        // Assert that the result does not overflow
        assert!(result >= a);
        assert!(result >= b);
    }

    #[test]
    fn kani_concrete_playback_verify_add_7155943916565760311() {
        let concrete_vals: Vec<Vec<u8>> = vec![
            // 13835058055282163713ul
            vec![1, 0, 0, 0, 0, 0, 0, 192],
            // 9223372036854775804ul
            vec![252, 255, 255, 255, 255, 255, 255, 127],
        ];
        kani::concrete_playback_run(concrete_vals, verify_add);
    }
}
