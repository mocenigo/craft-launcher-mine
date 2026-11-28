//! macOS: the header is the window's title bar, drawn by a pane with its own context (`craft_window::panes`)
//! at the platform's scale, so the launcher's zoom leaves it the size of the traffic lights beside it.

use crate::i18n::Lang;
use crate::theme::{self, ThemeChoice};

/// The title bar's pane, with what was last given to its context.
pub(crate) struct TitlePane {
    pane: craft_window::panes::Pane,
    fonts: Option<Option<Lang>>,
    theme: Option<ThemeChoice>,
}

impl TitlePane {
    pub(crate) fn new(window: &egui::Context) -> Self {
        let pane = craft_window::panes::Pane::new(window, "title bar");
        egui_extras::install_image_loaders(pane.context());
        Self { pane, fonts: None, theme: None }
    }

    /// Draws `header` at zoom 1 with the window's fonts and theme, its top left corner at `at` (window points).
    pub(crate) fn show(
        &mut self,
        window: &egui::Context,
        at: egui::Pos2,
        choice: ThemeChoice,
        cjk: Option<Lang>,
        header: impl FnOnce(&mut egui::Ui),
    ) {
        let ctx = self.pane.context();
        if self.fonts != Some(cjk) {
            ctx.set_fonts(theme::font_definitions(cjk));
            self.fonts = Some(cjk);
        }
        if self.theme != Some(choice) {
            theme::apply(ctx, choice);
            self.theme = Some(choice);
        }
        let strip = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(f32::INFINITY, crate::views::HEADER_H));
        self.pane.show(window, 1.0, at, strip, header);
    }
}
