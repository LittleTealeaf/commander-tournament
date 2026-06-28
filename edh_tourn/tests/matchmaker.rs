use edh_tourn::{game::next_mode::NextPlayerMode, player::PlayerId, tournament::Tournament};
mod play_next {
    use super::*;

    fn make_precon(tournament: &mut Tournament, id: PlayerId) {
        let mut config = tournament.matchmaker_config().clone();
        config.exclude_precons = true;
        tournament.set_matchmaker_config(config);

        tournament
            .update_player_info(&id, |mut info| {
                info.set_precon(true);
                info
            })
            .unwrap();
    }

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

            let [winner, _, _, _] = others;
            tournament.debug_game(others, winner).unwrap();

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

            let [winner, _, _, _] = others;
            for _ in 0..2 {
                tournament.debug_game(others, winner).unwrap();
            }

            let [player_a, player_b, player_c, _] = others;
            tournament
                .debug_game([target, player_a, player_b, player_c], target)
                .unwrap();

            make_precon(&mut tournament, precon);

            let least_games = tournament
                .matchmaker()
                .get_next_lead_player(NextPlayerMode::LeastGames)
                .unwrap();

            assert_eq!(least_games, target);
        }
    }

    mod longest_break {
        use super::*;

        #[test]
        fn returns_longest() {
            let mut tournament = Tournament::new();
            let [pa, pb, pc, pd, pe] = tournament.register_debug_players().unwrap();

            tournament.debug_game([pa, pb, pc, pd], pa).unwrap();
            tournament.debug_game([pa, pb, pc, pe], pc).unwrap();

            let value = tournament
                .matchmaker()
                .get_next_lead_player(NextPlayerMode::LongestBreak)
                .unwrap();

            assert_eq!(value, pd);
        }

        #[test]
        fn returns_zero_games() {
            let mut tournament = Tournament::new();
            let [pa, pb, pc, pd, pe] = tournament.register_debug_players().unwrap();
            tournament.debug_game([pa, pb, pc, pd], pa).unwrap();

            let value = tournament
                .matchmaker()
                .get_next_lead_player(NextPlayerMode::LongestBreak)
                .unwrap();
            assert_eq!(value, pe);
        }

        #[test]
        fn filters_precon() {
            let mut tournament = Tournament::new();
            let [pa, pb, pc, pd, expected, precon] = tournament.register_debug_players().unwrap();
            make_precon(&mut tournament, precon);
            tournament.debug_game([pa, pb, pc, precon], precon).unwrap();
            tournament.debug_game([pa, pb, pc, expected], pa).unwrap();
            tournament.debug_game([pa, pb, pc, pd], pa).unwrap();

            let value = tournament
                .matchmaker()
                .get_next_lead_player(NextPlayerMode::LongestBreak)
                .unwrap();
            assert_eq!(value, expected);
        }
    }

    mod longest_lead_break {
        use super::*;

        #[test]
        fn returns_longest() {
            let mut tournament = Tournament::new();
            let [pa, pb, pc, pd] = tournament.register_debug_players().unwrap();

            tournament.debug_game([pa, pb, pc, pd], pb).unwrap();
            tournament.debug_game([pb, pa, pc, pd], pb).unwrap();
            tournament.debug_game([pc, pb, pa, pd], pb).unwrap();
            tournament.debug_game([pd, pb, pa, pc], pb).unwrap();

            let value = tournament
                .matchmaker()
                .get_next_lead_player(NextPlayerMode::LongestLeadBreak)
                .unwrap();
            assert_eq!(value, pa);
        }

        #[test]
        fn returns_zero_leads() {
            let mut tournament = Tournament::new();
            let [pa, pb, pc, pd] = tournament.register_debug_players().unwrap();

            tournament.debug_game([pb, pa, pc, pd], pb).unwrap();
            tournament.debug_game([pc, pb, pa, pd], pb).unwrap();
            tournament.debug_game([pd, pb, pa, pc], pb).unwrap();

            let value = tournament
                .matchmaker()
                .get_next_lead_player(NextPlayerMode::LongestLeadBreak)
                .unwrap();
            assert_eq!(value, pa);
        }

        #[test]
        fn filters_precon() {
            let mut tournament = Tournament::new();
            let [pa, pb, pc, pd] = tournament.register_debug_players().unwrap();
            make_precon(&mut tournament, pa);

            tournament.debug_game([pa, pb, pc, pd], pb).unwrap();
            tournament.debug_game([pb, pa, pc, pd], pb).unwrap();
            tournament.debug_game([pc, pb, pa, pd], pb).unwrap();
            tournament.debug_game([pd, pb, pa, pc], pb).unwrap();

            let value = tournament
                .matchmaker()
                .get_next_lead_player(NextPlayerMode::LongestLeadBreak)
                .unwrap();
            assert_eq!(value, pb);
        }
    }

    mod longest_since_win {
        use super::*;

        #[test]
        fn returns_longest() {
            let mut tournament = Tournament::new();
            let [pa, pb, pc, pd] = tournament.register_debug_players().unwrap();

            tournament.debug_game([pa, pb, pc, pd], pa).unwrap();
            tournament.debug_game([pb, pa, pc, pd], pb).unwrap();
            tournament.debug_game([pc, pb, pa, pd], pc).unwrap();
            tournament.debug_game([pd, pb, pa, pc], pd).unwrap();

            let value = tournament
                .matchmaker()
                .get_next_lead_player(NextPlayerMode::LongestSinceWin)
                .unwrap();
            assert_eq!(value, pa);
        }

        #[test]
        fn zero_wins() {
            let mut tournament = Tournament::new();
            let [pa, pb, pc, pd] = tournament.register_debug_players().unwrap();

            tournament.debug_game([pb, pa, pc, pd], pa).unwrap();
            tournament.debug_game([pc, pb, pa, pd], pc).unwrap();
            tournament.debug_game([pd, pb, pa, pc], pd).unwrap();

            let value = tournament
                .matchmaker()
                .get_next_lead_player(NextPlayerMode::LongestSinceWin)
                .unwrap();
            assert_eq!(value, pb);
        }

        #[test]
        fn filters_precons() {
            let mut tournament = Tournament::new();
            let [pa, pb, pc, pd] = tournament.register_debug_players().unwrap();
            make_precon(&mut tournament, pa);

            tournament.debug_game([pa, pb, pc, pd], pa).unwrap();
            tournament.debug_game([pb, pa, pc, pd], pb).unwrap();
            tournament.debug_game([pc, pb, pa, pd], pc).unwrap();
            tournament.debug_game([pd, pb, pa, pc], pd).unwrap();

            let value = tournament
                .matchmaker()
                .get_next_lead_player(NextPlayerMode::LongestSinceWin)
                .unwrap();
            assert_eq!(value, pb);
        }
    }

    mod least_wins {
        use super::*;

        #[test]
        fn returns_least_wins() {
            let mut tournament = Tournament::new();
            let players = tournament.register_debug_players().unwrap();
            let [p_a, p_b, p_c, p_d] = players;
            let wins = [(p_a, 2), (p_b, 1), (p_c, 4), (p_d, 3)];

            for (player, count) in wins {
                for _ in 0..count {
                    tournament.debug_game(players, player).unwrap();
                }
            }

            let least_wins = tournament
                .matchmaker()
                .get_next_lead_player(NextPlayerMode::LeastWins)
                .unwrap();

            assert_eq!(least_wins, p_b);
        }

        #[test]
        fn returns_zero_wins() {
            let mut tournament = Tournament::new();
            let players = tournament.register_debug_players().unwrap();
            let target = tournament.register_debug_player().unwrap();
            let [p_a, p_b, p_c, p_d] = players;
            let wins = [(p_a, 2), (p_b, 1), (p_c, 4), (p_d, 3)];

            for (player, count) in wins {
                for _ in 0..count {
                    tournament.debug_game(players, player).unwrap();
                }
            }

            let least_wins = tournament
                .matchmaker()
                .get_next_lead_player(NextPlayerMode::LeastWins)
                .unwrap();

            assert_eq!(least_wins, target);
        }

        #[test]
        fn filter_precons() {
            let mut tournament = Tournament::new();
            let players = tournament.register_debug_players().unwrap();
            let precon = tournament.register_debug_player().unwrap();
            let [p_a, p_b, p_c, p_d] = players;
            let wins = [(p_a, 2), (p_b, 1), (p_c, 4), (p_d, 3)];

            for (player, count) in wins {
                for _ in 0..count {
                    tournament.debug_game(players, player).unwrap();
                }
            }

            make_precon(&mut tournament, precon);

            let least_wins = tournament
                .matchmaker()
                .get_next_lead_player(NextPlayerMode::LeastWins)
                .unwrap();

            assert_eq!(least_wins, p_b);
        }
    }
}
