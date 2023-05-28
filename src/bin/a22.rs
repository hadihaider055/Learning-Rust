// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    } else {
        Some(a / b)
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_clamp() {
        let clamped_num = clamp(3000, 100, 2000);
        let expected = 2000;
        assert_eq!(
            clamped_num, expected,
            "The clamped number should be equal to 2000."
        );
    }

    #[test]
    fn make_div() {
        let division = div(6, 2);
        let answer = Some(3);
        assert_eq!(
            division, answer,
            "The answer should be the exact division result."
        );
    }

    #[test]
    fn concatted_name_fn() {
        let concatted_name = concat("Hadi", "Haider");
        let expected_concatted_name = String::from("HadiHaider");
        assert_eq!(
            concatted_name, expected_concatted_name,
            "The concatenated name should be the same."
        );
    }
}
