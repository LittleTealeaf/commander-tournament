use anyhow::anyhow;
use app::{
    App,
    logic::Message,
    view::{
        config_matchmaker::{MatchmakerConfigOption, MessageMatchmakerConfig},
        player::ViewPlayerMessage,
    },
};
use edh_tourn::{
    tournament::Tournament,
    player::{color::MtgColor, info::PlayerInfo},
};
use iced::widget::text_editor::{Action, Edit};
use itertools::chain;

#[test]
fn create_and_update_player_info() -> anyhow::Result<()> {
    const NAME: &str = "testing";
    const DESCRIPTION: &str = "This is a Description";
    const NEW_DESCRIPTION: &str = "This is a new description";
    const MOXFIELD_ID: &str = "09j23fj0f23j023jf2";
    const COLOR: MtgColor = MtgColor::Red;
    let mut app = App::default();

    let mut info = PlayerInfo::new(NAME.to_owned());
    info.set_description(DESCRIPTION.to_owned());
    info.set_moxfield_id(MOXFIELD_ID.to_owned());
    info.add_color(COLOR);
    helpers::update_player(&mut app, None, &info)?;

    let id = app
        .tournament()
        .get_player_id(&NAME.to_owned())
        .ok_or_else(|| anyhow!("Could not find player"))?;

    {
        let fetched_info = app
            .tournament()
            .get_player_info(&id)
            .expect("Expected ID to have Info");
        assert_eq!(&info, fetched_info);
    }

    info.set_description(NEW_DESCRIPTION.to_owned());

    helpers::update_player(&mut app, Some(id), &info)?;
    {
        let fetched_info = app
            .tournament()
            .get_player_info(&id)
            .expect("Expected ID to have Info");
        assert_eq!(&info, fetched_info);
    }

    Ok(())
}

#[test]
fn loading_tournaments() -> anyhow::Result<()> {
    for tourn in Tournament::test_tournaments() {
        let mut app = App::default();
        helpers::input_matchmaking_config(&mut app, &tourn)?;
        helpers::input_players(&mut app, &tourn)?;
    }

    Ok(())
}

mod helpers {
    use super::*;

    pub fn update_player(app: &mut App, id: Option<u32>, info: &PlayerInfo) -> anyhow::Result<()> {
        app.test_update(Message::batch(
            chain!(
                [
                    ViewPlayerMessage::Open(id),
                    ViewPlayerMessage::SetName(info.name().clone()),
                    ViewPlayerMessage::EditDescription(Action::SelectAll),
                    ViewPlayerMessage::EditDescription(Action::Edit(Edit::Delete)),
                    ViewPlayerMessage::EditDescription(Action::Edit(Edit::Paste(
                        info.description().to_owned().into(),
                    ))),
                    ViewPlayerMessage::ClearColors,
                ],
                info.moxfield_id()
                    .map(|id| ViewPlayerMessage::SetMoxfieldId(id.to_owned())),
                info.color_identity()
                    .colors()
                    .map(ViewPlayerMessage::ToggleColor),
                [ViewPlayerMessage::SaveAndClose]
            )
            .map(Into::into),
        ))?;
        Ok(())
    }

    pub fn input_players(app: &mut App, tournament: &Tournament) -> anyhow::Result<()> {
        for player in tournament.players().values() {
            update_player(app, None, player)?;
            let id = app
                .tournament()
                .get_player_id(player.name())
                .expect("Expected Player to Exist");
            let info = app
                .tournament()
                .get_player_info(&id)
                .expect("Expected Player Info");

            assert_eq!(player, info, "Expected info to be identical");
        }
        Ok(())
    }

    pub fn input_matchmaking_config(app: &mut App, tournament: &Tournament) -> anyhow::Result<()> {
        let config = tournament.ranking_config();
        app.test_update(Message::batch(
            [
                MessageMatchmakerConfig::Open,
                MessageMatchmakerConfig::SetConfigValue(
                    MatchmakerConfigOption::LeastPlayed,
                    format!("{}", config.least_played),
                ),
                MessageMatchmakerConfig::SetConfigValue(
                    MatchmakerConfigOption::Nemesis,
                    format!("{}", config.nemesis),
                ),
                MessageMatchmakerConfig::SetConfigValue(
                    MatchmakerConfigOption::LostWith,
                    format!("{}", config.lost_with),
                ),
                MessageMatchmakerConfig::SetConfigValue(
                    MatchmakerConfigOption::EloNeighbor,
                    format!("{}", config.elo_neighbor),
                ),
                MessageMatchmakerConfig::SetConfigValue(
                    MatchmakerConfigOption::WRNeighbor,
                    format!("{}", config.wr_neighbor),
                ),
                MessageMatchmakerConfig::SetConfigValue(
                    MatchmakerConfigOption::ExpectedNeighbor,
                    format!("{}", config.expected_neighbor),
                ),
                MessageMatchmakerConfig::Save,
            ]
            .map(Into::into),
        ))?;

        let tourn = app.tournament();
        let t_config = tourn.ranking_config();

        let pairs = [
            ("Least Played", config.least_played, t_config.least_played),
            ("Nemesis", config.nemesis, t_config.nemesis),
            ("Lost With", config.lost_with, t_config.lost_with),
            ("Elo Neighbor", config.elo_neighbor, t_config.elo_neighbor),
            ("WR Neighbor", config.wr_neighbor, t_config.wr_neighbor),
            (
                "Expected Neighbor",
                config.expected_neighbor,
                t_config.expected_neighbor,
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
}
