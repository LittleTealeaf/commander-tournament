#![allow(clippy::indexing_slicing, reason = "tests")]
use approx::assert_relative_eq;
use commander_tournament_core::{player::color::ColorIdentity, tournament::Tournament};
use itertools::Itertools;

use commander_tournament_core::player::PlayerId;

mod aggregated_identity {

    use super::*;

    #[test]
    fn includes_present_identities() {
        let mut tourn = Tournament::new();
        let agg = tourn.analytics().aggregated_identity_stats();
        assert!(!agg.contains_key(&ColorIdentity::BLACK));
        assert!(!agg.contains_key(&ColorIdentity::BLUE));

        tourn
            .register_debug_player_info(|info| info.with_color_identity(ColorIdentity::BLACK))
            .unwrap();
        let agg = tourn.analytics().aggregated_identity_stats();
        assert!(agg.contains_key(&ColorIdentity::BLACK));
        assert!(!agg.contains_key(&ColorIdentity::BLUE));

        tourn
            .register_debug_player_info(|info| info.with_color_identity(ColorIdentity::BLUE))
            .unwrap();
        let agg = tourn.analytics().aggregated_identity_stats();
        assert!(agg.contains_key(&ColorIdentity::BLACK));
        assert!(agg.contains_key(&ColorIdentity::BLUE));
    }

    #[test]
    fn averages_elo() {
        let mut tourn = Tournament::new();
        let id_loser = tourn.register_debug_player().unwrap();
        let [id_high, id_base] = tourn
            .register_debug_players_info(|info| info.with_color_identity(ColorIdentity::SELESNYA))
            .unwrap();

        {
            let matchup = tourn
                .create_match([id_high, id_loser, id_loser, id_loser])
                .unwrap();
            let record = matchup.record(id_high).unwrap();
            tourn.record_game(record).unwrap();
        }

        let high_elo = tourn.get_player_or_default_stats(id_high).elo();
        let avg_elo = tourn.get_player_or_default_stats(id_base).elo();

        let target_elo = f64::midpoint(high_elo, avg_elo);

        let agg = tourn.analytics().aggregated_identity_stats();
        let agg_stats = &agg[&ColorIdentity::SELESNYA];
        let agg_stats_elo = agg_stats.avg_elo().unwrap();
        assert_relative_eq!(agg_stats_elo, target_elo);
    }

    #[test]
    fn filters_pool() {
        let mut tourn = Tournament::new();
        let [id_high, id_low] = tourn
            .register_debug_players_info(|info| info.with_color_identity(ColorIdentity::GRIXIS))
            .unwrap();

        {
            let matchup = tourn.create_match([id_high, id_low, id_low, id_low]).unwrap();
            let record = matchup.record(id_high).unwrap();
            tourn.record_game(record).unwrap();
        }

        let expected_elo = tourn.get_player_or_default_stats(id_high).elo();

        let analytics = tourn.analytics().with_pool(|id| id != id_low);
        let agg = analytics.aggregated_identity_stats();
        let stats = &agg[&ColorIdentity::GRIXIS];
        let elo = stats.avg_elo().unwrap();
        assert_relative_eq!(elo, expected_elo);
    }
}

mod player_vs_player_performance {

    use super::*;

    #[test]
    fn include_only_played() {
        let mut tourn = Tournament::new();
        let [id_target, id_active, id_hidden] = tourn.register_debug_players().unwrap();

        {
            let matchup = tourn
                .create_match([id_target, id_active, id_active, id_active])
                .unwrap();
            let record = matchup.record(id_target).unwrap();
            tourn.record_game(record).unwrap();
        }

        let analytics = tourn.analytics();

        let perfs = analytics
            .player_vs_player_performance(id_target)
            .unwrap()
            .collect_vec();

        let ids = perfs
            .iter()
            .map(|(p, _)| p.id())
            .collect::<im::HashSet<PlayerId>>();

        assert!(ids.contains(&id_active));
        assert!(!ids.contains(&id_hidden));
    }

    #[test]
    fn filter_pool() {
        let mut tourn = Tournament::new();
        let [id_target, id_active, id_hidden] = tourn.register_debug_players().unwrap();

        for id in [id_active, id_hidden] {
            let matchup = tourn.create_match([id_target, id, id, id]).unwrap();
            let record = matchup.record(id_target).unwrap();
            tourn.record_game(record).unwrap();
        }

        let analytics = tourn.analytics().with_pool(|id| id != id_hidden);

        let perfs = analytics
            .player_vs_player_performance(id_target)
            .unwrap()
            .collect_vec();

        let ids = perfs
            .iter()
            .map(|(p, _)| p.id())
            .collect::<im::HashSet<PlayerId>>();

        assert!(ids.contains(&id_active));
        assert!(!ids.contains(&id_hidden));
    }
}
