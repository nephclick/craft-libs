//! The RAW Develop settings: the typed session of the Camera Raw filter, without egui.

use serde::{Deserialize, Serialize};

/// One colour grading wheel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GradeWheel {
    /// Degrees 0..360.
    pub hue: f32,
    /// 0..=100.
    pub sat: f32,
    /// -100..=100.
    pub lum: f32,
}

/// What the vignette protects.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VignetteStyle {
    /// Colour priority.
    ColorPriority,
    /// Paint overlay.
    PaintOverlay,
    /// Highlight priority; a stored name outside the three reads as this one (serde's fallback is the last variant).
    #[default]
    #[serde(other)]
    HighlightPriority,
}

/// Longest LUT name, in bytes, the settings accept from commands and the control channel.
pub const MAX_LUT_NAME: usize = 256;

/// Most points a point curve accepts from commands and the control channel.
pub const MAX_CURVE_POINTS: usize = 16;

/// The demosaic algorithm a raw is opened with.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum Demosaic {
    /// Adaptive homogeneity-directed.
    Ahd,
    /// Modified high-contrast.
    Mhc,
    /// Bilinear.
    Bilinear,
}

/// The settings the RAW Develop workspace edits and the develop reads: the field set and serde names of PhotoCraft's
/// `CameraRaw`, without its three develop inputs (`seed`, `pixel_scale`, `encoding_gamma`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
#[non_exhaustive]
pub struct RawDevelopSettings {
    // Basic
    /// The temperature.
    pub temperature: f32,
    /// The tint.
    pub tint: f32,
    /// The exposure.
    pub exposure: f32,
    /// The contrast.
    pub contrast: f32,
    /// The highlights.
    pub highlights: f32,
    /// The shadows.
    pub shadows: f32,
    /// The whites.
    pub whites: f32,
    /// The blacks.
    pub blacks: f32,
    /// The texture.
    pub texture: f32,
    /// The clarity.
    pub clarity: f32,
    /// The dehaze.
    pub dehaze: f32,
    /// The vibrance.
    pub vibrance: f32,
    /// The saturation.
    pub saturation: f32,
    // Tone curve
    /// The curve highlights.
    pub curve_highlights: f32,
    /// The curve lights.
    pub curve_lights: f32,
    /// The curve darks.
    pub curve_darks: f32,
    /// The curve shadows.
    pub curve_shadows: f32,
    /// Region splits (percent).
    pub curve_splits: [f32; 3],
    /// Point curves as `[input, output]` in 0..=255 (empty = linear).
    pub point_curve: Vec<[f32; 2]>,
    /// The point curve red.
    pub point_curve_red: Vec<[f32; 2]>,
    /// The point curve green.
    pub point_curve_green: Vec<[f32; 2]>,
    /// The point curve blue.
    pub point_curve_blue: Vec<[f32; 2]>,
    // Color mixer: red, orange, yellow, green, aqua, blue, purple, magenta.
    /// The hsl hue.
    pub hsl_hue: [f32; 8],
    /// The hsl sat.
    pub hsl_sat: [f32; 8],
    /// The hsl lum.
    pub hsl_lum: [f32; 8],
    // Color grading
    /// The grade shadows.
    pub grade_shadows: GradeWheel,
    /// The grade midtones.
    pub grade_midtones: GradeWheel,
    /// The grade highlights.
    pub grade_highlights: GradeWheel,
    /// The grade global.
    pub grade_global: GradeWheel,
    /// The grade blending.
    pub grade_blending: f32,
    /// The grade balance.
    pub grade_balance: f32,
    // Detail
    /// The sharpen amount.
    pub sharpen_amount: f32,
    /// The sharpen radius.
    pub sharpen_radius: f32,
    /// The sharpen detail.
    pub sharpen_detail: f32,
    /// The sharpen masking.
    pub sharpen_masking: f32,
    /// The noise luminance.
    pub noise_luminance: f32,
    /// The noise luminance detail.
    pub noise_luminance_detail: f32,
    /// The noise color.
    pub noise_color: f32,
    /// The noise color detail.
    pub noise_color_detail: f32,
    // Effects
    /// The grain amount.
    pub grain_amount: f32,
    /// The grain size.
    pub grain_size: f32,
    /// The grain roughness.
    pub grain_roughness: f32,
    /// The vignette amount.
    pub vignette_amount: f32,
    /// The vignette midpoint.
    pub vignette_midpoint: f32,
    /// The vignette roundness.
    pub vignette_roundness: f32,
    /// The vignette feather.
    pub vignette_feather: f32,
    /// The vignette highlights.
    pub vignette_highlights: f32,
    /// The vignette style.
    pub vignette_style: VignetteStyle,
    // Calibration
    /// Shadows › Tint: −100 (green) … 100 (magenta), strongest in the darkest tones.
    pub shadow_tint: f32,
    /// Red, Green and Blue Primary › Hue: −100 … 100, the primary turned towards its neighbours.
    pub calibration_hue: [f32; 3],
    /// Red, Green and Blue Primary › Saturation: −100 (grey) … 100 (purer).
    pub calibration_sat: [f32; 3],
    // Optics (from Nephclick PicShot)
    /// Remove chromatic aberration.
    #[serde(default)]
    pub remove_ca: bool,
    /// Defringe purple amount (0..=20).
    #[serde(default)]
    pub defringe_purple: f32,
    /// Defringe green amount (0..=20).
    #[serde(default)]
    pub defringe_green: f32,
    // Geometry (from Nephclick PicShot)
    /// Distortion (-100..=100).
    #[serde(default)]
    pub distortion: f32,
    /// Vertical perspective (-100..=100).
    #[serde(default)]
    pub vertical: f32,
    /// Horizontal perspective (-100..=100).
    #[serde(default)]
    pub horizontal: f32,
    /// Rotate (-45..=45).
    #[serde(default)]
    pub rotate: f32,
    /// Scale (50..=150, default 100).
    #[serde(default = "default_scale")]
    pub scale: f32,
    /// Aspect ratio (-100..=100).
    #[serde(default)]
    pub aspect: f32,
    /// Offset X (-100..=100).
    #[serde(default)]
    pub offset_x: f32,
    /// Offset Y (-100..=100).
    #[serde(default)]
    pub offset_y: f32,
    // Lens Blur (from Nephclick PicShot)
    /// Lens blur amount (0..=100).
    #[serde(default)]
    pub lens_blur_amount: f32,
    /// Bokeh boost (0..=100).
    #[serde(default)]
    pub lens_blur_boost: f32,
    // Look
    /// The name of the colour look-up table (LUT) applied after the develop, as the app's LUT library names it; empty
    /// for none. A name the library does not hold leaves the picture as it is.
    #[serde(default)]
    pub lut: String,
    /// How strongly the LUT applies (0..=100).
    #[serde(default = "default_lut_amount")]
    pub lut_amount: f32,
    // Eye Fix
    /// The eyes Eye Fix corrects, in picture fractions.
    #[serde(default)]
    pub red_eyes: Vec<EyeSpot>,
    /// Eye Fix's pupil size (0..=100): how much of each spot's circle is corrected.
    #[serde(default = "default_half")]
    pub eye_pupil: f32,
    /// Eye Fix's darken (0..=100): how dark the corrected pupil becomes.
    #[serde(default = "default_half")]
    pub eye_darken: f32,
}

