use std::time::Duration;
use std::thread;
use std::io::{self, Write};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, AtomicU32, Ordering},};

use crate::classes::{Player, PlayerRes};
use crate::helper::{compute_score, create_goals};

/// Executes a complete round between two players using helper functions.
pub fn play_round(p1: &mut Player, p2: &mut Player, goal_count: u8) {
    println!("----- New Round -----");

    // Execute each player's turn and calculate their average score.
    let avg_score1 = play_turn(p1, goal_count);
    let avg_score2 = play_turn(p2, goal_count);

    // Compare average scores and update vitality accordingly.
    if avg_score1 == avg_score2 {
        println!("The round is tied; no change in vitality.");
    } else if avg_score1 > avg_score2 {
        let diff = avg_score1 - avg_score2;
        println!("{} wins the round!", p1.name);
        println!("{} loses {} vitality points.", p2.name, diff);
        p2.vitality = p2.vitality.saturating_sub(diff);
        apply_pusnishment(p2);
    } else {
        let diff = avg_score2 - avg_score1;
        println!("{} wins the round!", p2.name);
        println!("{} loses {} vitality points.", p1.name, diff);
        p1.vitality = p1.vitality.saturating_sub(diff);
        apply_pusnishment(p1);
    }

    // Display final player statistics at the end of the round.
    println!("\n--- End of Round ---");
    println!(
        "{}: Vitality: {}, Speed: {}, Strength: {}",
        p1.name, p1.vitality, p1.speed, p1.strength
    );
    println!(
        "{}: Vitality: {}, Speed: {}, Strength: {}",
        p2.name, p2.vitality, p2.speed, p2.strength
    );
}

/// Runs a single turn for a player by generating goals, executing the objective for each goal,
/// computing scores, and then calculating the average score for the turn.
fn play_turn(player: &mut Player, goal_count: u8) -> u32 {
    println!(
        "\nIt's {}'s turn (Vitality: {}, Speed: {}, Strength: {})",
        player.name, player.vitality, player.speed, player.strength
    );
    let goals = create_goals(goal_count);
    println!("Goals: {:?}", goals);
    let total_score: u32 = goals
        .iter()
        .enumerate()
        .map(|(i, target)| {
            println!(
                "\n--- {} - Objective {}: Target {} ---",
                player.name,
                i + 1,
                target
            );
            // Run the objective turn for the current goal.
            let result = run_one_turn(player.speed);
            let score = compute_score(*target, &result, player.strength);
            println!("Score for this objective: {}", score);
            score
        })
        .sum();
    let avg_score = if goal_count == 0 {
        0
    } else {
        (total_score as f32 / goal_count as f32).ceil() as u32
    };
    println!("{}'s average score: {}", player.name, avg_score);
    avg_score
}

//
// Interactive functions
//

// In production, these functions interact with the user via stdin/out.
// For tests, we override them with dummy versions.

#[cfg(not(test))]
fn run_one_turn(speed: u32) -> PlayerRes {
    // Wait for the user to initiate the objective.
    println!("Press ENTER to start this objective...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Shared variables for the counter and miss count between threads.
    let counter = Arc::new(Mutex::new(0u8));
    let miss = Arc::new(AtomicU32::new(0));
    let stop_flag = Arc::new(AtomicBool::new(false));

    // Clone the shared references for the counting thread.
    let counter_thread = Arc::clone(&counter);
    let miss_thread = Arc::clone(&miss);
    let stop_flag_thread = Arc::clone(&stop_flag);

    // Spawn a thread that increments the counter continuously.
    let handle = thread::spawn(move || {
        while !stop_flag_thread.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(speed as u64));
            let mut count = counter_thread.lock().unwrap();
            *count += 1;
            // Reset the counter and record a miss if the counter exceeds 100.
            if *count > 100 {
                *count = 0;
                miss_thread.fetch_add(1, Ordering::Relaxed);
            }
            // Display the current counter value and miss count.
            print!(
                "\rCounter: {} | Misses: {}",
                *count,
                miss_thread.load(Ordering::Relaxed)
            );
            io::stdout().flush().unwrap();
        }
    });

    // Wait for the user to signal stopping the counter.
    println!("\nPress ENTER to stop the counter...");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    // Signal the counting thread to stop and wait for it to finish.
    stop_flag.store(true, Ordering::Relaxed);
    handle.join().unwrap();

    // Retrieve the final counter value and miss count.
    let final_counter = *counter.lock().unwrap();
    let final_miss = miss.load(Ordering::Relaxed);

    println!("\nObjective stopped at: {} (Misses: {})", final_counter, final_miss);
    PlayerRes {
        counter: final_counter,
        miss: final_miss,
    }
}

