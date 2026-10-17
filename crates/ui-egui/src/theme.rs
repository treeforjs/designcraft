//! Design tokens (the shared craft-ui System / Light / Cream / Dim / Dark / High contrast
//! appearances), fonts
//! and egui style. The look follows treeforjs/craftworks DESIGN.md, not InDesign's.

use std::sync::Arc;

use egui::{Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Stroke, TextStyle, Visuals};
use serde::{Deserialize, Serialize};

/// The interface appearance: the shared craft-ui choices every craft app offers (System, Light,
/// Cream, Dim, Dark, High contrast). InDesign's four brightness levels are gone; older settings and
/// scripts that name them read as the nearest of these (Medium Dark is Dim, Medium Light is Light).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Brightness {
    /// Follow the operating system's light or dark setting.
    #[default]
    System,
    /// Off-white surfaces, dark text.
    #[serde(alias = "MediumLight")]
    Light,
    /// Warm paper surfaces with a red accent: light, but easier on the eyes than white.
    Cream,
    /// Mid-grey surfaces, light text: between Light and Dark.
    #[serde(alias = "MediumDark")]
    Dim,
    Dark,
    /// Accessibility: black chrome, white text, yellow selection and focus.
    HighContrast,
}

impl Brightness {
    pub const ALL: [Brightness; 6] =
        [Brightness::System, Brightness::Light, Brightness::Cream, Brightness::Dim, Brightness::Dark, Brightness::HighContrast];
    pub fn label(self) -> &'static str {
        self.craft().label()
    }
    pub fn id(self) -> &'static str {
        self.craft().id()
    }
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        if let Some(b) = Self::ALL.into_iter().find(|b| b.id().eq_ignore_ascii_case(s) || b.label().eq_ignore_ascii_case(s)) {
            return Some(b);
        }
        // The retired InDesign brightness levels.
        match s.to_ascii_lowercase().replace([' ', '-', '_'], "").as_str() {
            "mediumdark" => Some(Brightness::Dim),
            "mediumlight" => Some(Brightness::Light),
            _ => craft_ui::Appearance::parse(s).map(Self::from_craft),
        }
    }
    fn craft(self) -> craft_ui::Appearance {
        match self {
            Brightness::System => craft_ui::Appearance::System,
            Brightness::Light => craft_ui::Appearance::Light,
            Brightness::Cream => craft_ui::Appearance::Cream,
            Brightness::Dim => craft_ui::Appearance::Dim,
            Brightness::Dark => craft_ui::Appearance::Dark,
            Brightness::HighContrast => craft_ui::Appearance::HighContrast,
        }
    }
    fn from_craft(a: craft_ui::Appearance) -> Self {
        match a {
            craft_ui::Appearance::System => Brightness::System,
            craft_ui::Appearance::Light => Brightness::Light,
            craft_ui::Appearance::Cream => Brightness::Cream,
            craft_ui::Appearance::Dim => Brightness::Dim,
            craft_ui::Appearance::Dark => Brightness::Dark,
            craft_ui::Appearance::HighContrast => Brightness::HighContrast,
        }
    }
    /// The colour mode this choice draws in, given the desktop's theme. A page-layout app
    /// prefers light when the desktop says nothing.
    pub fn mode(self, system: Option<egui::Theme>) -> craft_ui::Mode {
        self.craft().resolve(system, false)
    }
}

/// Every colour the UI uses. Widgets never hard-code colours.
#[derive(Clone, Copy, Debug)]
pub struct Tokens {
    /// The craft-ui mode these tokens draw.
    pub mode: craft_ui::Mode,
    /// Dark surfaces (Dim, Dark, High contrast); Light and Cream are light.
    pub dark: bool,
    pub app_bar: Color32,
    pub panel: Color32,
    pub panel_darker: Color32,
    pub input: Color32,
    pub input_border: Color32,
    pub divider: Color32,
    pub text: Color32,
    pub text_strong: Color32,
    pub text_dim: Color32,
    pub text_disabled: Color32,
    pub icon: Color32,
    pub hover: Color32,
    pub tool_active: Color32,
    pub accent: Color32,
    pub accent_strong: Color32,
    pub row_selected: Color32,
    pub pasteboard: Color32,
    pub ruler: Color32,
    pub ruler_tick: Color32,
    pub tab_inactive: Color32,
    pub button: Color32,
    pub radius: u8,
    /// 1 pt chrome edges.
    pub border: Color32,
    /// Outline of fields and buttons.
    pub field_border: Color32,
    /// Active tool / toggle well.
    pub well: Color32,
    pub well_rim: Color32,
    pub ruler_text: Color32,
    pub section_divider: Color32,
    /// Panel tab strips, doc-tab bar, dock headers.
    pub tab_strip: Color32,
    /// Smart guides and spacing marks drawn over the page.
    pub smart_guide: Color32,
    /// Measurement labels on the canvas (smart dimensions, gaps).
    pub measure_bg: Color32,
    pub measure_text: Color32,
}