/// Most eyes Eye Fix corrects in one picture.
pub const MAX_EYE_SPOTS: usize = 32;

/// One eye Eye Fix corrects: its centre and radius as fractions of the picture's width and height (the radius of
/// its width), and whether it is an animal's eye (a bright reflection rather than a red one).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EyeSpot {
    /// Centre, 0..=1 of the width.
    pub x: f32,
    /// Centre, 0..=1 of the height.
    pub y: f32,
    /// Radius, as a fraction of the width.
    pub radius: f32,
    /// An animal's eye: the bright reflection is darkened, whatever its colour.
    #[serde(default)]
    pub pet: bool,
}

impl EyeSpot {
    /// A spot at `x`, `y` of radius `radius` (all fractions of the picture).
    pub fn new(x: f32, y: f32, radius: f32, pet: bool) -> Self {
        EyeSpot { x, y, radius, pet }
    }

    fn valid(&self) -> bool {
        [self.x, self.y].iter().all(|v| v.is_finite() && (0.0..=1.0).contains(v)) && self.radius.is_finite() && (0.0005..=0.25).contains(&self.radius)
    }
}

fn default_half() -> f32 {
    50.0
}

fn default_lut_amount() -> f32 {
    100.0
}

fn default_scale() -> f32 {
    100.0
}

