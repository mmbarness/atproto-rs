use bsky_rs::Client;
use derive_getters::Getters;
use eframe::egui;

#[derive(Default, Getters)]
struct BskyGui {
    client: Client,
    identity: String,
    provider: String,
    password: String,
    post_text: String,
    logged_in: bool,
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Bluesky Native",
        options,
        Box::new(|cc| Ok(Box::new(BskyGui::new(cc)))),
    );
}

impl BskyGui {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}
impl eframe::App for BskyGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Login");
            ui.horizontal(|ui| {
                let provider_label = ui.label("Provider: ");
                ui.text_edit_singleline(&mut self.provider)
                    .labelled_by(provider_label.id);
            });

            ui.horizontal(|ui| {
                let identity_label = ui.label("Identitifier: ");
                ui.text_edit_singleline(&mut self.identity)
                    .labelled_by(identity_label.id);
            });

            ui.horizontal(|ui| {
                let password_label = ui.label("Password: ");
                ui.text_edit_singleline(&mut self.password)
                    .labelled_by(password_label.id);
            });
            if self.logged_in {
                ui.horizontal(|ui| {
                    let post_text_label = ui.label("Post:");
                    ui.text_edit_multiline(&mut self.post_text)
                        .labelled_by(post_text_label.id);
                });
                if ui.button("Post").clicked() {
                    let jwt: String = self.client.jwt().to_owned();
                    let did: String = self.client.did().to_owned();
                    match self.client.post(did, jwt, self.post_text().to_owned()) {
                        Ok(_) => {
                            println!("Post made");
                        }
                        Err(e) => {
                            println!("Error making post: {}", e);
                        }
                    }
                }
            } else {
                if ui.button("Login").clicked() {
                    let mut provider: String;
                    if !self.provider.starts_with("https://") {
                        provider = "".to_owned() + "https://" + &self.provider;
                    } else {
                        provider = "".to_owned() + &self.provider;
                    }
                    if !self.provider.ends_with("/") {
                        provider = "".to_owned() + &provider + "/";
                    } else {
                        provider = "".to_owned() + &provider;
                    }
                    self.client = Client::new(&provider);
                    match self.client.login(&self.identity, self.password.to_string()) {
                        Ok(_) => {
                            println!("logged in");
                            ui.label(format!("Logged in as {}", self.identity));
                            self.logged_in = true;
                        }
                        Err(_) => println!("Not Logged in"),
                    }
                }
            }
        });
    }
}
