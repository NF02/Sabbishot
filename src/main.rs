use eframe::egui;
use screenshots::Screen;
use image::{ImageBuffer, Rgba};
use notify_rust::Notification;
use rfd::FileDialog;
use std::process;

// Application structure
#[derive(Default)]
struct App {
    main_menu: Menu,
    screens: Vec<Screen>,
    selected_screen: Option<usize>,
    save_path: String,
}

// Enum for menu management
#[derive(PartialEq)]
enum Menu {
    Main,
    FullScreen,
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
            save_path: String::new(),
            selected_screen: None,
        }
    }

    fn show_main_menu(&mut self, ui: &mut egui::Ui) {
        ui.heading("Main Menu");

        if ui.button("Full Screen").clicked(){
            self.main_menu = Menu::FullScreen;
        }
        if ui.button("Select Screen").clicked() {
            self.main_menu = Menu::ScreenSelector;
        }
        if ui.button("Information").clicked() {
            self.main_menu = Menu::Info;
        }
        if ui.button("Exit").clicked() {
            self.main_menu = Menu::Exit;
        }
    }

    fn show_full_screen(&mut self, ui: &mut egui::Ui) {
        ui.heading("Full Screen");
        if let Some(screen) = self.screens.first() {
            self.take_screenshot(screen);
        } else {
            Notification::new()
                .summary("Screenshot Error")
                .body("No available screen for capture!")
                .show()
                .unwrap();
        }
        self.main_menu = Menu::Main;
    }
    
    fn show_screen_selector(&mut self, ui: &mut egui::Ui) {
        ui.heading("Select Screen");

        if self.screens.is_empty() {
            ui.label("Screen not found...");
        } else {
            ui.horizontal(|ui| {
                for (i, screen) in self.screens.iter().enumerate() {
                    let label = format!(
                        "Screen {} - {}x{}",
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
                "Selected screen: {} - {}x{}",
                selected_screen.display_info.id,
                selected_screen.display_info.width,
                selected_screen.display_info.height
            ));

            if ui.button("Capture Screenshot").clicked() {
                self.take_screenshot(&self.screens[index]);
            }
        }

        if ui.button("Back").clicked() {
            self.main_menu = Menu::Main;
        }
    }

    fn take_screenshot(&self, screen: &Screen) {
        if let Ok(image) = screen.capture() {
            let default_filename = format!("screenshot-{}.png", screen.display_info.id);
            let filename = if self.save_path.is_empty() {
                default_filename
            } else {
                self.save_path.clone()
            };

            let img_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
                image.width(),
                image.height(),
                image.as_raw().to_vec(),
            );

            if let Some(buffer) = img_buffer {
                if let Err(e) = buffer.save(&filename) {
                    Notification::new()
                        .summary("Screenshot Error")
                        .body(&format!("Unable to save screenshot: {}", e))
                        .show()
                        .unwrap();
                } else {
                    Notification::new()
                        .summary("Screenshot Saved!")
                        .body(&format!("File saved as {}", filename))
                        .show()
                        .unwrap();
                }
            } else {
                Notification::new()
                    .summary("Screenshot Error")
                    .body("Error during ImageBuffer creation!")
                    .show()
                    .unwrap();
            }
        }
    }

    fn show_info(&mut self, ui: &mut egui::Ui) {
        ui.heading("Information");
        ui.label("This application allows you to capture screenshots on Wayland.");

        if ui.button("License").clicked() {
            self.main_menu = Menu::License;
        }
        if ui.button("Contributors").clicked() {
            self.main_menu = Menu::Contrib;
        }
        
        if ui.button("Back").clicked() {
            self.main_menu = Menu::Main;
        }
    }

    fn show_license(&mut self, ui: &mut egui::Ui) {
        ui.heading("License");
        ui.label("Software released under the MIT license.");
        if ui.button("Back").clicked() {
            self.main_menu = Menu::Info;
        }
    }

    fn show_contrib(&mut self, ui: &mut egui::Ui) {
        ui.heading("Contributors");
        ui.label("Open-source project, contributions welcome!");
        if ui.button("Back").clicked() {
            self.main_menu = Menu::Info;
        }
    }
}

// eGUI application implementation
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.main_menu {
                Menu::Main => self.show_main_menu(ui),
                Menu::FullScreen => self.show_full_screen(ui),
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
        "Sabbishot Capture Tool",
        native_options,
        Box::new(|_cc| Box::new(App::new())),
    )
}
