use eframe::{egui, App};

pub struct TuahApp {
    state: AppState,
    connection_code: String,
    username: String,
}

enum AppState {
    MainMenu,
    Hosting,
    Joining,
}

impl Default for TuahApp {
    fn default() -> Self {
        Self {
            state: AppState::MainMenu,
            connection_code: String::from("Not implemented"),
            username: String::from("Enter your name"),
        }
    }
}

impl eframe::App for TuahApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                match self.state {
                    AppState::MainMenu => {
                        ui.heading("📡 Select an action");
                        ui.add_space(10.0);
                        if ui
                            .add_sized([200.0, 40.0], egui::Button::new("🚀 Create room"))
                            .clicked()
                        {
                            self.state = AppState::Hosting;
                        }
                        ui.add_space(10.0);
                        if ui
                            .add_sized([200.0, 40.0], egui::Button::new("🔗 Join to the room"))
                            .clicked()
                        {
                            self.state = AppState::Joining;
                        }
                    }
                    AppState::Hosting => {
                        ui.heading("🎥 Broadcasting is not implemented yet :(");
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new(format!(
                                "🔑 Connection code: {}",
                                self.connection_code
                            ))
                            .size(18.0),
                        );
                        ui.add_space(10.0);
                        if ui
                            .add_sized([200.0, 40.0], egui::Button::new("⏹ Leave room"))
                            .clicked()
                        {
                            self.state = AppState::MainMenu;
                        }
                    }
                    AppState::Joining => {
                        ui.heading("🔗 Join to the broadcast");
                        ui.add_space(10.0);
                        ui.label("👤 Enter your name:");
                        ui.add(egui::TextEdit::singleline(&mut self.username).desired_width(500.0));
                        ui.add_space(10.0);
                        ui.label("🔑 Enter connection code:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.connection_code)
                                .desired_width(500.0),
                        );
                        ui.add_space(10.0);
                        if ui
                            .add_sized([200.0, 40.0], egui::Button::new("🔗 Connect"))
                            .clicked()
                        {
                            //todo!();
                        }
                        ui.add_space(10.0);
                        if ui
                            .add_sized([200.0, 40.0], egui::Button::new("↩ Back to the menu"))
                            .clicked()
                        {
                            self.state = AppState::MainMenu;
                        }
                    }
                }
            });
        });
    }
}
