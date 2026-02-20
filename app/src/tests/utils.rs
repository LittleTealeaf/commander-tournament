use edh_tourn::Tournament;

pub fn sample_tournament() -> Tournament {
    ron::from_str(include_str!("../../../tests/sample-game.ron")).unwrap()
}