fn hex(s: u32) -> Color32 {
    Color32::from_rgb((s >> 16) as u8, (s >> 8) as u8, s as u8)
}

impl Tokens {
    /// The tokens for `b`, with System resolved as when the desktop reports nothing (see
    /// [`Tokens::for_mode`]).
    pub fn for_brightness(b: Brightness) -> Self {
        Self::for_mode(b.mode(None))
    }

    /// DesignCraft's tokens for one of the shared craft-ui modes, so it matches the other apps.
    pub fn for_mode(mode: craft_ui::Mode) -> Self {
        let c = craft_ui::Tokens::new(mode, craft_ui::Density::Compact);
        Tokens {
            mode,
            dark: mode.is_dark(),
            app_bar: c.bg,
            panel: c.surface,
            panel_darker: c.bg,
            input: c.field,
            input_border: c.border,
            divider: c.divider,
            text: c.text,
            text_strong: c.text,
            text_dim: c.text_muted,
            text_disabled: c.text_disabled,
            icon: c.icon,
            hover: c.hover,
            tool_active: c.pressed,
            accent: c.accent,
            accent_strong: c.accent_hover,
            row_selected: c.accent_soft,
            pasteboard: c.bg,
            ruler: c.surface,
            ruler_tick: c.text_faint,
            tab_inactive: c.bg,
            button: c.surface_raised,
            radius: craft_ui::tokens::radius::SM,
            border: c.divider,
            field_border: c.border,
            well: c.accent_soft,
            well_rim: c.accent,
            ruler_text: c.text_muted,
            section_divider: c.divider,
            tab_strip: c.bg,
            // Page overlays keep their own hues: they must read on any artwork.
            smart_guide: hex(0x00c853),
            measure_bg: Color32::from_rgba_unmultiplied(70, 70, 70, 230),
            measure_text: Color32::WHITE,
        }
    }
}

/// Install the UI fonts for the interface language `lang` (it orders the CJK fallbacks).
pub fn install_fonts(ctx: &egui::Context, lang: &str) {
    ctx.set_fonts(font_definitions(designcraft_fonts::CRAFT_FONTS, lang));
}

