use egui::{scroll_area::State, ColorImage, ImageData, RichText, Color32, Vec2, Ui, Context};
use egui_extras::RetainedImage;
use webbrowser::{self, BrowserOptions};
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state


pub struct PersonalWeb {
    // Example stuff:
    state_index: i32,
    #[serde(skip)]
    images: Images
}

impl Default for PersonalWeb {
    fn default() -> Self {
        Self {
            state_index: 1,
            images: Images::default(),
        }
    }
}


struct Images { //Images that are seen throughout the app
    image_01: Option<egui_extras::RetainedImage>,
    image_02: Option<egui_extras::RetainedImage>,
    image_03: Option<egui_extras::RetainedImage>,
}



impl Default for Images {
    fn default() -> Self {
        return Self {
            image_01: Some(egui_extras::image::RetainedImage::from_image_bytes("test", include_bytes!("../assets/icon-256.png")).unwrap()),
            image_02: Some(egui_extras::image::RetainedImage::from_image_bytes("image2", include_bytes!("../assets/hort.png")).unwrap()),
            image_03: Some(egui_extras::image::RetainedImage::from_image_bytes("image2", include_bytes!("../assets/git.png")).unwrap()),
        };
    }
}


impl PersonalWeb {
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

impl eframe::App for PersonalWeb {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let Self { images } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("State1").clicked() {
                        self.state_index = 1;
                    }
                    if ui.button("State2").clicked() {
                        self.state_index = 2;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new("Always do CTRL + F5!").italics().underline().strong().color(Color32::RED));
                    });
                });


                egui::warn_if_debug_build(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.state_index {
                1 => {
                    ui.vertical_centered(|ui| ui.heading("Welcome"));
                    
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                // if ui.add(egui::ImageButton::new(self.images.image_03.as_ref()
                                //     .unwrap()
                                //     .texture_id(ctx), Vec2::new(50.0, 50.0)))
                                //     .on_hover_text_at_pointer("Enter my github")
                                //     .clicked() {
                                //         if webbrowser::open("https://github.com/RadsammyT?tab=repositories").is_ok() {};
                                
                                
                                //     }
                                ez_button(ui, ctx, &self.images.image_03, "My Github!", "https://github.com/RadsammyT?tab=repositories", 
                                Vec2::new(100.0, 100.0));
                            });
                        });
                    
                }
                
                2 => {
                    self.images.image_01.as_ref().unwrap().show(ui);
                    self.images.image_02.as_ref().unwrap().show(ui);
                }

                _ => {
                    ui.label(RichText::new(format!("Invalid state {} \nIf you got here after refreshing, do CTRL + F5 to force reload. \nOtherwise clear out your cookies/cache for this website. ", self.state_index)).color(Color32::RED));
                }
            }

        }); 

        

    }
}

pub fn ez_button(ui: &mut Ui, ctx: &egui::Context, image: &Option<RetainedImage>, text: &str, link: &str, vec: Vec2) {
    if ui.add(egui::ImageButton::new(image.as_ref()
    .unwrap()
    .texture_id(ctx), vec))
    .on_hover_text_at_pointer(text)
    .clicked() {
        if webbrowser::open(link).is_ok() {};


    }
}