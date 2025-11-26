use crate::desktop::DesktopEntry;
use gpui::{Div, ElementId, SharedString, Stateful, div, img, prelude::*, px, rgba};
use std::path::PathBuf;

const ICON_SIZE: f32 = 24.0;
const ACTION_INDICATOR_WIDTH: f32 = 64.0; // Approximate width for "Open" + kbd box

pub fn render_app_item(entry: &DesktopEntry, selected: bool, row: usize) -> Stateful<Div> {
    // Transparent by default, subtle highlight when selected
    let bg_color = if selected {
        rgba(0xffffff12) // ~7% white overlay
    } else {
        rgba(0x00000000) // fully transparent
    };

    let mut item = div()
        .id(ElementId::NamedInteger("app-item".into(), row as u64))
        .mx_2()
        .my(px(1.0)) // Small margin between items so selection boxes don't overlap
        .px_2()
        .py(px(7.0))
        .bg(bg_color)
        .rounded_md()
        .overflow_hidden()
        .relative()
        .flex()
        .flex_row()
        .items_center()
        .gap_2()
        .child(render_icon(&entry.icon_path))
        .child(render_text_content(
            &entry.name,
            entry.comment.as_deref(),
            selected,
        ));

    if selected {
        item = item.child(render_action_indicator());
    }

    item
}

fn render_icon(icon_path: &Option<PathBuf>) -> Div {
    let icon_container = div()
        .w(px(ICON_SIZE))
        .h(px(ICON_SIZE))
        .flex_shrink_0()
        .flex()
        .items_center()
        .justify_center();

    if let Some(path) = icon_path {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if matches!(ext, "png" | "jpg" | "jpeg" | "svg") {
            return icon_container.child(
                img(path.clone())
                    .w(px(ICON_SIZE))
                    .h(px(ICON_SIZE))
                    .rounded_sm(),
            );
        }
    }

    // Fallback: show a subtle placeholder
    icon_container.bg(rgba(0xffffff0a)).rounded_sm().child(
        div()
            .text_sm()
            .text_color(rgba(0xffffff40))
            .child(SharedString::from("?")),
    )
}

fn render_text_content(name: &str, comment: Option<&str>, selected: bool) -> Div {
    // Fixed height for consistent item sizing: title (16px) + description (18px) = 34px
    let content_height = 34.0;

    let name_element = div()
        .w_full()
        .text_sm()
        .line_height(px(16.0))
        .text_color(rgba(0xffffffE6)) // ~90% white
        .whitespace_nowrap()
        .overflow_hidden()
        .text_ellipsis()
        .child(SharedString::from(name.to_string()));

    // Calculate max width: window(600) - mx_2(16) - px_2(16) - icon(24) - gap(8) - some_buffer(16)
    let max_text_width = if selected {
        600.0 - 16.0 - 16.0 - 24.0 - 8.0 - 16.0 - ACTION_INDICATOR_WIDTH
    } else {
        600.0 - 16.0 - 16.0 - 24.0 - 8.0 - 16.0
    };

    let mut content = div()
        .h(px(content_height))
        .max_w(px(max_text_width))
        .flex()
        .flex_col()
        .justify_center()
        .overflow_hidden();

    content = content.child(name_element);

    // Only render description if present
    if let Some(comment_text) = comment {
        let description_element = div()
            .w_full()
            .text_xs()
            .h(px(18.0)) // Fixed height to fit descenders (p, g, y, etc.)
            .text_color(rgba(0xffffff66)) // ~40% white
            .whitespace_nowrap()
            .overflow_hidden()
            .text_ellipsis()
            .child(SharedString::from(comment_text.to_string()));

        content = content.child(description_element);
    }

    content
}

fn render_action_indicator() -> Div {
    div()
        .absolute()
        .right(px(8.0))
        .top_0()
        .bottom_0()
        .flex()
        .flex_row()
        .items_center()
        .gap_2()
        .child(
            div()
                .text_xs()
                .text_color(rgba(0xffffff66))
                .child(SharedString::from("Open")),
        )
        .child(
            // Kbd-style box
            div()
                .px(px(4.0))
                .pt(px(2.0))
                .pb(px(1.0))
                .bg(rgba(0xffffff10))
                .border_1()
                .border_color(rgba(0xffffff20))
                .rounded(px(3.0))
                .text_size(px(10.0))
                .line_height(px(10.0))
                .text_color(rgba(0xffffff99))
                .child(SharedString::from("↵")),
        )
}
