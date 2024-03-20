extern crate triword;

use triword::{get_word_list, solve, SolveResult};

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_solve_with_valid_input() {
        let word_list = get_word_list();

        let result1 = solve("TUN/GON/IN", 3, &word_list);
        assert_eq!(result1.solution, "DRA");
        assert_eq!(result1.words_used, ["TUNDRA", "DRAGON", "DRAIN"]);

        // empty space is valid
        let result2 = solve("RLY / LIN / MIL", 4, &word_list);
        assert_eq!(result2.solution, "EAGE");
        assert_eq!(result2.words_used, ["EAGERLY", "LINEAGE", "MILEAGE"]);
    }

    #[test]
    fn test_solve_with_invalid_input() {
        let word_list = get_word_list();

        let result1: SolveResult = solve("", 3, &word_list);
        assert_eq!(result1.solution, "");
        assert!(result1.words_used.is_empty());

        let result2 = solve("INVALID_INPUT", 3, &word_list);
        assert_eq!(result2.solution, "");
        assert!(result2.words_used.is_empty());
    }
}
