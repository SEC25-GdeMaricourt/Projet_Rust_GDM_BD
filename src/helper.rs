use crate::classes::PlayerRes;
use rand::Rng;

/// Generates a list of random objectives.
/// Each objective is a random number between 0 and 100.
///
/// # Arguments
///
/// * `count` - The number of objectives to generate.
///
/// # Returns
///
/// A vector containing `count` random numbers.
pub fn create_goals(count: u8) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..count).map(|_| rng.gen_range(0..=100)).collect()
}

/// Computes the score for a given objective.
///
/// # Arguments
///
/// * `goal` - The target value for the objective.
/// * `res` - The result for this objective (includes the counter value and miss count).
/// * `strength` - The player's strength attribute.
///
/// # Returns
///
/// The computed score for the objective.
pub fn compute_score(goal: u8, res: &PlayerRes, strength: u32) -> u32 {
    // Calculate the absolute difference between the target and the current counter.
    let raw_diff = if goal >= res.counter {
        goal - res.counter
    } else {
        res.counter - goal
    };

    // Calculate the circular difference to account for wrap-around when the difference is large.
    let circ_diff = if raw_diff > 50 {
        100 - raw_diff
    } else {
        raw_diff
    };

    // Determine the base score according to the circular difference.
    let base = if circ_diff == 0 {
        100
    } else if circ_diff <= 5 {
        80
    } else if circ_diff <= 10 {
        60
    } else if circ_diff <= 20 {
        40
    } else {
        20
    };

    // Compute and return the final score,
    // taking into account the player's strength and adjusting for any misses.
    (base as u32 + strength) / (res.miss + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_goals_zero() {
        // When count is zero, the resulting vector should be empty.
        let goals = create_goals(0);
        assert!(goals.is_empty());
    }

    #[test]
    fn test_create_goals_count() {
        // When a positive count is given, the vector should have that many elements,
        // and every element must be between 0 and 100 (inclusive).
        let count = 10;
        let goals = create_goals(count);
        assert_eq!(goals.len(), count as usize);
        for goal in goals {
            assert!(goal <= 100, "Goal {} is greater than 100", goal);
        }
    }

    #[test]
    fn test_compute_score_perfect_match() {
        // When the goal equals the counter, raw_diff is 0 so base = 100.
        // With no misses, score = (100 + strength) / 1.
        let strength = 20;
        let res = PlayerRes { counter: 50, miss: 0 };
        let goal = 50;
        let expected = (100 + strength) / 1; // 120
        let score = compute_score(goal, &res, strength);
        assert_eq!(score, expected);
    }

    #[test]
    fn test_compute_score_small_difference() {
        // When the difference is small and non-zero (<= 5), base should be 80.
        // For example, goal = 55 and counter = 50 gives raw_diff = 5 (and circ_diff = 5).
        let strength = 10;
        let res = PlayerRes { counter: 50, miss: 0 };
        let goal = 55;
        let expected = (80 + strength) / 1; // 90
        let score = compute_score(goal, &res, strength);
        assert_eq!(score, expected);
    }

    #[test]
    fn test_compute_score_circular_difference() {
        // When the raw difference is large, the circular difference applies.
        // For example, goal = 10 and counter = 90 gives raw_diff = 80,
        // so circ_diff = 100 - 80 = 20, thus base should be 40.
        let strength = 15;
        let res = PlayerRes { counter: 90, miss: 0 };
        let goal = 10;
        let expected = (40 + strength) / 1; // 55
        let score = compute_score(goal, &res, strength);
        assert_eq!(score, expected);
    }

    #[test]
    fn test_compute_score_with_misses() {
        // When there are misses, the final score should be divided by (miss + 1).
        // For example, with goal = 50, counter = 50 (perfect match) and miss = 2:
        // score = (100 + strength) / (2 + 1)
        let strength = 30;
        let res = PlayerRes { counter: 50, miss: 2 };
        let goal = 50;
        let expected = (100 + strength) / 3; // (130 / 3) = 43 (integer division)
        let score = compute_score(goal, &res, strength);
        assert_eq!(score, expected);
    }
}
