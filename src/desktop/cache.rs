use crate::desktop::entry::DesktopEntry;
use crate::desktop::scanner::scan_applications;
use crate::ui::icon::resolve_icon_path;

/// Resolve icon paths for all entries
fn resolve_all_icon_paths(entries: &mut [DesktopEntry]) {
    for entry in entries.iter_mut() {
        if entry.icon_path.is_none() {
            entry.icon_path = entry.icon.as_ref().and_then(|name| resolve_icon_path(name));
        }
    }
}

pub fn load_applications() -> Vec<DesktopEntry> {
    let mut entries = scan_applications();
    resolve_all_icon_paths(&mut entries);
    entries
}
