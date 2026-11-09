use crate::{
    app::{App, Message},
    games::Game,
};

pub fn show(ui: &mut egui::Ui, app: &App) {
    if ui.button("Zenless Zone Zero").clicked() {
        app.message_tx.send(Message::Game(Game::Zzz)).unwrap();
    }
}
