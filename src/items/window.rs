/// A window item representing an open window for window switching.
/// This is a placeholder for future implementation.
#[derive(Clone, Debug)]
pub struct WindowItem {
    pub id: String,
    pub title: String,
    pub app_id: String,
    pub app_name: String,
    pub icon_path: Option<std::path::PathBuf>,
}

impl WindowItem {
    pub fn new(
        id: String,
        title: String,
        app_id: String,
        app_name: String,
        icon_path: Option<std::path::PathBuf>,
    ) -> Self {
        Self {
            id,
            title,
            app_id,
            app_name,
            icon_path,
        }
    }
}
