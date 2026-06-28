use auto_const_array::auto_const_array;

#[derive(Debug, Copy, Clone, PartialEq, Eq, derive_more::Display, Default)]
pub enum NextPlayerMode {
    #[display("Least Games")]
    LeastGames,
    #[display("Longest Break")]
    #[default]
    LongestBreak,
    #[display("Longest Lead Break")]
    LongestLeadBreak,
    #[display("Least Wins")]
    LeastWins,
    #[display("Outlier Winrate")]
    OutlierWinrate,
    #[display("Longest Since Win")]
    LongestSinceWin,
    #[display("Closest to Peak")]
    PeakElo,
}

impl NextPlayerMode {
    auto_const_array! {
        pub const VALUES: [Self; _] = [
            Self::LongestBreak,
            Self::LongestLeadBreak,
            Self::LeastGames,
            Self::LeastWins,
            Self::OutlierWinrate,
            Self::LongestSinceWin,
            Self::PeakElo
        ]
    }
}
