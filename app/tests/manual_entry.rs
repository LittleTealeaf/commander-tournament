use anyhow::anyhow;
use app::{App, view::player::ViewPlayerMessage};
use edh_tourn::{Tournament, info::MtgColor};
use iced::widget::text_editor::{Action, Edit};
use itertools::chain;

fn test_input_players(app: &mut App, tournament: &Tournament) -> anyhow::Result<()> {
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
            info.colors()
                .iter()
                .map(|color| ViewPlayerMessage::ToggleColor(*color)),
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
                .colors()
                .iter()
                .map(|color| ViewPlayerMessage::ToggleColor(*color)),
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

#[test]
fn test_loading_tournaments() -> anyhow::Result<()> {
    for tourn in Tournament::test_tournaments() {
        let mut app = App::default();
        test_input_players(&mut app, &tourn)?;
    }

    Ok(())
}
