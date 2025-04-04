use clap::Parser;

mod classes;
mod helper;
mod turns_helper;
mod game_runner;

use classes::Player;
use game_runner::run_game;

/// Duel Game in Rust
#[derive(Parser, Debug)]
#[command(name = "Duel Game", about = "A duel game implemented in Rust", version = "0.1")]
struct Args {
    /// First player's name.
    #[arg(long, default_value = "Michel")]
    name1: String,
    /// Second player's name.
    #[arg(long, default_value = "Jacquie")]
    name2: String,
    /// Player vitality.
    #[arg(long, default_value_t = 50)]
    vitality: u32,
    /// Player speed (increment delay in ms).
    #[arg(long, default_value_t = 50)]
    speed: u32,
    /// Player strength.
    #[arg(long, default_value_t = 50)]
    strength: u32,
    /// Number of objectives per round.
    #[arg(long, default_value_t = 5)]
    goals: u8,
}

fn main() {
    // Initialize the logger.
    env_logger::init();

    // Parse command-line arguments.
    let args = Args::parse();

    // Create two players using the provided arguments.
    let p1 = Player::new(args.name1, args.vitality, args.speed, args.strength);
    let p2 = Player::new(args.name2, args.vitality, args.speed, args.strength);
    let goal_count = args.goals;

    // Start the game with the two players and the specified number of objectives per round.
    run_game(p1, p2, goal_count);
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_args_defaults() {
        // When no command-line arguments are provided, default values should be used.
        let args = Args::parse_from(&["test"]);
        assert_eq!(args.name1, "Michel");
        assert_eq!(args.name2, "Jacquie");
        assert_eq!(args.vitality, 50);
        assert_eq!(args.speed, 50);
        assert_eq!(args.strength, 50);
        assert_eq!(args.goals, 5);
    }

    #[test]
    fn test_args_custom_values() {
        // Provide custom command-line arguments and verify they are parsed correctly.
        let args = Args::parse_from(&[
            "test",
            "--name1", "Alice",
            "--name2", "Bob",
            "--vitality", "80",
            "--speed", "40",
            "--strength", "70",
            "--goals", "7",
        ]);
        assert_eq!(args.name1, "Alice");
        assert_eq!(args.name2, "Bob");
        assert_eq!(args.vitality, 80);
        assert_eq!(args.speed, 40);
        assert_eq!(args.strength, 70);
        assert_eq!(args.goals, 7);
    }
}
