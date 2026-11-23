mod compat;
mod players;

use backwards_compat::{
    BackwardsCompat, BackwardsCompatDeserialize, Migrated, VersionKey, backwards_compat,
};

use self::{
    compat::{V3Tournament, V4Tournament, V5Tournament, V6Tournament, V7Tournament},
    players::SerializedPlayerInfo,
};
use crate::{
    config::{game::GameConfig, matchmaker::MatchmakerConfig},
    error::TournamentError,
    game::entry::GameEntry,
    tournament::Tournament,
};

#[derive(Debug, derive_more::Constructor)]
#[backwards_compat(
    tag = "version",
    versions(
        "3": V3Tournament => "5",
        "4": V4Tournament,
        "5": V5Tournament => 7,
        "6": V6Tournament,
        7 | "7": V7Tournament,
        8: Self
    )
)]
pub(super) struct SerializedTournament {
    #[serde(rename = "pls", alias = "players")]
    players: SerializedPlayerInfo,
    #[serde(rename = "gms", alias = "games")]
    games: Vec<GameEntry>,
    #[serde(rename = "gc")]
    game_config: GameConfig,
    #[serde(rename = "mc")]
    matchmaker_config: MatchmakerConfig,
}

impl From<Tournament> for SerializedTournament {
    fn from(value: Tournament) -> Self {
        Self {
            game_config: value.game_config,
            matchmaker_config: value.matchmaker_config,
            players: FromIterator::from_iter(value.players),
            games: value.games.into_iter().map(GameEntry::from).collect(),
        }
    }
}

impl TryFrom<SerializedTournament> for Tournament {
    type Error = TournamentError;
    fn try_from(value: SerializedTournament) -> Result<Self, Self::Error> {
        let mut tournament = Self {
            game_config: value.game_config,
            matchmaker_config: value.matchmaker_config,
            players: value.players.value(),
            ..Self::default()
        };
        tournament.reload()?;
        for game in value.games {
            tournament.record_entry(game)?;
        }

        Ok(tournament)
    }
}

impl BackwardsCompat for Tournament {
    type Version = VersionKey;
    const CURRENT_VERSION: Self::Version = SerializedTournament::CURRENT_VERSION;
    const SERIALIZE_VERSION: Self::Version = SerializedTournament::SERIALIZE_VERSION;
    const SUPPORTED_VERSIONS: &'static [Self::Version] = SerializedTournament::SUPPORTED_VERSIONS;
    const ACCEPTS_UNTAGGED: bool = SerializedTournament::ACCEPTS_UNTAGGED;
}

impl<'de> BackwardsCompatDeserialize<'de> for Tournament {
    fn deserialize_migrated<D>(deserializer: D) -> Result<Migrated<Self>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let migrated = SerializedTournament::deserialize_migrated(deserializer)?;
        Ok(Migrated {
            value: Self::try_from(migrated.value).map_err(serde::de::Error::custom)?,
            source_version: migrated.source_version,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::tournament::Tournament;

    use rstest::rstest;

    #[rstest]
    fn deserialize(
        #[base_dir = "tests/fixtures/saves"]
        #[files("*.ron")]
        #[mode=str]
        data: &str,
    ) {
        let _: Tournament = ron::from_str(data).unwrap();
    }

    #[test]
    fn ron_serialize_loop() {
        let mut tournament = Tournament::generate_tournament(15, 100).unwrap();
        for _ in 0..10 {
            let ser = ron::ser::to_string(&tournament).unwrap();
            let deser: Tournament = ron::from_str(&ser).unwrap();
            assert_eq!(tournament.games().len(), deser.games().len());
            assert_eq!(tournament.players().len(), deser.players().len());

            tournament = deser;
            tournament.register_debug_player().unwrap();
            tournament
                .record_entry(tournament.random_game().unwrap())
                .unwrap();
        }
    }

    #[test]
    fn json_serialize_loop() {
        let mut tournament = Tournament::generate_tournament(15, 100).unwrap();
        for _ in 0..10 {
            let ser = serde_json::to_string(&tournament).unwrap();
            let deser: Tournament = serde_json::from_str(&ser).unwrap();
            assert_eq!(tournament.games().len(), deser.games().len());
            assert_eq!(tournament.players().len(), deser.players().len());

            tournament = deser;
            tournament.register_debug_player().unwrap();
            tournament
                .record_entry(tournament.random_game().unwrap())
                .unwrap();
        }
    }

    #[test]
    fn toml_serialize_loop() {
        let mut tournament = Tournament::generate_tournament(15, 100).unwrap();
        for _ in 0..10 {
            let ser = toml::to_string(&tournament).unwrap();
            let deser: Tournament = toml::from_str(&ser).unwrap();
            assert_eq!(tournament.games().len(), deser.games().len());
            assert_eq!(tournament.players().len(), deser.players().len());

            tournament = deser;
            tournament.register_debug_player().unwrap();
            tournament
                .record_entry(tournament.random_game().unwrap())
                .unwrap();
        }
    }

    #[test]
    fn deserialize_populates_player_table() {
        let mut tourn = Tournament::sample_game();
        let id = tourn.register_player(String::from("Test String")).unwrap();

        let serialized = ron::to_string(&tourn).unwrap();
        let de_tourn: Tournament = ron::from_str(&serialized).unwrap();

        assert_eq!(id, de_tourn.get_player_id(&String::from("Test String")).unwrap());
    }

    #[test]
    fn deserialize_configures_default_stats() {
        let mut tourn = Tournament::sample_game();
        let mut config = tourn.game_config().clone();
        *config.initial_elo_mut() += 1500.0;
        tourn.set_game_config(config).unwrap();
        let starting_elo = tourn.default_stats().elo();

        let serialized = ron::to_string(&tourn).unwrap();
        let de_tourn: Tournament = ron::from_str(&serialized).unwrap();
        assert!((starting_elo - de_tourn.default_stats().elo()) <= 1e-9);
    }
}