/// The UI fonts: the app's own, then the craft-fonts faces from `craft` (empty without
/// `CRAFT_FONTS_DIR`) as fallbacks at the end of every family: Japanese (BIZ UDPGothic first) and
/// Simplified Chinese, in the interface language's order, then Arabic. The `arabic` families
/// put the Arabic face first for right-to-left runs. egui has no system-font discovery, so
/// without craft-fonts CJK and Arabic UI text has no glyphs.
fn font_definitions(craft: &'static [designcraft_fonts::CraftFont], lang: &str) -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    let add = |fonts: &mut FontDefinitions, name: &str, data: &'static [u8]| {
        fonts.font_data.insert(name.into(), Arc::new(FontData::from_static(data)));
    };
    add(&mut fonts, "ui", include_bytes!("../../../assets/fonts/SourceSans3-Regular.ttf"));
    add(&mut fonts, "ui-semibold", include_bytes!("../../../assets/fonts/SourceSans3-Semibold.ttf"));
    add(&mut fonts, "mono", include_bytes!("../../../assets/fonts/JetBrainsMono-Regular.ttf"));
    fonts.families.entry(FontFamily::Proportional).or_default().insert(0, "ui".into());
    fonts.families.entry(FontFamily::Monospace).or_default().insert(0, "mono".into());
    fonts.families.insert(FontFamily::Name("semibold".into()), vec!["ui-semibold".into(), "ui".into()]);
    let pick = |script: &str| -> Vec<&designcraft_fonts::CraftFont> { craft.iter().filter(|f| f.scripts.contains(&script)).collect() };
    let mut japanese = pick("Jpan");
    // BIZ UDPGothic (the UI face) first, Regular before Bold; the semibold family prefers Bold.
    japanese.sort_by_key(|f| (f.family != "BIZ UDPGothic", f.style != "Regular"));
    let chinese = pick("Hans");
    let arabic = pick("Arab");
    let name = |f: &designcraft_fonts::CraftFont| format!("craft-{}-{}", f.family, f.style);
    for f in japanese.iter().chain(&chinese).chain(&arabic) {
        add(&mut fonts, &name(f), f.bytes);
    }
    // Han characters take the forms of the interface language: Chinese faces first for zh.
    let zh = lang.starts_with("zh");
    for (family, stack) in fonts.families.iter_mut() {
        let bold = *family == FontFamily::Name("semibold".into());
        let mut ja = japanese.clone();
        if bold {
            ja.sort_by_key(|f| (f.family != "BIZ UDPGothic", f.style != "Bold"));
        }
        let (first, second) = if zh { (&chinese, &ja) } else { (&ja, &chinese) };
        stack.extend(first.iter().chain(second).chain(&arabic).map(|f| name(f)));
    }
    // Arabic runs (rtl.rs): the Arabic face first, so letters and spaces shape as one run.
    for family in ["arabic", "arabic-semibold"] {
        let ui = if family == "arabic" { "ui" } else { "ui-semibold" };
        let stack = arabic.iter().map(|f| name(f)).chain([ui.to_string(), "ui".to_string()]).chain(japanese.iter().chain(&chinese).map(|f| name(f)));
        let mut seen = std::collections::HashSet::new();
        fonts.families.insert(FontFamily::Name(family.into()), stack.filter(|n| seen.insert(n.clone())).collect());
    }
    fonts
}

pub fn semibold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("semibold".into()))
}

pub fn apply(ctx: &egui::Context, t: &Tokens) {
    ctx.data_mut(|d| d.insert_temp(egui::Id::NULL, *t));
    let text_styles: std::collections::BTreeMap<TextStyle, FontId> = [
        (TextStyle::Small, FontId::proportional(10.0)),
        (TextStyle::Body, FontId::proportional(11.0)),
        (TextStyle::Button, FontId::proportional(11.0)),
        (TextStyle::Heading, FontId::new(15.0, FontFamily::Name("semibold".into()))),
        (TextStyle::Monospace, FontId::monospace(11.5)),
    ]
    .into();
    let mut v = if t.dark { Visuals::dark() } else { Visuals::light() };
    let r = CornerRadius::same(t.radius);
    v.panel_fill = t.panel;
    v.window_fill = t.panel;
    v.extreme_bg_color = t.input;
    v.faint_bg_color = t.panel_darker;
    v.window_stroke = Stroke::new(1.0, t.divider);
    v.window_corner_radius = CornerRadius::same(craft_ui::tokens::radius::XL);
    v.menu_corner_radius = CornerRadius::same(craft_ui::tokens::radius::LG);
    v.selection.bg_fill = t.accent_strong;
    v.selection.stroke = Stroke::new(1.0, t.text_strong);
    v.hyperlink_color = t.accent;
    v.override_text_color = Some(t.text);
    for (w, fill) in [
        (&mut v.widgets.noninteractive, t.panel),
        (&mut v.widgets.inactive, t.button),
        (&mut v.widgets.hovered, t.hover),
        (&mut v.widgets.active, t.tool_active),
        (&mut v.widgets.open, t.hover),
    ] {
        w.bg_fill = fill;
        w.weak_bg_fill = fill;
        w.corner_radius = r;
        w.fg_stroke = Stroke::new(1.0, t.text);
    }
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, t.divider);
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, t.field_border);
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, t.field_border);
    v.widgets.active.bg_stroke = Stroke::new(1.0, t.field_border);
    // Outlined buttons on the panel surface.
    v.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    v.widgets.inactive.bg_fill = t.input;
    v.popup_shadow = egui::epaint::Shadow { offset: [0, 4], blur: 12, spread: 0, color: Color32::from_black_alpha(90) };
    ctx.set_visuals(v);
    ctx.global_style_mut(|s| {
        s.text_styles = text_styles.clone();
        s.spacing.item_spacing = egui::vec2(6.0, 4.0);
        s.spacing.button_padding = egui::vec2(6.0, 2.0);
        s.spacing.interact_size = egui::vec2(20.0, 21.0);
        s.spacing.menu_margin = egui::Margin::same(4);
        s.animation_time = 0.06;
    });
}

