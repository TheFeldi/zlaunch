use crate::ui::theme::theme;
use gpui::{Div, ElementId, SharedString, Stateful, div, img, prelude::*, px};
use std::path::PathBuf;

/// Create the base container for a list item with selection styling.
pub fn item_container(row: usize, selected: bool) -> Stateful<Div> {
    let theme = theme();

    let bg_color = if selected {
        theme.item_background_selected
    } else {
        theme.item_background
    };

    div()
        .id(ElementId::NamedInteger("list-item".into(), row as u64))
        .mx(theme.item_margin_x)
        .my(theme.item_margin_y)
        .px(theme.item_padding_x)
        .py(theme.item_padding_y)
        .bg(bg_color)
        .rounded(theme.item_border_radius)
        .overflow_hidden()
        .relative()
        .flex()
        .flex_row()
        .items_center()
        .gap_2()
}

/// Render an icon from a file path, with fallback placeholder.
pub fn render_icon(icon_path: Option<&PathBuf>) -> Div {
    let theme = theme();
    let size = theme.icon_size;

    let icon_container = div()
        .w(size)
        .h(size)
        .flex_shrink_0()
        .flex()
        .items_center()
        .justify_center();

    if let Some(path) = icon_path {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if matches!(ext, "png" | "jpg" | "jpeg" | "svg") {
            return icon_container.child(img(path.clone()).w(size).h(size).rounded_sm());
        }
    }

    // Fallback: show a subtle placeholder
    icon_container
        .bg(theme.icon_placeholder_background)
        .rounded_sm()
        .child(
            div()
                .text_sm()
                .text_color(theme.icon_placeholder_color)
                .child(SharedString::from("?")),
        )
}

/// Render the text content (title and optional description).
pub fn render_text_content(name: &str, description: Option<&str>, selected: bool) -> Div {
    let theme = theme();

    let name_element = div()
        .w_full()
        .text_sm()
        .line_height(theme.item_title_line_height)
        .text_color(theme.item_title_color)
        .whitespace_nowrap()
        .overflow_hidden()
        .text_ellipsis()
        .child(SharedString::from(name.to_string()));

    let max_width = theme.max_text_width(selected);

    let mut content = div()
        .h(theme.item_content_height)
        .max_w(max_width)
        .flex()
        .flex_col()
        .justify_center()
        .overflow_hidden();

    content = content.child(name_element);

    if let Some(desc) = description {
        let description_element = div()
            .w_full()
            .text_xs()
            .h(px(18.0)) // Fixed height to fit descenders
            .text_color(theme.item_description_color)
            .whitespace_nowrap()
            .overflow_hidden()
            .text_ellipsis()
            .child(SharedString::from(desc.to_string()));

        content = content.child(description_element);
    }

    content
}

/// Render the action indicator shown on selected items.
pub fn render_action_indicator(label: &str) -> Div {
    let theme = theme();

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
                .text_color(theme.action_label_color)
                .child(SharedString::from(label.to_string())),
        )
        .child(
            // Kbd-style box for Enter key
            div()
                .px(px(4.0))
                .pt(px(2.0))
                .pb(px(1.0))
                .bg(theme.action_key_background)
                .border_1()
                .border_color(theme.action_key_border)
                .rounded(px(3.0))
                .text_size(px(10.0))
                .line_height(px(10.0))
                .text_color(theme.action_key_color)
                .child(SharedString::from("↵")),
        )
}
