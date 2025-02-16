use eframe::egui;
use screenshots::Screen;
use image::{ImageBuffer, Rgba};
use notify_rust::Notification;
use std::process;

// Struttura dell'applicazione
#[derive(Default)]
struct App {
    main_menu: Menu,
    screens: Vec<Screen>,
    selected_screen: Option<usize>,
}

// Enum per la gestione dei menu
#[derive(PartialEq)]
enum Menu {
    Main,
    ScreenSelector,
    Info,
    License,
    Contrib,
    Exit,
}

impl Default for Menu {
    fn default() -> Self {
        Menu::Main
    }
}

impl App {
    fn new() -> Self {
        let screens = Screen::all().unwrap_or_default();
        App {
            main_menu: Menu::Main,
            screens,
            selected_screen: None,
        }
    }

    fn show_main_menu(&mut self, ui: &mut egui::Ui) {
        ui.heading("Menu Principale");

        if ui.button("Seleziona Schermo").clicked() {
            self.main_menu = Menu::ScreenSelector;
        }
        if ui.button("Informazioni").clicked() {
            self.main_menu = Menu::Info;
        }
        if ui.button("Esci").clicked() {
            self.main_menu = Menu::Exit;
        }
    }

    fn show_screen_selector(&mut self, ui: &mut egui::Ui) {
        ui.heading("Seleziona uno schermo");

        if self.screens.is_empty() {
            ui.label("Nessuno schermo trovato.");
        } else {
	    ui.horizontal(|ui| {
		for (i, screen) in self.screens.iter().enumerate() {
                    let label = format!(
			"Schermo {} - {}x{}",
			screen.display_info.id,
			screen.display_info.width,
			screen.display_info.height
                    );
                    if ui.button(label).clicked() {
			self.selected_screen = Some(i);
                    }
		}
	    });
        }

        if let Some(index) = self.selected_screen {
            let selected_screen = &self.screens[index];
            ui.label(format!(
                "Schermo selezionato: {} - {}x{}",
                selected_screen.display_info.id,
                selected_screen.display_info.width,
                selected_screen.display_info.height
            ));

            if ui.button("Cattura Screenshot").clicked() {
                self.take_screenshot(&self.screens[index]);
            }
        }

        if ui.button("Indietro").clicked() {
            self.main_menu = Menu::Main;
        }
    }

    fn take_screenshot(&self, screen: &Screen) {
        if let Ok(image) = screen.capture() {
            let filename = format!("screenshot-{}.png", screen.display_info.id);

            let img_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
                image.width(),
                image.height(),
                image.as_raw().to_vec(),
            );

            if let Some(buffer) = img_buffer {
                if let Err(e) = buffer.save(&filename) {
                    Notification::new()
                        .summary("Errore Screenshot")
                        .body(&format!("Impossibile salvare lo screenshot: {}", e))
                        .show()
                        .unwrap();
                } else {
                    Notification::new()
                        .summary("Screenshot salvato!")
                        .body(&format!("File salvato come {}", filename))
                        .show()
                        .unwrap();
                }
            } else {
                Notification::new()
                    .summary("Errore Screenshot")
                    .body("Errore durante la creazione dell'ImageBuffer!")
                    .show()
                    .unwrap();
            }
        }
    }

    fn show_info(&mut self, ui: &mut egui::Ui) {
        ui.heading("Informazioni");
        ui.label("Questa applicazione permette di catturare screenshot su Wayland.");

	if ui.button("Licenza").clicked() {
            self.main_menu = Menu::License;
        }
	if ui.button("Contributori").clicked() {
            self.main_menu = Menu::Contrib;
        }
	
	if ui.button("Indietro").clicked() {
            self.main_menu = Menu::Main;
        }
    }

    fn show_license(&mut self, ui: &mut egui::Ui) {
        ui.heading("Licenza");
        ui.label("Software rilasciato sotto licenza MIT.");
        if ui.button("Indietro").clicked() {
            self.main_menu = Menu::Info;
        }
    }

    fn show_contrib(&mut self, ui: &mut egui::Ui) {
        ui.heading("Contributori");
        ui.label("Progetto open-source, contributi benvenuti!");
        if ui.button("Indietro").clicked() {
            self.main_menu = Menu::Info;
        }
    }
}

// Implementazione dell'applicazione eGUI
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.main_menu {
                Menu::Main => self.show_main_menu(ui),
                Menu::ScreenSelector => self.show_screen_selector(ui),
                Menu::Info => self.show_info(ui),
                Menu::License => self.show_license(ui),
                Menu::Contrib => self.show_contrib(ui),
                Menu::Exit => {
		    process::exit(0x0000);
		},
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Stalker Capture",
        native_options,
        Box::new(|_cc| Box::new(App::new())),
    )
}