impl Tokens {
    /// Tokens stored by [`apply`].
    pub fn get(ctx: &egui::Context) -> Tokens {
        ctx.data(|d| d.get_temp::<Tokens>(egui::Id::NULL)).unwrap_or_else(|| Tokens::for_brightness(Brightness::Light))
    }
}

#[cfg(test)]
mod appearance_tests {
    use super::*;

    /// The colours are craft-ui's, so DesignCraft matches the other craft apps; System follows
    /// the desktop, and the retired InDesign levels still load.
    #[test]
    fn tokens_come_from_craft_ui_and_legacy_names_load() {
        for b in Brightness::ALL {
            assert_eq!(Brightness::parse(b.id()), Some(b));
            assert_eq!(Brightness::parse(b.label()), Some(b));
        }
        let dark = craft_ui::Tokens::new(craft_ui::Mode::Dark, craft_ui::Density::Compact);
        let t = Tokens::for_brightness(Brightness::Dark);
        assert_eq!((t.panel, t.accent, t.text, t.dark), (dark.surface, dark.accent, dark.text, true));
        assert!(!Tokens::for_brightness(Brightness::Light).dark && Tokens::for_brightness(Brightness::HighContrast).dark);
        assert_eq!(Brightness::System.mode(Some(egui::Theme::Dark)), craft_ui::Mode::Dark);
        assert_eq!(Brightness::System.mode(None), craft_ui::Mode::Light);
        assert_eq!(Brightness::parse("mediumDark"), Some(Brightness::Dim));
        assert_eq!(Brightness::parse("Medium Light"), Some(Brightness::Light));
        assert_eq!(serde_json::from_str::<Brightness>("\"MediumDark\"").ok(), Some(Brightness::Dim));
        assert_eq!(serde_json::from_str::<Brightness>("\"MediumLight\"").ok(), Some(Brightness::Light));
        assert_eq!(Brightness::parse("purple"), None);
        // The control channel and MCP spell High contrast in camelCase.
        assert_eq!(Brightness::parse("highContrast"), Some(Brightness::HighContrast));
    }

    /// Cream and Dim are offered in craft-ui's order, round-trip through ids, labels and serde,
    /// and Cream draws as a light theme (Dim as a dark one).
    #[test]
    fn cream_and_dim_are_offered_and_cream_is_light() {
        let ids: Vec<&str> = Brightness::ALL.iter().map(|b| b.id()).collect();
        assert_eq!(ids, ["system", "light", "cream", "dim", "dark", "high-contrast"]);
        let labels: Vec<&str> = Brightness::ALL.iter().map(|b| b.label()).collect();
        assert_eq!(labels, ["System", "Light", "Cream", "Dim", "Dark", "High contrast"]);
        for b in Brightness::ALL {
            let json = serde_json::to_string(&b).unwrap();
            assert_eq!(serde_json::from_str::<Brightness>(&json).ok(), Some(b));
        }
        assert_eq!(Brightness::parse("cream"), Some(Brightness::Cream));
        assert_eq!(Brightness::parse("Dim"), Some(Brightness::Dim));
        assert_eq!(Brightness::parse("sepia"), Some(Brightness::Cream));
        assert_eq!(Brightness::parse("paper"), Some(Brightness::Cream));
        assert_eq!(Brightness::parse("grey"), Some(Brightness::Dim));
        assert_eq!(Brightness::parse("medium"), Some(Brightness::Dim));
        let cream = Tokens::for_brightness(Brightness::Cream);
        assert_eq!(cream.mode, craft_ui::Mode::Cream);
        assert!(!cream.dark, "Cream is a light theme");
        let craft = craft_ui::Tokens::new(craft_ui::Mode::Cream, craft_ui::Density::Compact);
        assert_eq!((cream.panel, cream.accent, cream.text), (craft.surface, craft.accent, craft.text));
        let dim = Tokens::for_brightness(Brightness::Dim);
        assert!(dim.dark && dim.mode == craft_ui::Mode::Dim);
        assert_ne!(dim.panel, Tokens::for_brightness(Brightness::Dark).panel);
    }
}

