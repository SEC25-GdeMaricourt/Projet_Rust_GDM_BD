use std::io::{self, Write};
use crate::{classes::Player, turns_helper::play_round};

/// Runs the game by initializing players and managing the rounds and turns.
pub fn run_game(p1: Player, p2: Player, goal_count: u8) {
    println!("Players have been initialized:");
    println!("{}: {:?}", p1.name, p1);
    println!("{}: {:?}", p2.name, p2);

    let mut turns: bool = true;
    let mut rounds: bool = true;

    // Continue running rounds until the game ends.
    while rounds {
        // Run individual turns within the current round.
        while turns {
            // Clone the players for each turn to preserve their original state.
            play_round(&mut p1.clone(), &mut p2.clone(), goal_count);
            turns = re_run_turn();
        }
        rounds = re_run_round(&p1, &p2);
    }
}

/// Prompts the user to decide whether to replay the current turn.
/// Returns `true` if the user chooses to replay, `false` otherwise.
fn re_run_turn() -> bool {
    print!("Replay the current turn? [Y/N]: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().eq_ignore_ascii_case("y")
}

/// Prompts the user to decide whether to start a new round.
/// Automatically ends the game if any player has zero vitality.
/// Returns `true` to start a new round, `false` to end the game.
fn re_run_round(p1: &Player, p2: &Player) -> bool {
    print!("Start a new round? [Y/N]: ");
    io::stdout().flush().unwrap();
    let mut new_game = String::new();
    io::stdin().read_line(&mut new_game).unwrap();

    if p1.vitality == 0 || p2.vitality == 0 {
        // If a player has run out of vitality, end the game.
        println!("A player has no vitality left. Game over! Sorry, but that's it!");
        false
    } else if new_game.trim().eq_ignore_ascii_case("y") {
        // User opted to start a new round.
        true
    } else {
        // User chose to stop playing.
        println!("Thanks for playing!");
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::io::BufRead;

    // Helper function to simulate the logic of `re_run_turn` by reading from a provided input.
    fn simulate_re_run_turn(input: &str) -> bool {
        let mut cursor = Cursor::new(input);
        let mut line = String::new();
        cursor.read_line(&mut line).unwrap();
        line.trim().eq_ignore_ascii_case("y")
    }

    #[test]
    fn test_re_run_turn_yes() {
        // Simulate user input "y"
        assert!(simulate_re_run_turn("y\n"));
    }

    #[test]
    fn test_re_run_turn_no() {
        // Simulate user input "n"
        assert!(!simulate_re_run_turn("n\n"));
    }

    // Helper function to simulate the logic of `re_run_round` by reading from provided input.
    // Note that if either player has zero vitality, the function should return false regardless of input.
    fn simulate_re_run_round(input: &str, p1: &Player, p2: &Player) -> bool {
        let mut cursor = Cursor::new(input);
        let mut line = String::new();
        cursor.read_line(&mut line).unwrap();
        if p1.vitality == 0 || p2.vitality == 0 {
            false
        } else {
            line.trim().eq_ignore_ascii_case("y")
        }
    }

    #[test]
    fn test_re_run_round_yes() {
        let p1 = Player::new("Alice".to_string(), 50, 50, 50);
        let p2 = Player::new("Bob".to_string(), 50, 50, 50);
        // Simulate user input "y" when both players have vitality.
        assert!(simulate_re_run_round("y\n", &p1, &p2));
    }

    #[test]
    fn test_re_run_round_no() {
        let p1 = Player::new("Alice".to_string(), 50, 50, 50);
        let p2 = Player::new("Bob".to_string(), 50, 50, 50);
        // Simulate user input "n".
        assert!(!simulate_re_run_round("n\n", &p1, &p2));
    }

    #[test]
    fn test_re_run_round_game_over() {
        let p1 = Player::new("Alice".to_string(), 0, 50, 50); // p1 has zero vitality
        let p2 = Player::new("Bob".to_string(), 50, 50, 50);
        // Even if user input is "y", the function should return false because a player's vitality is 0.
        assert!(!simulate_re_run_round("y\n", &p1, &p2));
    }

    // Note: Testing `run_game` is challenging due to its interactive nature and use of cloning.
    // Proper testing would require refactoring the function to allow dependency injection of input/output streams.
    // Therefore, `run_game` is not covered by these unit tests.
}
