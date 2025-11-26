use crate::items::ListItem;
use crate::ui::items::render_item;
use crate::ui::theme::theme;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use gpui::{App, Context, SharedString, Task, Window, div, prelude::*};
use gpui_component::IndexPath;
use gpui_component::list::{ListDelegate, ListItem as GpuiListItem, ListState};
use std::sync::Arc;

/// A generic delegate for displaying and filtering list items.
pub struct ItemListDelegate {
    items: Arc<Vec<ListItem>>,
    filtered_indices: Vec<usize>,
    selected_index: Option<usize>,
    query: String,
    on_confirm: Option<Arc<dyn Fn(&ListItem) + Send + Sync>>,
    on_cancel: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl ItemListDelegate {
    pub fn new(items: Vec<ListItem>) -> Self {
        let len = items.len();

        Self {
            items: Arc::new(items),
            filtered_indices: (0..len).collect(),
            selected_index: if len > 0 { Some(0) } else { None },
            query: String::new(),
            on_confirm: None,
            on_cancel: None,
        }
    }

    /// Set the callback for when an item is confirmed (Enter pressed).
    pub fn set_on_confirm(&mut self, callback: impl Fn(&ListItem) + Send + Sync + 'static) {
        self.on_confirm = Some(Arc::new(callback));
    }

    /// Set the callback for when the list is cancelled (Escape pressed).
    pub fn set_on_cancel(&mut self, callback: impl Fn() + Send + Sync + 'static) {
        self.on_cancel = Some(Arc::new(callback));
    }

    /// Returns the items Arc for use in background filtering.
    pub fn items(&self) -> Arc<Vec<ListItem>> {
        Arc::clone(&self.items)
    }

    /// Filter items on a background thread - returns filtered indices.
    pub fn filter_items_sync(items: &[ListItem], query: &str) -> Vec<usize> {
        if query.is_empty() {
            (0..items.len()).collect()
        } else {
            let matcher = SkimMatcherV2::default();
            let mut scored: Vec<(usize, i64)> = items
                .iter()
                .enumerate()
                .filter_map(|(idx, item)| {
                    matcher
                        .fuzzy_match(item.name(), query)
                        .map(|score| (idx, score))
                })
                .collect();

            scored.sort_by(|a, b| b.1.cmp(&a.1));
            scored.into_iter().map(|(idx, _)| idx).collect()
        }
    }

    /// Apply pre-computed filter results.
    pub fn apply_filter_results(&mut self, query: String, indices: Vec<usize>) {
        // Only apply if query still matches (user might have typed more)
        if self.query == query {
            self.filtered_indices = indices;
            self.selected_index = if self.filtered_indices.is_empty() {
                None
            } else {
                Some(0)
            };
        }
    }

    fn filter_items(&mut self) {
        self.filtered_indices = Self::filter_items_sync(&self.items, &self.query);
        self.selected_index = if self.filtered_indices.is_empty() {
            None
        } else {
            Some(0)
        };
    }

    fn get_item_at(&self, row: usize) -> Option<&ListItem> {
        self.filtered_indices
            .get(row)
            .and_then(|&idx| self.items.get(idx))
    }

    pub fn clear_query(&mut self) {
        self.query.clear();
        self.filter_items();
    }

    pub fn set_query(&mut self, query: String) {
        self.query = query;
        self.filter_items();
    }

    /// Set query without filtering (for async filtering).
    pub fn set_query_only(&mut self, query: String) {
        self.query = query;
    }

    /// Get current query.
    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn filtered_count(&self) -> usize {
        self.filtered_indices.len()
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.selected_index
    }

    pub fn set_selected(&mut self, index: usize) {
        self.selected_index = Some(index);
    }

    pub fn do_confirm(&self) {
        if let Some(idx) = self.selected_index
            && let Some(item) = self.get_item_at(idx)
            && let Some(ref on_confirm) = self.on_confirm
        {
            on_confirm(item);
        }
    }

    pub fn do_cancel(&self) {
        if let Some(ref on_cancel) = self.on_cancel {
            on_cancel();
        }
    }
}

impl ListDelegate for ItemListDelegate {
    type Item = GpuiListItem;

    fn items_count(&self, _section: usize, _cx: &App) -> usize {
        self.filtered_indices.len()
    }

    fn render_item(
        &self,
        ix: IndexPath,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<Self::Item> {
        let item = self.get_item_at(ix.row)?;
        let selected = self.selected_index == Some(ix.row);

        let item_content = render_item(item, selected, ix.row);

        // Reset ListItem default padding - we handle all styling ourselves
        Some(
            GpuiListItem::new(("list-item", ix.row))
                .py_0()
                .px_0()
                .child(item_content),
        )
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        _window: &mut Window,
        _cx: &mut Context<ListState<Self>>,
    ) {
        self.selected_index = ix.map(|i| i.row);
    }

    fn perform_search(
        &mut self,
        query: &str,
        _window: &mut Window,
        _cx: &mut Context<ListState<Self>>,
    ) -> Task<()> {
        self.query = query.to_string();
        self.filter_items();
        Task::ready(())
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        _window: &mut Window,
        _cx: &mut Context<ListState<Self>>,
    ) {
        self.do_confirm();
    }

    fn cancel(&mut self, _window: &mut Window, _cx: &mut Context<ListState<Self>>) {
        self.do_cancel();
    }

    fn render_empty(&self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let t = theme();
        div()
            .w_full()
            .h(t.empty_state_height)
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_sm()
                    .text_color(t.empty_state_color)
                    .child(SharedString::from("No items found")),
            )
    }
}
