
pub fn is_power_of_2(n: usize) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

pub fn find_next_power_of_2(n: usize) -> usize {
    match n {
        0 => 1 ,
        _ => {
            let mut power_of_2 = 1;
            loop {
                if n <= power_of_2 {
                    return power_of_2;
                }
                power_of_2 *= 2;
            }
        }
    }
}



#[cfg(test)]
mod test {
    #[test]    
    fn test_is_power_of_2() {
        use std::collections::HashSet;
        use util::find_next_power_of_2;
        use util::is_power_of_2;
        

        assert!(!is_power_of_2(0));
        assert!(is_power_of_2(1));
        assert!(is_power_of_2(2));
        assert!(!is_power_of_2(3));
        assert!(is_power_of_2(4));
        assert!(!is_power_of_2(5));
        
        let mut n = 1;
        let mut powers_of_2 = HashSet::new();
        for _ in 1 .. 21 {
            powers_of_2.insert(n);
            assert!(is_power_of_2(n));
            n *= 2;
        }

        for i in 1 .. 1_000_000 {
            if is_power_of_2(i) {
                eprintln!("{}", i);
                assert!(powers_of_2.contains(&i))
            }
        }
    }

    fn test_find_next_highest_power_of_2() {
        use util::find_next_power_of_2;
        assert_eq!(find_next_power_of_2(1), 1);
        assert_eq!(find_next_power_of_2(1), 1);

    }
}