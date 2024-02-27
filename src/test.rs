mod solver;

use solver::{solve, get_word_list};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_with_valid_input() {
        let word_list = get_word_list();
        let result1 = solve("TUN/GON/IN", 3, &word_list);
        let result2 = solve("RLY/LIN/MIL", 4, &word_list);

        assert_eq!(result1, "dra");
        assert_eq!(result2, "eage");
    }

    #[test]
    fn test_solve_with_invalid_input() {
        let word_list = get_word_list();
        let result1 = solve("", 3, &word_list);
        let result2 = solve("INVALID_INPUT", 3, &word_list);

        assert_eq!(result1, "");
        assert_eq!(result2, "");
    }
}
