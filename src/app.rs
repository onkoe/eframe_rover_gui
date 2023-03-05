use crate::video::video::take_zmq_data;
use crate::zmq_connector::zmq_connector::start_zmq;
use eframe::egui;
use egui::widgets::Button; //, Color32, Frame, Sense, TextStyle, Ui };
use egui_extras::{image, RetainedImage};
use tokio::runtime::{Builder, Runtime};
use tokio::task::spawn_blocking;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
    #[serde(skip)]
    muh_photo: RetainedImage,

    #[serde(skip)]
    runtime: Runtime,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,

            // default image
            muh_photo: RetainedImage::from_image_bytes(
                "no_image_yet.jpeg",
                include_bytes!("no_image_yet.jpeg"),
            )
            .unwrap(),

            // async runtime
            runtime: Builder::new_current_thread().enable_all().build().unwrap(),
            // TODO: PUT ZmqClient up here SO IT DOESNT GET RECREATED EACH FRAME
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            label,
            value,
            muh_photo,
            runtime,
        } = self;

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        // YES FILE AND QUIT ON WEB PAGES YEEEEAH
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    let my_button = ui.add(Button::new("hey"));
                    if my_button.clicked() {}
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("ayo");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("eframe_rover_gui :)");

            // IMAGE STUFF
            ui.heading("This is an image:");
            self.muh_photo.show(ui);

            // TODO: call zmq_connector. get ZmqClient
            //let runtime = Builder::new_current_thread().enable_all().build().unwrap();

            let zmq_bruh = runtime
                .block_on(spawn_blocking(|| start_zmq("127.0.0.1".to_string(), 1234)))
                .unwrap();
            let value = match zmq_bruh {
                Ok(Ok(zmq_guy)) => runtime.block_on(future_result),
                Err(e) => panic!("Error: {:?}", e),
            };

            ui.heading("This is an image you can click:");
            if ui
                .add(egui::ImageButton::new(
                    self.muh_photo.texture_id(ctx),
                    self.muh_photo.size_vec2(),
                ))
                .clicked()
            {
                // TODO...

                // call video each time clicked. get new frame
                // ... make sure to add ZmqClient as parameter

                self.muh_photo = RetainedImage::from_image_bytes(
                    "zmq image",
                    take_zmq_data().unwrap().as_bytes(),
                )
                .unwrap();
            }

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });
    }
}
