use crate::items::ApplicationItem;
use gpui::{Div, Stateful, prelude::*};

use super::base::{item_container, render_action_indicator, render_icon, render_text_content};

/// Render an application item.
pub fn render_application(app: &ApplicationItem, selected: bool, row: usize) -> Stateful<Div> {
    let mut item = item_container(row, selected)
        .child(render_icon(app.icon_path.as_ref()))
        .child(render_text_content(
            &app.name,
            app.description.as_deref(),
            selected,
        ));

    if selected {
        item = item.child(render_action_indicator("Open"));
    }

    item
}
