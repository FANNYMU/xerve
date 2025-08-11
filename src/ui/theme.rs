use eframe::egui;


pub const ACCENT: egui::Color32 = egui::Color32::from_rgb(80, 180, 255);
pub const BG_DARK: egui::Color32 = egui::Color32::from_rgb(24, 24, 27);
pub const BG_PANEL: egui::Color32 = egui::Color32::from_rgb(30, 30, 34);
pub const BG_CARD: egui::Color32 = egui::Color32::from_rgb(35, 35, 39);
pub const STROKE: egui::Color32 = egui::Color32::from_rgb(58, 58, 62);
pub const TEXT: egui::Color32 = egui::Color32::from_rgb(230, 230, 230);
pub const TEXT_MUTED: egui::Color32 = egui::Color32::from_rgb(150, 150, 150);

pub const GREEN: egui::Color32 = egui::Color32::from_rgb(46, 160, 67);
pub const RED: egui::Color32 = egui::Color32::from_rgb(220, 53, 69);
pub const BLUE: egui::Color32 = egui::Color32::from_rgb(0, 123, 255);

pub fn apply_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.spacing.button_padding = egui::vec2(16.0, 10.0);
    style.spacing.item_spacing = egui::vec2(10.0, 10.0);
    style.spacing.window_margin = egui::Margin::symmetric(12i8, 12i8);

                    
    style.visuals.widgets.noninteractive.bg_fill = BG_PANEL;
    style.visuals.extreme_bg_color = BG_DARK;
    style.visuals.widgets.inactive.bg_fill = BG_CARD;
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(45, 45, 50);
    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(50, 50, 56);
    style.visuals.widgets.open.bg_fill = egui::Color32::from_rgb(55, 55, 60);
    style.visuals.window_fill = BG_DARK;
    style.visuals.window_stroke = egui::Stroke::new(1.0, STROKE);
    style.visuals.panel_fill = BG_DARK;

    // Slightly brighter text than default dark theme
    style.visuals.override_text_color = Some(TEXT);

    ctx.set_style(style);

    // Text sizes
    let fonts = egui::FontDefinitions::default();
    ctx.set_fonts(fonts);
}

pub fn card_frame(style: &egui::Style) -> egui::Frame {
    egui::Frame::group(style)
        .fill(BG_PANEL)
        .stroke(egui::Stroke::new(1.0, STROKE))
        .corner_radius(12.0)
        .inner_margin(egui::Margin::symmetric(12i8, 12i8))
}

pub fn content_container(ui: &mut egui::Ui, inner: impl FnOnce(&mut egui::Ui)) {
    let avail = ui.available_width();
    let max = avail.min(980.0).max(340.0);
    let pad = ((avail - max) / 2.0).max(0.0);

    ui.horizontal(|ui| {
        if pad > 0.0 { ui.add_space(pad); }
        ui.vertical(|ui| {
            ui.set_width(max);
            inner(ui);
        });
        if pad > 0.0 { ui.add_space(pad); }
    });
}

pub fn status_colors(status: &str) -> (egui::Color32, egui::Color32) {
    match status {
        "Running" => (egui::Color32::WHITE, GREEN),
        "Stopped" => (egui::Color32::WHITE, RED),
        _ => (egui::Color32::WHITE, egui::Color32::GRAY),
    }
}

pub fn subtle_label(ui: &mut egui::Ui, text: impl Into<String>, size: f32) {
    ui.label(
        egui::RichText::new(text.into())
            .size(size)
            .color(TEXT_MUTED),
    );
}
