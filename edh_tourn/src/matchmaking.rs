use crate::player::PlayerId;

#[derive(Debug, Clone, Copy)]
pub enum MatchmakingPlayer {
    Player(PlayerId),
    LongestBreak,
    LeastPlayed,
}
