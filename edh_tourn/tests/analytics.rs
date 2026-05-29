use edh_tourn::{
    error::TournamentError,
    player::{color::ColorIdentity, info::PlayerInfo},
    tournament::Tournament,
};

mod aggregated_identity {
    use approx::assert_relative_eq;

    use super::*;

    fn register_with_color_identity(
        tourn: &mut Tournament,
        identity: ColorIdentity,
    ) -> Result<(), TournamentError> {
        let id = tourn.register_debug_player()?;
        tourn.update_player_info(&id, |info| info.with_color_identity(identity))?;
        Ok(())
    }

    #[test]
    fn includes_present_identities() {
        let mut tourn = Tournament::new();
        register_with_color_identity(&mut tourn, ColorIdentity::BLACK).unwrap();
        assert!(
            tourn
                .analytics()
                .aggregated_identity_stats()
                .contains_key(&ColorIdentity::BLACK)
        );

        register_with_color_identity(&mut tourn, ColorIdentity::BLUE).unwrap();

        let agg = tourn.analytics().aggregated_identity_stats();
        assert!(agg.contains_key(&ColorIdentity::BLUE));
        assert!(agg.contains_key(&ColorIdentity::BLACK));
    }

    #[test]
    #[allow(clippy::indexing_slicing, reason = "tests")]
    fn averages_elo() {
        let mut tourn = Tournament::new();
        let [id_high, id_low] = tourn.register_debug_players().unwrap();
        for _ in 0..5 {
            let matchup = tourn.create_match([id_high, id_low, id_low, id_low]).unwrap();
            let record = matchup.record(id_high).unwrap();
            tourn.record_game(record).unwrap();
        }
        let high_elo = tourn.get_player_or_default_stats(id_high).elo();
        let low_elo = tourn.get_player_or_default_stats(id_low).elo();
        let avg_elo = f64::midpoint(high_elo, low_elo);

        tourn
            .update_player_info(&id_high, |info| info.with_color_identity(ColorIdentity::RED))
            .unwrap();
        {
            let aggregates = tourn.analytics().aggregated_identity_stats();
            let stats = &aggregates[&ColorIdentity::RED];
            assert_relative_eq!(stats.avg_elo().unwrap(), high_elo);
        }
        tourn
            .update_player_info(&id_low, |info| info.with_color_identity(ColorIdentity::RED))
            .unwrap();
        {
            let aggregates = tourn.analytics().aggregated_identity_stats();
            let stats = &aggregates[&ColorIdentity::RED];
            assert_relative_eq!(stats.avg_elo().unwrap(), avg_elo);
        }
    }
}
