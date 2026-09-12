#[derive(Debug, Copy, Clone, PartialEq, Eq, derive_more::Display, Default, strum::VariantArray)]
pub enum NextPlayerMode {
    #[display("Longest Break")]
    #[default]
    LongestBreak,
    #[display("Least Played")]
    LeastPlayed,
    #[display("Win Streak")]
    WinStreak,
    #[display("Loss Streak")]
    LossStreak,
    #[display("Game Streak")]
    GameStreak,
}
