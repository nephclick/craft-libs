//! The Presets and Color LUT sections of the side column: looks the app keeps, applied as settings.
//!
//! The app owns the looks: it hands the workspace the presets (built-in and the user's) and the names of the LUTs in its
//! library ([`RawLooks`]), and answers the events the sections raise (save a preset, delete one, load a `.cube` file).
//! A picked preset replaces the settings as one history step; the LUT and its amount are ordinary settings fields.

use craft_behavior::EditEvents;
use craft_theme::TypeRole;
use craft_ui::buttons::Button;
use craft_ui::contract::{Bind, Entry, Props as _, Show as _, ShowContainer as _, clicked};
use craft_ui::display::Text;
use craft_ui::form::SliderField;
use craft_ui::layout_components::Row;
use craft_ui::pickers::Select;
use craft_ui::runtime::env::{Env, Label};
use craft_ui::text::TextField;
use egui::Ui;

use super::controls::lift;
use super::words::RawDevelopWords;
use crate::raw::{MAX_LUT_NAME, RawDevelopSettings};

/// Longest preset name the name field hands on, in characters.
const MAX_PRESET_NAME: usize = 80;

/// A named develop preset.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RawPreset {
    /// The name the preset button shows.
    pub name: String,
    /// The settings the preset applies.
    pub settings: RawDevelopSettings,
    /// Whether the app ships the preset: a built-in preset can't be deleted.
    pub builtin: bool,
}

impl RawPreset {
    /// A preset named `name` that applies `settings`.
    pub fn new(name: impl Into<String>, settings: RawDevelopSettings, builtin: bool) -> Self {
        RawPreset { name: name.into(), settings, builtin }
    }
}

/// The looks the app offers the workspace: its presets and the names of the LUTs in its library.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct RawLooks {
    /// Built-in presets first, then the user's.
    pub presets: Vec<RawPreset>,
    /// The names of the LUTs the develop can apply.
    pub luts: Vec<String>,
}

impl RawLooks {
    /// Looks of `presets` and the LUTs named `luts`.
    pub fn new(presets: Vec<RawPreset>, luts: Vec<String>) -> Self {
        RawLooks { presets, luts }
    }
}

/// What the looks sections asked for this pass.
#[derive(Default)]
pub(super) struct LooksPass {
    /// Edits of the LUT and its amount.
    pub(super) edit: EditEvents<RawDevelopSettings>,
    /// The index of the preset picked.
    pub(super) preset: Option<usize>,
    /// The name of a preset to save from the settings.
    pub(super) save: Option<String>,
    /// The name of a user preset to delete.
    pub(super) delete: Option<String>,
    /// Load LUT was pressed.
    pub(super) import_lut: bool,
}

/// The Presets panel: a search field, then every preset in a list (the app's, then the user's with Delete), then
/// the row that saves the settings as a new preset. `name` is the new preset's name, `search` the search text.
pub(super) fn presets_panel(ui: &mut Ui, looks: &RawLooks, name: &mut String, search: &mut String) -> LooksPass {
    let words = Env::of(ui).words::<RawDevelopWords>();
    let mut pass = LooksPass::default();
    TextField::search(words.search_presets.clone(), Bind::bound(&mut *search)).show(ui);
    let query = search.trim().to_lowercase();
    let matches = |preset: &RawPreset| query.is_empty() || preset.name.to_lowercase().contains(&query);
    let width = ui.available_width();
    let list = |ui: &mut Ui, title: &Label, builtin: bool, pass: &mut LooksPass| {
        let shown: Vec<(usize, &RawPreset)> = looks.presets.iter().enumerate().filter(|(_, p)| p.builtin == builtin && matches(p)).collect();
        ui.add_space(6.0);
        Text::new(title.clone()).role(TypeRole::Caption).show(ui);
        if shown.is_empty() && !builtin && query.is_empty() {
            Text::new(words.no_user_presets.clone()).role(TypeRole::Caption).show(ui);
        }
        for (at, preset) in shown {
            ui.horizontal(|ui| {
                let row = if builtin { width } else { (width - 34.0).max(60.0) };
                if ui.add_sized([row, 24.0], egui::Button::new(preset.name.as_str()).frame(true)).clicked() {
                    pass.preset = Some(at);
                }
                if !builtin && ui.add_sized([26.0, 24.0], egui::Button::new("×")).on_hover_text(words.delete_preset.as_str()).clicked() {
                    pass.delete = Some(preset.name.clone());
                }
            });
        }
    };
    list(ui, &words.builtin_presets, true, &mut pass);
    list(ui, &words.user_presets, false, &mut pass);
    ui.add_space(8.0);
    Row::new().fill_width().show(ui, |ui, _| {
        TextField::new(words.preset_name.clone(), Bind::bound(&mut *name)).placeholder(words.preset_name.clone()).show(ui);
        if clicked(&Button::new(words.save_preset.clone()).id_salt("raw-preset-save").show(ui)) {
            let trimmed: String = name.trim().chars().take(MAX_PRESET_NAME).collect();
            if !trimmed.is_empty() {
                pass.save = Some(trimmed);
            }
        }
    });
    if pass.save.is_some() {
        name.clear();
    }
    pass
}

/// The Color LUT panel over `settings`: the LUT, its amount, and Load LUT.
pub(super) fn lut_panel(ui: &mut Ui, looks: &RawLooks, settings: &RawDevelopSettings) -> LooksPass {
    let words = Env::of(ui).words::<RawDevelopWords>();
    let mut pass = LooksPass::default();
    let mut entries = vec![Entry::new(String::new(), words.lut_none.clone())];
    entries.extend(looks.luts.iter().filter(|lut| lut.len() <= MAX_LUT_NAME).map(|lut| Entry::new(lut.clone(), Label::from(lut.clone()))));
    // A LUT the settings name that the library no longer holds still shows, so it can be changed.
    if !settings.lut.is_empty() && !looks.luts.contains(&settings.lut) {
        entries.push(Entry::new(settings.lut.clone(), Label::from(settings.lut.clone())));
    }
    let events = Select::new(words.lut.clone(), Bind::given(settings.lut.clone()), &entries).show(ui).events;
    pass.edit.extend(lift(events, settings, &|s: &mut RawDevelopSettings, lut: String| s.lut = lut));
    let amount = SliderField::new(words.lut_amount.clone(), Bind::given(f64::from(settings.lut_amount)).reset(100.0), 0.0..=100.0).step(1.0);
    let events = amount.show(ui).edit.events;
    pass.edit.extend(lift(events, settings, &|s: &mut RawDevelopSettings, v: f64| s.lut_amount = v as f32));
    if clicked(&Button::new(words.load_lut.clone()).id_salt("raw-load-lut").show(ui)) {
        pass.import_lut = true;
    }
    pass
}
