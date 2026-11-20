//! The panels the strip on the window's right switches between, besides Adjust, Presets and Color LUT: Reshape
//! (perspective, rotation and scale, with Level), Eye Fix (red and animal eyes) and Compare (before and after).

use craft_behavior::EditEvents;
use craft_theme::TypeRole;
use craft_ui::buttons::Button;
use craft_ui::contract::{Bind, Entry, Props as _, Show as _, clicked};
use craft_ui::display::Text;
use craft_ui::form::SliderField;
use craft_ui::pickers::Select;
use craft_ui::runtime::env::{Env, Label};
use egui::{Color32, ColorImage, Ui};

use super::controls::{GEOMETRY, lift};
use super::words::RawDevelopWords;
use crate::raw::RawDevelopSettings;

/// Which panel the side column shows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum RawPanel {
    #[default]
    Adjust,
    Presets,
    Reshape,
    EyeFix,
    Lut,
    Compare,
}

impl RawPanel {
    pub(super) const ALL: [RawPanel; 6] = [RawPanel::Adjust, RawPanel::Presets, RawPanel::Reshape, RawPanel::EyeFix, RawPanel::Lut, RawPanel::Compare];

    pub(super) fn word(self, words: &RawDevelopWords) -> &Label {
        match self {
            RawPanel::Adjust => &words.panel_adjust,
            RawPanel::Presets => &words.panel_presets,
            RawPanel::Reshape => &words.panel_reshape,
            RawPanel::EyeFix => &words.panel_eye_fix,
            RawPanel::Lut => &words.panel_lut,
            RawPanel::Compare => &words.panel_compare,
        }
    }

    /// The glyph on the panel's button in the strip.
    pub(super) fn glyph(self) -> &'static str {
        match self {
            RawPanel::Adjust => "☰",
            RawPanel::Presets => "★",
            RawPanel::Reshape => "⬚",
            RawPanel::EyeFix => "◉",
            RawPanel::Lut => "◐",
            RawPanel::Compare => "◫",
        }
    }
}

/// How Compare lays out the picture.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum CompareLayout {
    #[default]
    Single,
    SideBySide,
    Stacked,
}

/// The strip of panel buttons: one per panel, the current one marked. Returns the panel picked.
pub(super) fn strip(ui: &mut Ui, current: RawPanel) -> Option<RawPanel> {
    let words = Env::of(ui).words::<RawDevelopWords>();
    let mut picked = None;
    ui.vertical_centered(|ui| {
        for panel in RawPanel::ALL {
            let text = egui::RichText::new(panel.glyph()).size(18.0);
            let button = egui::Button::selectable(panel == current, text).min_size(egui::vec2(36.0, 36.0));
            if ui.add(button).on_hover_text(panel.word(&words).as_str()).clicked() {
                picked = Some(panel);
            }
            ui.add_space(2.0);
        }
    });
    picked
}

/// What Reshape asked for this pass.
#[derive(Default)]
pub(super) struct ReshapePass {
    pub(super) edit: EditEvents<RawDevelopSettings>,
    pub(super) level: bool,
    pub(super) reset: bool,
}

/// The Reshape panel: Level and Reset, then the geometry sliders.
pub(super) fn reshape_panel(ui: &mut Ui, settings: &RawDevelopSettings) -> ReshapePass {
    let words = Env::of(ui).words::<RawDevelopWords>();
    let defaults = RawDevelopSettings::default();
    let mut pass = ReshapePass::default();
    ui.horizontal(|ui| {
        if clicked(&Button::new(words.level.clone()).id_salt("raw-level").show(ui)) {
            pass.level = true;
        }
        if clicked(&Button::new(words.reset_reshape.clone()).id_salt("raw-reset-reshape").show(ui)) {
            pass.reset = true;
        }
    });
    for row in GEOMETRY {
        let set = row.set;
        let field = SliderField::new((row.word)(&words).clone(), Bind::given(f64::from((row.get)(settings))).reset(f64::from((row.get)(&defaults))), row.min..=row.max)
            .step(row.step);
        let events = field.show(ui).edit.events;
        pass.edit.extend(lift(events, settings, &move |s: &mut RawDevelopSettings, v: f64| set(s, v as f32)));
    }
    pass
}

/// The settings with Reshape's fields back at their defaults.
pub(super) fn without_reshape(settings: &RawDevelopSettings) -> RawDevelopSettings {
    let defaults = RawDevelopSettings::default();
    let mut next = settings.clone();
    for row in GEOMETRY {
        (row.set)(&mut next, (row.get)(&defaults));
    }
    next
}

