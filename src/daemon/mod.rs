#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use std::time::Duration;
use std::{fs::File, thread};
use egui::TextStyle;

pub mod registered_template;

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }

    fn table_ui(&mut self, ui: &mut egui::Ui) {
        use egui_extras::{Column, TableBuilder};

        let text_height = egui::TextStyle::Body
            .resolve(ui.style())
            .size
            .max(ui.spacing().interact_size.y);

        let striped = false;
        let resizable = true;

        let table = TableBuilder::new(ui)
        .striped(striped)
        .resizable(resizable)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::initial(100.0))
        .column(Column::initial(100.0))
        .column(Column::initial(100.0));
        // .min_scrolled_height(0.0);

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.vertical_centered( | ui | {
                        ui.heading("Templates")
                        }
                    );
                });
                header.col(|ui| {
                    ui.vertical_centered( | ui | {
                        ui.heading("Core")
                        }
                    );
                });
                header.col(|ui| {
                    ui.vertical_centered( | ui | {
                        ui.heading("Outputs")
                        }
                    );
                });
            }).body(| mut body | {
                body.row(text_height, | mut row | {
                    row.col(| ui | {
                        ui.label("testing");
                    });
                    row.col(| ui | {
                        ui.menu_button("test", | ui| {
                            if ui.button("change").changed() {
                                
                            }
                        });
                    });
                });
                body.row(text_height, | mut row | {
                    row.col(| ui | {
                        ui.label("testing");
                    });
                });
            });

    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            use egui::menu;

            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        // …
                    }
                });
            });

            // ui.vertical(| ui | {
            //     ui.label("testing")
            // });

            let body_text_size = TextStyle::Body.resolve(ui.style()).size;

            use egui_extras::{Size, StripBuilder};

            StripBuilder::new(ui)
                .size(Size::remainder().at_least(100.0)) // for the table
                .size(Size::exact(body_text_size)) // for the source code link
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            self.table_ui(ui);
                        });
                    });
                });
        });
   }
}

pub fn daemon(_args: &super::Args) -> Result<(), Box<dyn std::error::Error>> {
    // // Spawn file manager
    // thread::spawn(check_files);

    // Start the interactive tui
    interactive_ui()?;

    Ok(())
}

pub fn interactive_ui() -> Result<(), Box<dyn std::error::Error>> {
    let native = eframe::NativeOptions::default();
    eframe::run_native("EGUI APP", native, Box::new(|cc| Box::new(MyEguiApp::new(&cc))))?;

    // let raw_input: egui::RawInput = gather_input();
    Ok(())
}

pub fn check_files() -> Result<(), Box<dyn std::error::Error + Send>> {
    let time = Duration::from_millis(5000);

    loop {
        let _test = File::open("templates/CVD/Unity.shader").unwrap();

        thread::sleep(time);
    }
}
