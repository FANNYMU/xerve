use eframe::egui::viewport::IconData;

pub fn load_icon_from_file(path: &str) -> Option<IconData> {
    let image = image::open(path).ok()?.into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    Some(IconData { rgba, width, height })
}