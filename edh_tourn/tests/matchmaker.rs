use edh_tourn::{game::next_mode::NextPlayerMode, player::PlayerId, tournament::Tournament};
mod play_next {
    use super::*;

    mod least_games {
        use super::*;

        #[test]
        fn returns_least() {
            for i in 0..10 {
                let mut tournament = Tournament::new();
                let [op_a, op_b, op_c] = tournament.register_debug_players().unwrap();

                let players: [PlayerId; 10] = tournament.register_debug_players().unwrap();
                let indexed = players.into_iter().enumerate();
                let with_counts = indexed
                    .map(|(id, player)| ((id + i) % 10, player))
                    .collect::<Vec<_>>();

                let (_, expected) = *with_counts.iter().find(|(count, _)| *count == 0).unwrap();

                for (games, player) in with_counts {
                    for _ in 0..=games {
                        let matchup = tournament.create_match([player, op_a, op_b, op_c]).unwrap();
                        let record = matchup.record(player).unwrap();
                        tournament.record_game(record).unwrap();
                    }
                }

                let least_games = tournament
                    .matchmaker()
                    .get_next_lead_player(NextPlayerMode::LeastGames)
                    .unwrap();
                assert_eq!(least_games, expected);
            }
        }

        #[test]
        fn returns_zero_games() {
            let mut tournament = Tournament::new();
            let others = tournament.register_debug_players().unwrap();
            let player = tournament.register_debug_player().unwrap();

            let matchup = tournament.create_match(others).unwrap();
            let [winner, ..] = others;
            let record = matchup.record(winner).unwrap();
            tournament.record_game(record).unwrap();

            let least_games = tournament
                .matchmaker()
                .get_next_lead_player(NextPlayerMode::LeastGames)
                .unwrap();

            assert_eq!(least_games, player);
        }

        #[test]
        fn filters_precons() {
            let mut tournament = Tournament::new();
            let others = tournament.register_debug_players().unwrap();
            let [target, precon] = tournament.register_debug_players().unwrap();

            tournament
                .update_player_info(&precon, |mut info| {
                    info.set_precon(true);
                    info
                })
                .unwrap();

            for _ in 0..2 {
                let matchup = tournament.create_match(others).unwrap();
                let [winner, ..] = others;
                let record = matchup.record(winner).unwrap();
                tournament.record_game(record).unwrap();
            }

            let [player_a, player_b, player_c, _] = others;
            let matchup = tournament
                .create_match([target, player_a, player_b, player_c])
                .unwrap();
            let record = matchup.record(target).unwrap();
            tournament.record_game(record).unwrap();

            let mut config = tournament.matchmaker_config().clone();
            config.exclude_precons = true;
            tournament.set_matchmaker_config(config);

            let least_games = tournament
                .matchmaker()
                .get_next_lead_player(NextPlayerMode::LeastGames)
                .unwrap();

            assert_eq!(least_games, target);
        }
    }
}

