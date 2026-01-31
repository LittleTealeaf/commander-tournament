use std::collections::{HashMap, HashSet};



struct Game {
    players: HashMap<String, f64>,
    winner: String,
}

impl Game {
    pub fn has_player(&self, player: &String) -> bool {
        self.players.contains_key(player)
    }
}


pub struct Torunament {
    players: Vec<String>,
    games: Vec<Game>
}

impl Torunament {

    pub fn register_player(&mut self, player: String) {
        self.players.push(player);
    }

    pub fn get_winrate(&self, player: &String) -> Option<f64> {
        if !self.players.contains(player) {
            return None
        }
        let ut co
        for game in games {

        }
    }

    pub fn import_game(&mut self, players: Vec<String>, winner: String) -> Result<(), TournamentError> {
        if !players.contains(&winner) {
            return Err(TournamentError::WinnerIsNotPlayer)
        }





        Ok(())
    }
}


pub enum TournamentError {
    WinnerIsNotPlayer
}
