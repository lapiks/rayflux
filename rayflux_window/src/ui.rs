use crate::app::AppContext;

/// Gives access to gui rendering
pub trait UiFeature {
    fn ui(&mut self, ctx: &egui::Context, app_ctx: &AppContext);
}

#[derive(Default)]
pub struct UserInterface {
    features: Vec<Box<dyn UiFeature>>,
}

impl UserInterface {
    pub fn init(&mut self) {
        // Register engine ui systems
        self.register(Box::new(Performances::default()));
    }

    /// Register a feature UI
    pub fn register(&mut self, feature: Box<dyn UiFeature>) {
        self.features.push(feature);
    }

    /// Execute all features UI
    pub fn run_ui(&mut self, ctx: &egui::Context, app_ctx: &AppContext) {
        for feature in self.features.iter_mut() {
            feature.ui(ctx, app_ctx);
        }
    }
}

#[derive(Default)]
pub struct Performances {}

impl UiFeature for Performances {
    fn ui(&mut self, ctx: &egui::Context, app_ctx: &AppContext) {
        egui::Window::new("Performances")
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 10.0)) // flottant en haut à droite
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .show(ctx, |ui| {
                ui.label(format!("FPS: {:.1}", app_ctx.time.fps()));
            });
    }
}