#[cfg(test)]
mod japanese_font_tests {
    fn families() -> [egui::FontFamily; 3] {
        [egui::FontFamily::Proportional, egui::FontFamily::Monospace, egui::FontFamily::Name("semibold".into())]
    }

    fn ctx_with(craft: &'static [designcraft_fonts::CraftFont]) -> egui::Context {
        let ctx = egui::Context::default();
        ctx.set_fonts(super::font_definitions(craft, "ja"));
        let mut output = ctx.run_ui(egui::RawInput::default(), |_| {});
        output.textures_delta.clear();
        ctx
    }

    #[test]
    fn japanese_ui_glyphs_are_available_in_every_family() {
        if designcraft_fonts::CRAFT_FONTS.is_empty() {
            eprintln!("skipped: built without craft-fonts (set CRAFT_FONTS_DIR to a checkout)");
            return;
        }
        let ctx = ctx_with(designcraft_fonts::CRAFT_FONTS);
        ctx.fonts_mut(|fonts| {
            for family in families() {
                let font = egui::FontId::new(13.0, family);
                // Real glyphs (no tofu) for every character.
                for ch in "日本語の文字縦書き横書き組み方向ルビ圏点".chars() {
                    assert!(fonts.has_glyph(&font, ch), "missing {ch} in {font:?}");
                }
            }
        });
        // The UI face is BIZ UDPGothic, ahead of the Mincho faces.
        let defs = super::font_definitions(designcraft_fonts::CRAFT_FONTS, "ja");
        let stack = &defs.families[&egui::FontFamily::Proportional];
        assert_eq!(stack.iter().find(|n| n.starts_with("craft-")).map(String::as_str), Some("craft-BIZ UDPGothic-Regular"));
    }

    #[test]
    fn ui_works_without_craft_fonts() {
        let ctx = ctx_with(&[]);
        ctx.fonts_mut(|fonts| {
            // (egui's has_glyph reports false for characters of the face that also supplies the
            // replacement glyph, as Source Sans Semibold does in the semibold stack.)
            let body = egui::FontId::new(13.0, egui::FontFamily::Proportional);
            assert!(fonts.has_glyphs(&body, "DesignCraft"));
            for family in families() {
                let font = egui::FontId::new(13.0, family);
                let galley = fonts.layout_no_wrap("日本語 DesignCraft".into(), font, egui::Color32::WHITE);
                assert!(galley.size().x > 0.0);
            }
        });
        // And the real installer works with whatever this build has.
        super::install_fonts(&egui::Context::default(), "ja");
    }
}

#[cfg(test)]
mod ukrainian_font_tests {
    #[test]
    fn every_catalog_cyrillic_glyph_exists_in_the_bundled_ui_fonts() {
        // Check the actual installed face data, without craft-fonts or system fonts.
        // egui's has_glyph can report false for the face providing its replacement glyph.
        let definitions = super::font_definitions(&[], "uk");
        let db = designcraft_fonts::FontDb::with_font_dirs(Vec::new());
        db.set_system_fallback(false);
        let characters: std::collections::HashSet<_> = crate::i18n::ukrainian_catalog()
            .iter()
            .flat_map(|text| text.chars())
            .chain("ҐґЄєІіЇї".chars())
            .filter(|c| matches!(*c, '\u{0400}'..='\u{052f}'))
            .collect();
        for (name, family, style) in
            [("ui", "Source Sans 3", "Regular"), ("ui-semibold", "Source Sans 3", "Semibold"), ("mono", "JetBrains Mono", "Regular")]
        {
            let data = &definitions.font_data[name].font;
            let face = db.face(family, style);
            assert_eq!(face.data(), data.as_ref(), "configured UI face {name}");
            for character in &characters {
                assert_ne!(face.glyph_for(*character), 0, "missing {character} in {name}");
            }
        }
    }
}
