use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "nvcontrol",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(NvControlApp)),
    )
}

#[derive(Default)]
struct NvControlApp;

impl eframe::App for NvControlApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("nvcontrol: NVIDIA Control Panel for Linux + Wayland");
            ui.label("This is a placeholder GUI. Implement controls here.");
        });
    }
}
