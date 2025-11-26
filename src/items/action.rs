/// The kind of action to perform.
/// This is a placeholder for future implementation.
#[derive(Clone, Debug)]
pub enum ActionKind {
    /// Shutdown the system
    Shutdown,
    /// Reboot the system
    Reboot,
    /// Suspend the system
    Suspend,
    /// Lock the screen
    Lock,
    /// Log out of the session
    Logout,
    /// Custom command execution
    Command(String),
}

/// An action item representing a functional command (shutdown, reboot, etc.).
/// This is a placeholder for future implementation.
#[derive(Clone, Debug)]
pub struct ActionItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon_name: Option<String>,
    pub kind: ActionKind,
}

impl ActionItem {
    pub fn new(
        id: String,
        name: String,
        description: Option<String>,
        icon_name: Option<String>,
        kind: ActionKind,
    ) -> Self {
        Self {
            id,
            name,
            description,
            icon_name,
            kind,
        }
    }
}
