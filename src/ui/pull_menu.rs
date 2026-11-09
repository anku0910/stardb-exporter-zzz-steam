use std::thread;

use crate::{
    app::{App, Message, State},
    games,
};

pub fn show(ui: &mut egui::Ui, app: &App) {
    match app.game {
        games::Game::Zzz => {
            ui.label(format!(
                "Path: {}",
                app.paths
                    .zzz
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or("None".to_string())
            ));
        }
    }

    if ui.button("Automatic").clicked() {
        match app.game.game_path() {
            Ok(path) => app.message_tx.send(Message::Path(path)).unwrap(),
            Err(e) => app
                .message_tx
                .send(Message::GoTo(State::Error(e.to_string())))
                .unwrap(),
        }
    }

    if ui
        .button("Manual selection (e.g. F:\\SteamLibrary\\steamapps\\common\\Zenless Zone Zero\\games\\ZenlessZoneZero Game\\ZenlessZoneZero_Data)")
        && let Some(path) = rfd::FileDialog::new().pick_folder()
    {
        app.message_tx.send(Message::Path(path)).unwrap();
    }

    if let Some(path) = match app.game {
        games::Game::Zzz => &app.paths.zzz,
    } {
        if ui.button("Get Url").clicked() {
            let message_tx = app.message_tx.clone();
            let path = path.clone();

            thread::spawn(move || {
                match games::pulls_from_game_path(&path) {
                    Ok(url) => message_tx.send(Message::GoTo(State::Pulls(url))),
                    Err(e) => message_tx.send(Message::GoTo(State::Error(e.to_string()))),
                }
                .unwrap()
            });

            app.message_tx
                .send(Message::GoTo(State::Waiting("Running".to_string())))
                .unwrap();
        }
    } else {
        ui.add_enabled(false, egui::Button::new("Get Url"));
    }
}
