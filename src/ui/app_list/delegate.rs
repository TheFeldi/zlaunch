use crate::desktop::{DesktopEntry, launch_application};
use crate::ui::app_list::item::render_app_item;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use gpui::{App, Context, SharedString, Task, Window, div, prelude::*, px, rgba};
use gpui_component::IndexPath;
use gpui_component::list::{ListDelegate, ListItem, ListState};
use std::sync::Arc;

pub struct AppListDelegate {
    entries: Arc<Vec<DesktopEntry>>,
    filtered_indices: Vec<usize>,
    selected_index: Option<usize>,
    query: String,
    on_hide: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl AppListDelegate {
    pub fn new(entries: Vec<DesktopEntry>) -> Self {
        let len = entries.len();

        Self {
            entries: Arc::new(entries),
            filtered_indices: (0..len).collect(),
            selected_index: if len > 0 { Some(0) } else { None },
            query: String::new(),
            on_hide: None,
        }
    }

    pub fn set_on_hide(&mut self, callback: impl Fn() + Send + Sync + 'static) {
        self.on_hide = Some(Arc::new(callback));
    }

    fn request_hide(&self) {
        if let Some(ref on_hide) = self.on_hide {
            on_hide();
        }
    }

    /// Returns the entries Arc for use in background filtering
    pub fn entries(&self) -> Arc<Vec<DesktopEntry>> {
        Arc::clone(&self.entries)
    }

    /// Filter entries on a background thread - returns filtered indices
    pub fn filter_entries_sync(entries: &[DesktopEntry], query: &str) -> Vec<usize> {
        if query.is_empty() {
            (0..entries.len()).collect()
        } else {
            let matcher = SkimMatcherV2::default();
            let mut scored: Vec<(usize, i64)> = entries
                .iter()
                .enumerate()
                .filter_map(|(idx, entry)| {
                    matcher
                        .fuzzy_match(&entry.name, query)
                        .map(|score| (idx, score))
                })
                .collect();

            scored.sort_by(|a, b| b.1.cmp(&a.1));
            scored.into_iter().map(|(idx, _)| idx).collect()
        }
    }

    /// Apply pre-computed filter results
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

    fn filter_entries(&mut self) {
        self.filtered_indices = Self::filter_entries_sync(&self.entries, &self.query);
        self.selected_index = if self.filtered_indices.is_empty() {
            None
        } else {
            Some(0)
        };
    }

    fn get_entry_at(&self, row: usize) -> Option<&DesktopEntry> {
        self.filtered_indices
            .get(row)
            .and_then(|&idx| self.entries.get(idx))
    }

    pub fn clear_query(&mut self) {
        self.query.clear();
        self.filter_entries();
    }

    pub fn set_query(&mut self, query: String) {
        self.query = query;
        self.filter_entries();
    }

    /// Set query without filtering (for async filtering)
    pub fn set_query_only(&mut self, query: String) {
        self.query = query;
    }

    /// Get current query
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

    pub fn do_confirm(&mut self) {
        if let Some(idx) = self.selected_index
            && let Some(entry) = self.get_entry_at(idx)
        {
            let _ = launch_application(entry);
        }
        self.request_hide();
    }

    pub fn do_cancel(&mut self) {
        self.request_hide();
    }
}

impl ListDelegate for AppListDelegate {
    type Item = ListItem;

    fn items_count(&self, _section: usize, _cx: &App) -> usize {
        self.filtered_indices.len()
    }

    fn render_item(
        &self,
        ix: IndexPath,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<Self::Item> {
        let entry = self.get_entry_at(ix.row)?;
        let selected = self.selected_index == Some(ix.row);

        let item_content = render_app_item(entry, selected, ix.row);

        // Reset ListItem default padding - we handle all styling in our own item
        Some(
            ListItem::new(("app-item", ix.row))
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
        self.filter_entries();
        Task::ready(())
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        _window: &mut Window,
        _cx: &mut Context<ListState<Self>>,
    ) {
        if let Some(idx) = self.selected_index
            && let Some(entry) = self.get_entry_at(idx)
        {
            let _ = launch_application(entry);
        }
        self.request_hide();
    }

    fn cancel(&mut self, _window: &mut Window, _cx: &mut Context<ListState<Self>>) {
        self.request_hide();
    }

    fn render_empty(&self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .h(px(200.0))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_sm()
                    .text_color(rgba(0xffffff40))
                    .child(SharedString::from("No applications found")),
            )
    }
}
