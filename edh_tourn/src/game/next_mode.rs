#[derive(Debug, Copy, Clone, PartialEq, Eq, derive_more::Display, Default, strum::VariantArray)]
pub enum NextPlayerMode {
    #[display("Longest Break")]
    #[default]
    LongestBreak,
    #[display("Least Played")]
    LeastPlayed,


// #[display("Least Games")]
                  // LeastGames,
                  // #[display("Longest Break")]
                  // #[default]
                  // LongestBreak,
                  // #[display("Longest Lead Break")]
                  // LongestLeadBreak,
                  // #[display("Least Wins")]
                  // LeastWins,
                  // #[display("Outlier Winrate")]
                  // OutlierWinrate,
                  // #[display("Longest Since Win")]
                  // LongestSinceWin,
                  // #[display("Closest to Peak")]
                  // PeakElo,
                  // #[display("Winstreak")]
                  // Winstreak,
                  // #[display("Losing Streak")]
                  // LossStreak,
}
