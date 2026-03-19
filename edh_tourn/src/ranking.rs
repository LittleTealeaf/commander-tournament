use core::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RankingMethod {
    LeastPlayed,
    LostWith,
    Nemesis,
    EloNeighbors,
    WRNeighbors,
    ExpectedNeighbors,
    #[default]
    Combined,
}

impl RankingMethod {
    pub const VALUES: [Self; 7] = [
        Self::Combined,
        Self::LeastPlayed,
        Self::Nemesis,
        Self::LostWith,
        Self::EloNeighbors,
        Self::WRNeighbors,
        Self::ExpectedNeighbors,
    ];
}

impl Display for RankingMethod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::LeastPlayed => "Least Played",
            Self::LostWith => "Lost With",
            Self::Nemesis => "Nemesis",
            Self::EloNeighbors => "Elo Neighbors",
            Self::WRNeighbors => "WR Neighbors",
            Self::ExpectedNeighbors => "Expected Neighbors",
            Self::Combined => "Combined",
        })
    }
}
