/// Structure representing a player in the duel game.
#[derive(Debug, Clone)]
pub struct Player {
    /// The player's name.
    pub name: String,
    /// The player's vitality (health points).
    pub vitality: u32,
    /// The player's speed, used to determine the counter increment rate.
    pub speed: u32,
    /// The player's strength, which affects score calculation.
    pub strength: u32,
}

impl Player {
    /// Creates a new player with the specified attributes.
    pub fn new(name: String, vitality: u32, speed: u32, strength: u32) -> Self {
        Player {
            name,
            vitality,
            speed,
            strength,
        }
    }
}

impl Default for Player {
    /// Default values for a player:
    /// name: "Joueur", vitality: 50, speed: 50, strength: 50.
    fn default() -> Self {
        Player {
            name: String::from("Joueur"),
            vitality: 50,
            speed: 50,
            strength: 50,
        }
    }
}

/// Structure representing the result of stopping the counter during an objective.
pub struct PlayerRes {
    /// The counter value at the moment of stopping.
    pub counter: u8,
    /// The number of times the counter exceeded 100 and was reset (miss count).
    pub miss: u32,
}

impl Default for PlayerRes {
    /// Default values for PlayerRes:
    /// counter: 0, miss: 0.
    fn default() -> Self {
        PlayerRes {
            counter: 0,
            miss: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_player() {
        let name = "Alice".to_string();
        let vitality = 60;
        let speed = 40;
        let strength = 70;
        let player = Player::new(name.clone(), vitality, speed, strength);
        assert_eq!(player.name, name);
        assert_eq!(player.vitality, vitality);
        assert_eq!(player.speed, speed);
        assert_eq!(player.strength, strength);
    }

    #[test]
    fn test_default_player() {
        let player = Player::default();
        assert_eq!(player.name, "Joueur");
        assert_eq!(player.vitality, 50);
        assert_eq!(player.speed, 50);
        assert_eq!(player.strength, 50);
    }

    #[test]
    fn test_default_player_res() {
        let res = PlayerRes::default();
        assert_eq!(res.counter, 0);
        assert_eq!(res.miss, 0);
    }
}
