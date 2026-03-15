#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct RankingConfig {
    pub least_played: usize,
    pub nemesis: usize,
    pub lost_with: usize,
    pub elo_neighbor: usize,
    pub wr_neighbor: usize,
    pub expected_neighbor: usize,
}

impl Default for RankingConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl RankingConfig {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            least_played: 6,
            nemesis: 4,
            lost_with: 5,
            elo_neighbor: 3,
            wr_neighbor: 3,
            expected_neighbor: 4,
        }

    }
}
