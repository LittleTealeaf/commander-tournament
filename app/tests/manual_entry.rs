use anyhow::anyhow;
use app::{
    App,
    logic::Message,
    view::{
        config_matchmaker::{MatchmakerConfigOption, MessageMatchmakerConfig},
        player::ViewPlayerMessage,
    },
};
use edh_tourn::{Tournament, player::color::MtgColor};
use iced::widget::text_editor::{Action, Edit};
use itertools::chain;

fn input_players(app: &mut App, tournament: &Tournament) -> anyhow::Result<()> {
    let mut iter_players = tournament.players().values().cloned();

    if let Some(info) = iter_players.next() {
        // Test with first setting a random text and then updating the info
        const NAME: &str = "TESTING PLAYER A";
        app.test_updates([
            ViewPlayerMessage::Open(None),
            ViewPlayerMessage::SetName(NAME.to_owned()),
            ViewPlayerMessage::ToggleColor(MtgColor::Red),
            ViewPlayerMessage::SaveAndClose,
        ])?;

        let id = app
            .tournament()
            .get_player_id(&NAME.to_owned())
            .ok_or_else(|| anyhow!("Player not found"))?;

        app.test_updates(chain!(
            [
                ViewPlayerMessage::Open(Some(id)),
                ViewPlayerMessage::SetName(info.name().clone()),
                ViewPlayerMessage::EditDescription(Action::Edit(Edit::Paste(
                    info.description().to_owned().into()
                ),)),
                ViewPlayerMessage::SetMoxfieldId(info.moxfield_id().cloned().unwrap_or_default(),),
                ViewPlayerMessage::ToggleColor(MtgColor::Red)
            ],
            info.color_identity()
                .to_colors()
                .map(ViewPlayerMessage::ToggleColor),
            [ViewPlayerMessage::SaveAndClose]
        ))?;

        assert_eq!(
            info,
            app.tournament().get_player_info(&id).cloned().unwrap(),
            "Expected Info to be updated accordingly"
        );
    }

    for player in iter_players {
        app.test_updates(chain!(
            [
                ViewPlayerMessage::Open(None),
                ViewPlayerMessage::SetName(player.name().to_owned()),
                ViewPlayerMessage::EditDescription(Action::Edit(Edit::Paste(
                    player.description().to_owned().into()
                ),)),
                ViewPlayerMessage::SetMoxfieldId(player.moxfield_id().cloned().unwrap_or_default(),)
            ],
            player
                .color_identity()
                .to_colors()
                .map(ViewPlayerMessage::ToggleColor),
            [ViewPlayerMessage::SaveAndClose]
        ))?;

        let id = app
            .tournament()
            .get_player_id(player.name())
            .expect("Expected Player to Exist");
        let info = app
            .tournament()
            .get_player_info(&id)
            .expect("Expected Player Info");

        assert_eq!(&player, info, "Expected info to be identical");
    }

    Ok(())
}

fn input_matchmaking_config(app: &mut App, tournament: &Tournament) -> anyhow::Result<()> {
    let config = tournament.config();
    app.test_update(Message::batch(
        [
            MessageMatchmakerConfig::Open,
            MessageMatchmakerConfig::SetConfigValue(
                MatchmakerConfigOption::LeastPlayed,
                format!("{}", config.match_weight_least_played),
            ),
            MessageMatchmakerConfig::SetConfigValue(
                MatchmakerConfigOption::Nemesis,
                format!("{}", config.match_weight_nemesis),
            ),
            MessageMatchmakerConfig::SetConfigValue(
                MatchmakerConfigOption::LostWith,
                format!("{}", config.match_weight_lost_with),
            ),
            MessageMatchmakerConfig::SetConfigValue(
                MatchmakerConfigOption::EloNeighbor,
                format!("{}", config.match_weight_elo_neighbor),
            ),
            MessageMatchmakerConfig::SetConfigValue(
                MatchmakerConfigOption::WRNeighbor,
                format!("{}", config.match_weight_wr_neighbor),
            ),
            MessageMatchmakerConfig::SetConfigValue(
                MatchmakerConfigOption::ExpectedNeighbor,
                format!("{}", config.match_weight_expected_neighbor),
            ),
            MessageMatchmakerConfig::Save,
        ]
        .map(Into::into),
    ))?;

    let tourn = app.tournament();
    let t_config = tourn.config();

    let pairs = [
        (
            "Least Played",
            config.match_weight_least_played,
            t_config.match_weight_least_played,
        ),
        (
            "Nemesis",
            config.match_weight_nemesis,
            t_config.match_weight_nemesis,
        ),
        (
            "Lost With",
            config.match_weight_lost_with,
            t_config.match_weight_lost_with,
        ),
        (
            "Elo Neighbor",
            config.match_weight_elo_neighbor,
            t_config.match_weight_elo_neighbor,
        ),
        (
            "WR Neighbor",
            config.match_weight_wr_neighbor,
            t_config.match_weight_wr_neighbor,
        ),
        (
            "Expected Neighbor",
            config.match_weight_expected_neighbor,
            t_config.match_weight_expected_neighbor,
        ),
    ];

    for (name, expected, found) in pairs {
        assert!(
            approx::abs_diff_eq!(expected, found),
            "{name} is different: {expected} to {found}"
        );
    }

    Ok(())
}

#[test]
fn test_loading_tournaments() -> anyhow::Result<()> {
    for tourn in Tournament::test_tournaments() {
        let mut app = App::default();

        input_matchmaking_config(&mut app, &tourn)?;
        input_players(&mut app, &tourn)?;
    }

    Ok(())
}