impl Default for RawDevelopSettings {
    fn default() -> Self {
        RawDevelopSettings {
            temperature: 0.0,
            tint: 0.0,
            exposure: 0.0,
            contrast: 0.0,
            highlights: 0.0,
            shadows: 0.0,
            whites: 0.0,
            blacks: 0.0,
            texture: 0.0,
            clarity: 0.0,
            dehaze: 0.0,
            vibrance: 0.0,
            saturation: 0.0,
            curve_highlights: 0.0,
            curve_lights: 0.0,
            curve_darks: 0.0,
            curve_shadows: 0.0,
            curve_splits: [25.0, 50.0, 75.0],
            point_curve: Vec::new(),
            point_curve_red: Vec::new(),
            point_curve_green: Vec::new(),
            point_curve_blue: Vec::new(),
            hsl_hue: [0.0; 8],
            hsl_sat: [0.0; 8],
            hsl_lum: [0.0; 8],
            grade_shadows: GradeWheel::default(),
            grade_midtones: GradeWheel::default(),
            grade_highlights: GradeWheel::default(),
            grade_global: GradeWheel::default(),
            grade_blending: 50.0,
            grade_balance: 0.0,
            sharpen_amount: 0.0,
            sharpen_radius: 1.0,
            sharpen_detail: 25.0,
            sharpen_masking: 0.0,
            noise_luminance: 0.0,
            noise_luminance_detail: 50.0,
            noise_color: 0.0,
            noise_color_detail: 50.0,
            grain_amount: 0.0,
            grain_size: 25.0,
            grain_roughness: 50.0,
            vignette_amount: 0.0,
            vignette_midpoint: 50.0,
            vignette_roundness: 0.0,
            vignette_feather: 50.0,
            vignette_highlights: 0.0,
            vignette_style: VignetteStyle::HighlightPriority,
            shadow_tint: 0.0,
            calibration_hue: [0.0; 3],
            calibration_sat: [0.0; 3],
            remove_ca: false,
            defringe_purple: 0.0,
            defringe_green: 0.0,
            distortion: 0.0,
            vertical: 0.0,
            horizontal: 0.0,
            rotate: 0.0,
            scale: 100.0,
            aspect: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
            lens_blur_amount: 0.0,
            lens_blur_boost: 0.0,
            lut: String::new(),
            lut_amount: 100.0,
            red_eyes: Vec::new(),
            eye_pupil: 50.0,
            eye_darken: 50.0,
        }
    }
}

/// Checks a point curve from untrusted input: empty (linear) or 2..=16 finite points in 0..=255 with inputs
/// increasing by at least one level. Deserialization stays lenient: earlier editors saved curves this rejects, and the
/// develop sorts and de-duplicates them, so stored Smart Filters keep rendering.
pub fn validate_curve(points: &[[f32; 2]]) -> Result<(), String> {
    if points.len() == 1 {
        return Err("a nonempty curve needs at least two points".into());
    }
    if points.len() > MAX_CURVE_POINTS {
        return Err(format!("at most {MAX_CURVE_POINTS} curve points"));
    }
    if !points.iter().flatten().all(|v| v.is_finite() && (0.0..=255.0).contains(v)) {
        return Err("curve coordinates must be finite and in 0..=255".into());
    }
    if points.windows(2).any(|w| w[1][0] - w[0][0] < 1.0) {
        return Err("curve inputs must increase by at least one level".into());
    }
    Ok(())
}

impl GradeWheel {
    fn neutral(&self) -> bool {
        self.sat == 0.0 && self.lum == 0.0
    }
}

fn is_linear(curve: &[[f32; 2]]) -> bool {
    curve.len() < 2 || curve.iter().all(|p| (p[0] - p[1]).abs() < 1e-3)
}

impl RawDevelopSettings {
    /// Refuses malformed point curves, naming the first bad one.
    pub fn validate(&self) -> Result<(), String> {
        for (name, curve) in [
            ("pointCurve", &self.point_curve),
            ("pointCurveRed", &self.point_curve_red),
            ("pointCurveGreen", &self.point_curve_green),
            ("pointCurveBlue", &self.point_curve_blue),
        ] {
            validate_curve(curve).map_err(|e| format!("{name}: {e}"))?;
        }
        self.validate_look()
    }

