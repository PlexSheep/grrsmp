pub(crate) fn version() -> String {
    format!(
        "Version: {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
}
