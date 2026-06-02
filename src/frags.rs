use {
    crate::ship::Ship,
    chrono::{DateTime, Utc},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    You,
    Player(String),
}

#[derive(Debug, Clone)]
pub struct Frag {
    pub timestamp: DateTime<Utc>,
    pub killer: Target,
    pub victim: Target,
    pub star_system: Option<String>,
    pub ship: Option<Ship>,
}

impl Frag {
    pub fn is_kill(&self) -> bool {
        self.victim != Target::You
    }

    pub fn death(
        timestamp: DateTime<Utc>,
        star_system: Option<String>,
        ship: Option<Ship>,
        player: String,
    ) -> Frag {
        Frag {
            timestamp,
            star_system,
            ship,
            killer: Target::Player(player),
            victim: Target::You,
        }
    }

    pub fn kill(
        timestamp: DateTime<Utc>,
        star_system: Option<String>,
        ship: Option<Ship>,
        player: String,
    ) -> Frag {
        Frag {
            timestamp,
            star_system,
            ship,
            killer: Target::You,
            victim: Target::Player(player),
        }
    }
}
