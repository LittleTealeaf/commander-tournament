pub mod game_record;
pub mod leaderboard;
pub mod ranking;

#[derive(Debug)]
pub struct State {
    leaderboard: leaderboard::State
}


#[derive(Debug, Clone)]
pub enum Message {
    Leaderboard(leaderboard::Message)
}