    /// Refuses a LUT name longer than [`MAX_LUT_NAME`] bytes and a LUT amount outside 0..=100.
    fn validate_look(&self) -> Result<(), String> {
        if self.lut.len() > MAX_LUT_NAME {
            return Err(format!("lut: a LUT name is at most {MAX_LUT_NAME} bytes"));
        }
        if !(self.lut_amount.is_finite() && (0.0..=100.0).contains(&self.lut_amount)) {
            return Err("lutAmount: must be in 0..=100".into());
        }
        if self.red_eyes.len() > MAX_EYE_SPOTS {
            return Err(format!("redEyes: at most {MAX_EYE_SPOTS} eyes"));
        }
        if !self.red_eyes.iter().all(EyeSpot::valid) {
            return Err("redEyes: each eye needs x and y in 0..=1 and a radius in 0.0005..=0.25".into());
        }
        if ![self.eye_pupil, self.eye_darken].iter().all(|v| v.is_finite() && (0.0..=100.0).contains(v)) {
            return Err("eyePupil and eyeDarken must be in 0..=100".into());
        }
        Ok(())
    }

    /// [`validate`](Self::validate) for an edit of `stored` settings: a curve left as stored is accepted, so editing
    /// another control of an older Smart Filter still works.
    pub fn validate_changes(&self, stored: &Self) -> Result<(), String> {
        for (name, curve, kept) in [
            ("pointCurve", &self.point_curve, &stored.point_curve),
            ("pointCurveRed", &self.point_curve_red, &stored.point_curve_red),
            ("pointCurveGreen", &self.point_curve_green, &stored.point_curve_green),
            ("pointCurveBlue", &self.point_curve_blue, &stored.point_curve_blue),
        ] {
            if curve != kept {
                validate_curve(curve).map_err(|e| format!("{name}: {e}"))?;
            }
        }
        if self.lut != stored.lut
            || self.lut_amount != stored.lut_amount
            || self.red_eyes != stored.red_eyes
            || self.eye_pupil != stored.eye_pupil
            || self.eye_darken != stored.eye_darken
        {
            self.validate_look()?;
        }
        Ok(())
    }

    fn wb_neutral(&self) -> bool {
        self.temperature == 0.0 && self.tint == 0.0 && self.exposure == 0.0
    }

    fn tone_neutral(&self) -> bool {
        [self.contrast, self.highlights, self.shadows, self.texture, self.clarity, self.whites, self.blacks].iter().all(|v| *v == 0.0)
    }

    fn hsl_neutral(&self) -> bool {
        self.hsl_hue.iter().chain(&self.hsl_sat).chain(&self.hsl_lum).all(|v| *v == 0.0)
    }

    fn grade_neutral(&self) -> bool {
        self.grade_shadows.neutral() && self.grade_midtones.neutral() && self.grade_highlights.neutral() && self.grade_global.neutral()
    }

    fn calibration_neutral(&self) -> bool {
        self.shadow_tint == 0.0 && self.calibration_hue.iter().chain(&self.calibration_sat).all(|v| *v == 0.0)
    }

    fn curves_neutral(&self) -> bool {
        [self.curve_highlights, self.curve_lights, self.curve_darks, self.curve_shadows].iter().all(|v| *v == 0.0)
            && [&self.point_curve, &self.point_curve_red, &self.point_curve_green, &self.point_curve_blue].iter().all(|c| is_linear(c))
    }

    /// Whether every control is neutral, so the settings leave the picture as it is.
    pub fn is_identity(&self) -> bool {
        self.wb_neutral()
            && self.calibration_neutral()
            && self.tone_neutral()
            && self.dehaze == 0.0
            && self.vibrance == 0.0
            && self.saturation == 0.0
            && self.curves_neutral()
            && self.hsl_neutral()
            && self.grade_neutral()
            && self.sharpen_amount == 0.0
            && self.noise_luminance == 0.0
            && self.noise_color == 0.0
            && self.grain_amount == 0.0
            && self.vignette_amount == 0.0
            && !self.remove_ca
            && self.defringe_purple == 0.0
            && self.defringe_green == 0.0
            && self.distortion == 0.0
            && self.vertical == 0.0
            && self.horizontal == 0.0
            && self.rotate == 0.0
            && (self.scale == 100.0 || self.scale == 0.0)
            && self.aspect == 0.0
            && self.offset_x == 0.0
            && self.offset_y == 0.0
            && self.lens_blur_amount == 0.0
            && (self.lut.is_empty() || self.lut_amount == 0.0)
            && self.red_eyes.is_empty()
    }
}
