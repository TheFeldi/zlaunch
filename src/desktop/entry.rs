use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct DesktopEntry {
    pub id: String,
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
    /// Pre-resolved icon path for fast rendering
    pub icon_path: Option<PathBuf>,
    pub comment: Option<String>,
    pub categories: Vec<String>,
    pub terminal: bool,
    pub path: PathBuf,
}

impl DesktopEntry {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: String,
        name: String,
        exec: String,
        icon: Option<String>,
        icon_path: Option<PathBuf>,
        comment: Option<String>,
        categories: Vec<String>,
        terminal: bool,
        path: PathBuf,
    ) -> Self {
        Self {
            id,
            name,
            exec,
            icon,
            icon_path,
            comment,
            categories,
            terminal,
            path,
        }
    }
}