/// The tilt of the picture's horizon, in degrees (clockwise on screen positive), from the near-horizontal and
/// near-vertical edges of `image` (straight RGBA): the angle most of its strong edges lean by. `None` without edges.
pub(super) fn measure_tilt(image: &ColorImage) -> Option<f32> {
    let [w, h] = image.size;
    if w < 8 || h < 8 {
        return None;
    }
    let luma: Vec<f32> = image.pixels.iter().map(|c: &Color32| 0.299 * f32::from(c.r()) + 0.587 * f32::from(c.g()) + 0.114 * f32::from(c.b())).collect();
    let at = |x: usize, y: usize| luma.get(y * w + x).copied().unwrap_or(0.0);
    // Deviation from level, -20..=20 degrees, in quarter-degree bins, weighted by edge strength.
    const BINS: usize = 161;
    let mut hist = [0.0f32; BINS];
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let gx = (at(x + 1, y - 1) + 2.0 * at(x + 1, y) + at(x + 1, y + 1)) - (at(x - 1, y - 1) + 2.0 * at(x - 1, y) + at(x - 1, y + 1));
            let gy = (at(x - 1, y + 1) + 2.0 * at(x, y + 1) + at(x + 1, y + 1)) - (at(x - 1, y - 1) + 2.0 * at(x, y - 1) + at(x + 1, y - 1));
            let mag = (gx * gx + gy * gy).sqrt();
            if mag < 80.0 {
                continue;
            }
            // The edge runs across the gradient; fold its angle into -45..45 around the nearest axis.
            let edge = gy.atan2(gx).to_degrees() + 90.0;
            let mut dev = edge.rem_euclid(90.0);
            if dev > 45.0 {
                dev -= 90.0;
            }
            if dev.abs() <= 20.0 {
                let bin = ((dev + 20.0) * 4.0).round() as usize;
                if let Some(slot) = hist.get_mut(bin) {
                    *slot += mag;
                }
            }
        }
    }
    let (best, weight) = hist.iter().enumerate().fold((0usize, 0.0f32), |(bi, bw), (i, &v)| if v > bw { (i, v) } else { (bi, bw) });
    (weight > 0.0).then(|| best as f32 / 4.0 - 20.0)
}

/// What Eye Fix asked for this pass.
#[derive(Default)]
pub(super) struct EyePass {
    pub(super) edit: EditEvents<RawDevelopSettings>,
    pub(super) clear: bool,
}

/// The Eye Fix panel: how to use it, the kind of eye and the size of a new spot (`pet`, `size` in percent of the
/// width), Pupil Size and Darken, the count and Clear All.
pub(super) fn eye_panel(ui: &mut Ui, settings: &RawDevelopSettings, pet: &mut bool, size: &mut f32) -> EyePass {
    let words = Env::of(ui).words::<RawDevelopWords>();
    let mut pass = EyePass::default();
    Text::new(words.eye_fix_hint.clone()).role(TypeRole::Caption).show(ui);
    let kinds = [Entry::new(false, words.eye_red.clone()), Entry::new(true, words.eye_pet.clone())];
    if let Some(&picked) = Select::new(words.eye_kind.clone(), Bind::given(*pet), &kinds).show(ui).events.value() {
        *pet = picked;
    }
    let shown = SliderField::new(words.eye_size.clone(), Bind::given(f64::from(*size)).reset(2.0), 0.2..=12.0).step(0.1).show(ui);
    if let Some(&v) = shown.edit.events.value() {
        *size = (v as f32).clamp(0.2, 12.0);
    }
    let pupil = SliderField::new(words.eye_pupil.clone(), Bind::given(f64::from(settings.eye_pupil)).reset(50.0), 0.0..=100.0).step(1.0);
    pass.edit.extend(lift(pupil.show(ui).edit.events, settings, &|s: &mut RawDevelopSettings, v: f64| s.eye_pupil = v as f32));
    let darken = SliderField::new(words.eye_darken.clone(), Bind::given(f64::from(settings.eye_darken)).reset(50.0), 0.0..=100.0).step(1.0);
    pass.edit.extend(lift(darken.show(ui).edit.events, settings, &|s: &mut RawDevelopSettings, v: f64| s.eye_darken = v as f32));
    let count = settings.red_eyes.len().to_string();
    Text::new(Label::from(words.eye_count.fill(&[("n", count.as_str())]))).role(TypeRole::Caption).show(ui);
    if !settings.red_eyes.is_empty() && clicked(&Button::new(words.eye_clear.clone()).id_salt("raw-eye-clear").show(ui)) {
        pass.clear = true;
    }
    pass
}

/// The Compare panel: the layout, and Before for the single view.
pub(super) fn compare_panel(ui: &mut Ui, layout: &mut CompareLayout, before: &mut bool) {
    let words = Env::of(ui).words::<RawDevelopWords>();
    let layouts = [
        Entry::new(CompareLayout::Single, words.compare_single.clone()),
        Entry::new(CompareLayout::SideBySide, words.compare_side.clone()),
        Entry::new(CompareLayout::Stacked, words.compare_stacked.clone()),
    ];
    if let Some(&picked) = Select::new(words.panel_compare.clone(), Bind::given(*layout), &layouts).show(ui).events.value() {
        *layout = picked;
    }
    if *layout == CompareLayout::Single {
        let label = if *before { words.after_pane.clone() } else { words.before_pane.clone() };
        if clicked(&Button::new(label).id_salt("raw-compare-before").show(ui)) {
            *before = !*before;
        }
        Text::new(words.compare_hint.clone()).role(TypeRole::Caption).show(ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_level_picture_measures_level_and_a_blank_one_measures_nothing() {
        let mut image = ColorImage::filled([64, 64], Color32::BLACK);
        for y in 32..64 {
            for x in 0..64 {
                image.pixels[y * 64 + x] = Color32::WHITE;
            }
        }
        let tilt = measure_tilt(&image).unwrap();
        assert!(tilt.abs() < 0.5, "{tilt}");
        assert_eq!(measure_tilt(&ColorImage::filled([64, 64], Color32::GRAY)), None);
    }
}
