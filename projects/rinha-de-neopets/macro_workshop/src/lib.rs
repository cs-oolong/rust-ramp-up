pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

macro_rules! hardcoded_macro {
    () => {
        {
            let x = 4;
            x * x
        }
    };
}

macro_rules! square {
    ($expression:expr) => {
        {
            let value = $expression;
            value * value
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn squared_by_hand() {
        let x = 1 + 2;   // expr evaluated once
        let ans = x * x; // 9
        assert_eq!(ans, 9);
    }

    #[test]
    fn test_hardcoded_macro() {
        assert_eq!(hardcoded_macro!(), 16);
    }

    fn square_with_expressions() {
        assert_eq!(square!(1+2+3+4), 100);
        assert_eq!(square!(2*10), 400);
    }
}