#[cfg(test)]
fn run_one_turn(_speed: u32) -> PlayerRes {
    // Dummy implementation for tests.
    PlayerRes {
        counter: 50,
        miss: 0,
    }
}

#[cfg(not(test))]
fn apply_pusnishment(victim: &mut Player) {
    // Prompt the user to choose a penalty for the opponent.
    println!("Choose a penalty to apply to {}:", victim.name);
    println!("1: -5 to speed");
    println!("2: -5 to strength");
    print!("Your choice (1 or 2): ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    match choice.trim() {
        "1" => {
            victim.speed = victim.speed.saturating_sub(5);
            println!("{} loses 5 speed.", victim.name);
        }
        "2" => {
            victim.strength = victim.strength.saturating_sub(5);
            println!("{} loses 5 strength.", victim.name);
        }
        _ => println!("Invalid choice. No penalty applied."),
    }
}

#[cfg(test)]
fn apply_pusnishment(_victim: &mut Player) {
    // Dummy version for tests: do nothing.
}

//
// Unit tests
//
#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::Player;

    #[test]
    fn test_play_turn_zero_goals() {
        // With 0 goals, the average score should be 0.
        let mut player = Player::new("TestPlayer".to_string(), 50, 50, 10);
        let avg = play_turn(&mut player, 0);
        assert_eq!(avg, 0);
    }

    #[test]
    fn test_run_one_turn_dummy() {
        // In test mode, run_one_turn should return the dummy value.
        let res = run_one_turn(50);
        assert_eq!(res.counter, 50);
        assert_eq!(res.miss, 0);
    }

    #[test]
    fn test_apply_pusnishment_dummy() {
        // The dummy version of apply_pusnishment should not change the player's attributes.
        let mut player = Player::new("TestPlayer".to_string(), 50, 50, 10);
        let orig_speed = player.speed;
        let orig_strength = player.strength;
        apply_pusnishment(&mut player);
        assert_eq!(player.speed, orig_speed);
        assert_eq!(player.strength, orig_strength);
    }

    #[test]
    fn test_play_round_tie() {
        // Create two players with identical attributes so that they tie.
        let mut player1 = Player::new("Alice".to_string(), 50, 50, 10);
        let mut player2 = Player::new("Bob".to_string(), 50, 50, 10);
        play_round(&mut player1, &mut player2, 3);
        // In a tie, neither player's vitality should change.
        assert_eq!(player1.vitality, 50);
        assert_eq!(player2.vitality, 50);
    }

    #[test]
    fn test_play_round_non_tie() {
        // Create two players with different strengths to force a non-tie outcome.
        // With the dummy run_one_turn, compute_score returns (100 + strength) for a perfect match.
        // For player1 with strength 20, score = 120.
        // For player2 with strength 10, score = 110.
        // Thus, player2 should lose (120 - 110) = 10 vitality.
        let mut player1 = Player::new("Alice".to_string(), 50, 50, 20);
        let mut player2 = Player::new("Bob".to_string(), 50, 50, 10);
        play_round(&mut player1, &mut player2, 3);
        assert_eq!(player1.vitality, 50);
        assert_eq!(player2.vitality, 40);
    }
}
