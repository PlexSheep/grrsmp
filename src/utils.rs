pub(crate) const GUI_SPACING_MID: i32 = 8;
pub(crate) const GUI_SPACING_LARGE: i32 = 12;
pub(crate) const GUI_SPACING_XLARGE: i32 = 16;
pub(crate) const GUI_SPACING_XXLARGE: i32 = 24;
pub(crate) const GUI_SPACING_XXXLARGE: i32 = 32;

pub(crate) fn version() -> String {
    format!(
        "Version: {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
}
