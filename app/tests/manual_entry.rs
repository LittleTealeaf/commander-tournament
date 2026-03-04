use anyhow::anyhow;
use app::{App, traits::HandleMessage, view::view_player::ViewPlayerMessage};
use edh_tourn::{Tournament, info::MtgColor};
use iced::widget::text_editor::{Action, Edit};

fn test_input_players(app: &mut App, tournament: &Tournament) -> anyhow::Result<()> {
    let mut iter_players = tournament.players().values().cloned();

    if let Some(info) = iter_players.next() {
        // Test with first setting a random text and then updating the info
        const NAME: &str = "TESTING PLAYER A";
        app.test_update(ViewPlayerMessage::Open(None))?;
        app.test_update(ViewPlayerMessage::SetName(NAME.to_owned()))?;
        app.test_update(ViewPlayerMessage::ToggleColor(MtgColor::Red))?;
        app.test_update(ViewPlayerMessage::SaveAndClose)?;

        let id = app
            .tournament()
            .get_player_id(&NAME.to_owned())
            .ok_or_else(|| anyhow!("Player not found"))?;

        app.test_update(ViewPlayerMessage::Open(Some(id)))?;
        app.test_update(ViewPlayerMessage::SetName(info.name().clone()))?;
        app.test_update(ViewPlayerMessage::EditDescription(Action::Edit(
            Edit::Paste(info.description().to_owned().into()),
        )))?;
        app.test_update(ViewPlayerMessage::SetMoxfieldId(
            info.moxfield_id().cloned().unwrap_or_default(),
        ))?;
        app.test_update(ViewPlayerMessage::ToggleColor(MtgColor::Red))?;
        for color in info.colors() {
            app.test_update(ViewPlayerMessage::ToggleColor(*color))?;
        }
        app.test_update(ViewPlayerMessage::SaveAndClose)?;

        assert_eq!(
            info,
            app.tournament().get_player_info(&id).cloned().unwrap(),
            "Expected Info to be updated accordingly"
        );
    }

    for player in iter_players {
        app.test_update(ViewPlayerMessage::Open(None))?;
        app.test_update(ViewPlayerMessage::SetName(player.name().to_owned()))?;
        app.test_update(ViewPlayerMessage::EditDescription(Action::Edit(
            Edit::Paste(player.description().to_owned().into()),
        )))?;
        app.test_update(ViewPlayerMessage::SetMoxfieldId(
            player.moxfield_id().cloned().unwrap_or_default(),
        ))?;
        for color in player.colors() {
            app.test_update(ViewPlayerMessage::ToggleColor(*color))?;
        }
        app.test_update(ViewPlayerMessage::SaveAndClose)?;

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
