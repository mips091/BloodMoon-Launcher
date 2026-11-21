//! Horizon's design tokens, the one source of truth for the launcher (`launcher/src/theme.rs`),
//! the game's on-screen keyboard (`horizon-client`'s `ui_theme`) and the website (`tokens.css` in this crate,
//! generated from [`css`] by `cargo run -p xtask -- tokens`, which the site's build reads from main). Change a value here and all three
//! follow. The system is described in `docs/design-system.md`.
//!
//! Cool neutrals in two appearances and one violet accent; Geist and Geist Mono, bundled here
//! under the SIL Open Font License (`fonts/OFL-Geist.txt`). The launcher's night look and its
//! Bricolage Grotesque are in [`night`]. Colours are `0xRRGGBB`; sizes
//! are in logical pixels.
use std::fmt::Write as _;
use std::time::Duration;

pub const FONT: &str = "Geist";
pub const MONO: &str = "Geist Mono";
pub mod glass;
pub mod night;

/// The launcher's and the website's display face (headings, navigation, buttons, badges only):
/// Pixelify Sans, bundled under the SIL Open Font License (`fonts/OFL-PixelifySans.txt`).
pub const PIXEL: &str = "Pixelify Sans";

/// The bundled font files.
pub mod fonts {
    pub const GEIST_REGULAR: &[u8] = include_bytes!("../fonts/Geist-Regular.ttf");
    pub const GEIST_MEDIUM: &[u8] = include_bytes!("../fonts/Geist-Medium.ttf");
    pub const GEIST_SEMIBOLD: &[u8] = include_bytes!("../fonts/Geist-SemiBold.ttf");
    pub const GEIST_MONO: &[u8] = include_bytes!("../fonts/GeistMono-Regular.ttf");
    pub const PIXEL_MEDIUM: &[u8] = include_bytes!("../fonts/PixelifySans-Medium.ttf");
    pub const PIXEL_SEMIBOLD: &[u8] = include_bytes!("../fonts/PixelifySans-SemiBold.ttf");
    pub const PIXEL_BOLD: &[u8] = include_bytes!("../fonts/PixelifySans-Bold.ttf");
    /// [`crate::night`]'s faces, cut from upstream's variable font by `tools/bake_bricolage.py`.
    pub const BRICOLAGE_REGULAR: &[u8] = include_bytes!("../fonts/BricolageGrotesque-Regular.ttf");
    pub const BRICOLAGE_MEDIUM: &[u8] = include_bytes!("../fonts/BricolageGrotesque-Medium.ttf");
    pub const BRICOLAGE_SEMIBOLD: &[u8] =
        include_bytes!("../fonts/BricolageGrotesque-SemiBold.ttf");
    pub const BRICOLAGE_BOLD: &[u8] = include_bytes!("../fonts/BricolageGrotesque-Bold.ttf");
    pub const BRICOLAGE_DISPLAY_BOLD: &[u8] =
        include_bytes!("../fonts/BricolageGrotesqueDisplay-Bold.ttf");
    /// The glass look's headings ([`crate::glass`]): SemiBold at optical sizes 56 and 28.
    pub const BRICOLAGE_DISPLAY_SEMIBOLD: &[u8] =
        include_bytes!("../fonts/BricolageGrotesqueDisplay-SemiBold.ttf");
    pub const BRICOLAGE_HEADING_SEMIBOLD: &[u8] =
        include_bytes!("../fonts/BricolageGrotesqueHeading-SemiBold.ttf");
}

/// The glass look's icons: Hugeicons' free set (MIT, `icons/glass/LICENSE-hugeicons.txt`), the
/// ones the mockup uses, 24 unit outlines stroked 1.6 in `currentColor`, by Hugeicons' name
/// (`"folder-01"`, `"more-vertical"`…). `github` is GitHub's mark, drawn for Horizon (filled,
/// its cat cut out by a mask), beside the links to GitHub Sponsors. `sparkles` (Home's What's
/// new) is drawn for Horizon in the same 1.6 stroke.
pub mod glass_icons {
    macro_rules! icons {
        ($($name:literal),* $(,)?) => {
            /// Every icon, `(name, svg)`.
            pub const ALL: &[(&str, &[u8])] = &[
                $(($name, include_bytes!(concat!("../icons/glass/", $name, ".svg")))),*
            ];
        };
    }
    icons!(
        "ai-chat-02",
        "arrow-down-01",
        "arrow-right-01",
        "book",
        "map",
        "cancel-01",
        "clock",
        "color-picker",
        "computer",
        "computer-terminal-01",
        "copy-01",
        "dashboard-square-01",
        "delete-02",
        "discord",
        "download-04",
        "earth",
        "favourite",
        "flash",
        "folder-01",
        "game-controller-01",
        "github",
        "globe-02",
        "hammer",
        "image-01",
        "information-circle",
        "link-01",
        "minus-sign",
        "more-horizontal",
        "more-vertical",
        "package-01",
        "pencil-edit-02",
        "play",
        "plus-sign",
        "puzzle",
        "refresh",
        "search-01",
        "sliders-horizontal",
        "sparkles",
        "stop",
        "tick-02",
        "upload-04",
        "user",
        "user-add-01",
        "user-multiple-02",
        "view",
        "view-off-slash",
    );

    /// The icon called `name`, if there is one.
    pub fn get(name: &str) -> Option<&'static [u8]> {
        ALL.iter().find(|(n, _)| *n == name).map(|(_, svg)| *svg)
    }
}

/// The launcher's pixel icons: Pixelarticons (MIT, `icons/pixel/LICENSE-pixelarticons.txt`), 24
/// unit SVGs filled with `currentColor`, by name (`"home"`, `"play"`…).
pub mod icons {
    macro_rules! icons {
        ($($name:literal),* $(,)?) => {
            /// Every icon, `(name, svg)`.
            pub const ALL: &[(&str, &[u8])] = &[
                $(($name, include_bytes!(concat!("../icons/pixel/", $name, ".svg")))),*
            ];
        };
    }
    icons!(
        "archive",
        "arrow-left",
        "book-open",
        "box",
        "bug",
        "check",
        "chevron-down",
        "chevron-right",
        "clock",
        "close",
        "download",
        "external-link",
        "folder",
        "gamepad",
        "globe",
        "heart",
        "home",
        "image",
        "info-box",
        "map",
        "message",
        "monitor",
        "notes",
        "package",
        "play",
        "plus",
        "reload",
        "search",
        "server",
        "sliders",
        // Not Pixelarticons': drawn for Horizon's ratings, in the same 2-unit grid.
        "star",
        "terminal",
        "trash",
        "user",
        "zap",
    );

    /// An icon's SVG by name.
    pub fn get(name: &str) -> Option<&'static [u8]> {
        ALL.iter().find(|(n, _)| *n == name).map(|(_, svg)| *svg)
    }
}

/// Horizon's icon at 64 px (PNG), for window title bars and task bars at run time. Rendered from
/// `docs/media/brand/icon.svg` by `cargo run -p xtask -- icons`.
pub const ICON_PNG: &[u8] = &[137,80,78,71,13,10,26,10,0,0,0,13,73,72,68,82,0,0,0,64,0,0,0,64,8,6,0,0,0,170,105,113,222,0,0,1,242,73,68,65,84,120,218,237,155,191,78,195,48,16,198,27,171,66,180,115,150,60,4,75,23,22,158,128,149,149,141,55,96,102,100,230,13,216,88,187,178,51,118,225,53,178,120,14,35,76,169,140,229,218,231,63,119,190,56,103,169,82,43,53,245,125,191,251,236,250,28,167,187,190,218,253,110,86,220,212,102,229,77,0,8,0,1,32,0,4,128,0,16,0,237,52,173,167,147,214,211,9,250,253,14,123,37,120,28,14,231,247,15,227,55,186,120,243,115,223,239,111,67,215,108,41,132,83,136,79,109,209,14,208,122,50,9,7,133,83,139,159,93,0,201,126,52,0,83,188,11,66,109,241,85,39,65,151,248,37,180,45,166,112,238,217,143,118,128,61,230,191,110,238,214,183,14,232,251,253,249,229,107,75,200,126,214,28,176,212,49,95,4,64,43,226,147,0,64,196,47,197,254,82,12,197,2,104,201,250,226,128,88,0,45,102,95,28,32,0,16,247,3,204,246,243,242,54,230,254,198,238,245,121,168,182,31,0,29,255,247,79,143,35,117,6,115,193,100,3,168,33,186,36,140,36,0,177,162,75,216,55,118,24,65,251,4,3,128,136,254,124,255,24,168,151,195,16,48,62,24,65,0,161,14,76,209,181,107,130,80,172,46,16,94,0,151,126,208,39,154,75,81,116,41,118,27,130,19,128,235,98,168,104,142,149,161,75,207,12,66,97,138,231,252,239,48,235,252,231,0,91,252,124,97,110,29,192,105,127,192,214,168,32,212,114,5,112,42,164,108,55,40,23,25,140,101,39,87,8,170,212,90,125,169,37,53,248,214,88,201,224,41,230,132,227,112,0,245,163,106,4,141,237,4,168,120,178,114,216,7,161,54,216,232,219,227,88,217,203,1,145,115,22,161,154,3,124,34,32,2,74,37,34,233,136,12,231,13,210,88,39,41,106,187,114,18,159,12,128,35,132,212,120,100,91,188,133,34,39,39,142,98,231,4,107,76,140,37,18,160,56,5,83,163,63,197,49,40,202,126,208,142,202,98,12,9,12,192,164,103,133,57,58,171,163,126,108,142,219,17,155,78,158,27,148,133,144,0,16,0,2,64,0,8,128,213,182,63,173,164,229,111,70,104,6,96,0,0,0,0,73,69,78,68,174,66,96,130];

/// How a type step sizes on the web.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Web {
    /// The app size, fixed.
    Fixed,
    /// From `min` px on a 375 px phone to `max` px at 1440 px and wider.
    Fluid(f32, f32),
    /// A share of the viewport width, in `vw`.
    Viewport(f32),
}

/// One step of the type scale.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Step {
    pub name: &'static str,
    /// The size in the apps (launcher and game menus).
    pub px: f32,
    pub web: Web,
    /// Line height as a multiple of the size.
    pub line: f32,
    /// Letter spacing in em.
    pub tracking: f32,
    pub weight: u16,
    pub mono: bool,
}

impl Step {
    /// The line height in pixels at the app size.
    pub fn line_px(&self) -> f32 {
        (self.px * self.line).round()
    }
}

const fn step(
    name: &'static str,
    px: f32,
    web: Web,
    line: f32,
    tracking: f32,
    weight: u16,
    mono: bool,
) -> Step {
    Step {
        name,
        px,
        web,
        line,
        tracking,
        weight,
        mono,
    }
}

/// Indices (`01`), counts and timestamps, in mono.
pub const MICRO: Step = step("micro", 11., Web::Fixed, 1.5, 0.04, 400, true);
/// Footnotes and help under a control.
pub const CAPTION: Step = step("caption", 12., Web::Fixed, 1.5, 0., 400, false);
/// Row labels, navigation and buttons.
pub const LABEL: Step = step("label", 13., Web::Fixed, 1.5, 0., 400, false);
/// Body text in the apps.
pub const UI: Step = step("ui", 14., Web::Fixed, 1.5, 0., 400, false);
/// Paragraphs.
pub const BODY: Step = step("body", 15., Web::Fluid(15., 17.), 1.6, 0., 400, false);
/// Section heads.
pub const HEADING: Step = step("heading", 20., Web::Fluid(20., 24.), 1.3, -0.01, 500, false);
/// The names in an index list.
pub const TITLE: Step = step("title", 28., Web::Fluid(32., 48.), 1.05, -0.02, 500, false);
/// Page titles.
pub const DISPLAY: Step = step(
    "display",
    44.,
    Web::Fluid(34., 64.),
    1.05,
    -0.025,
    500,
    false,
);
/// The giant faint signature.
pub const WORDMARK: Step = step("wordmark", 220., Web::Viewport(18.), 0.8, -0.04, 500, false);

pub const TYPE: [Step; 9] = [
    MICRO, CAPTION, LABEL, UI, BODY, HEADING, TITLE, DISPLAY, WORDMARK,
];

/// The spacing scale.
pub const SPACE: [f32; 9] = [4., 8., 12., 16., 24., 32., 48., 64., 96.];
/// Page margins: wide windows and pages, and phones.
pub const GUTTER: f32 = 64.;
pub const GUTTER_NARROW: f32 = 16.;
/// The longest paragraph line, in characters.
pub const MEASURE: f32 = 56.;

/// Corner radii: fields and small controls; plates (hover rows, console, dialogs); pills
/// (buttons, segmented controls, switches).
pub const RADIUS: f32 = 8.;
pub const RADIUS_PLATE: f32 = 12.;
pub const RADIUS_PILL: f32 = 999.;
/// Rules between rows are one pixel; there are no other borders.
pub const RULE: f32 = 1.;

/// Heights: buttons, the one large action, a segment, a field, an index row in the apps and
/// on the web.
pub const CONTROL: f32 = 32.;
pub const CONTROL_LARGE: f32 = 40.;
pub const SEGMENT: f32 = 28.;
pub const FIELD: f32 = 36.;
pub const ROW: f32 = 64.;
pub const ROW_WEB: f32 = 86.;
/// The index column of a list (where `01` sits) and how far the hover plate reaches past a row.
pub const INDEX: f32 = 40.;
pub const PLATE_BLEED: f32 = 16.;
/// A pressed primary button's opacity, and disabled controls'.
pub const PRESSED: f32 = 0.85;
pub const DISABLED: f32 = 0.4;

/// A duration and its easing curve (CSS `cubic-bezier` control points).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Motion {
    pub name: &'static str,
    pub ms: u64,
    pub curve: [f32; 4],
}

impl Motion {
    pub const fn duration(&self) -> Duration {
        Duration::from_millis(self.ms)
    }
    pub fn secs(&self) -> f32 {
        self.ms as f32 / 1000.
    }
    /// The eased progress for a linear `t` in 0..1.
    pub fn ease(&self, t: f32) -> f32 {
        bezier(self.curve, t)
    }
}

/// Standard ease, for hovers, presses and colour.
pub const QUICK: Motion = Motion {
    name: "quick",
    ms: 150,
    curve: [0.4, 0., 0.2, 1.],
};
/// Ease-out with a long tail, for selection, switches and page changes.
pub const MOVE: Motion = Motion {
    name: "move",
    ms: 240,
    curve: [0.23, 1., 0.32, 1.],
};
/// Content arriving: a fade and a [`RISE`].
pub const ENTER: Motion = Motion {
    name: "enter",
    ms: 600,
    curve: [0.23, 1., 0.32, 1.],
};
pub const MOTION: [Motion; 3] = [QUICK, MOVE, ENTER];
/// The delay between list rows arriving, and how far content rises as it arrives.
pub const STAGGER: Duration = Duration::from_millis(40);
pub const RISE: f32 = 8.;

/// The launcher's opening: the wordmark holds, then the interface arrives.
pub const OPENING_HOLD: Duration = Duration::from_millis(1400);
pub const OPENING_MAX: Duration = Duration::from_millis(2500);
pub const OPENING_REVEAL: Duration = Duration::from_millis(800);

/// A CSS `cubic-bezier(x1, y1, x2, y2)` at `t`: solves x for the curve parameter, returns y.
pub fn bezier([x1, y1, x2, y2]: [f32; 4], t: f32) -> f32 {
    let t = t.clamp(0., 1.);
    if t == 0. || t == 1. {
        return t;
    }
    let at = |a: f32, b: f32, s: f32| {
        let u = 1. - s;
        3. * u * u * s * a + 3. * u * s * s * b + s * s * s
    };
    let (mut lo, mut hi) = (0f32, 1f32);
    for _ in 0..24 {
        let mid = (lo + hi) / 2.;
        if at(x1, x2, mid) < t {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    at(y1, y2, (lo + hi) / 2.)
}

pub fn ease_out_quint(t: f32) -> f32 {
    1. - (1. - t.clamp(0., 1.)).powi(5)
}
pub fn ease_out_cubic(t: f32) -> f32 {
    1. - (1. - t.clamp(0., 1.)).powi(3)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Palette {
    /// The background.
    pub page: u32,
    /// The hover plate, fields, the segmented track, the console.
    pub plate_soft: u32,
    /// Pressed plates, a switch that is off, the faint wordmark.
    pub plate: u32,
    /// Dialogs and menus above the page.
    pub surface: u32,
    /// One-pixel rules between rows, slider and progress tracks.
    pub rule: u32,
    /// Text, selection, the slider's fill.
    pub ink: u32,
    /// Secondary text and values.
    pub ink_soft: u32,
    /// Labels, descriptions, indices.
    pub muted: u32,
    /// The main action, links, focus, progress.
    pub accent: u32,
    pub on_accent: u32,
    pub danger: u32,
    pub success: u32,
    pub dark: bool,
}

pub const LIGHT: Palette = Palette {
    page: 0xfafafc,
    plate_soft: 0xf3f3f8,
    plate: 0xebecf4,
    surface: 0xffffff,
    rule: 0xe2e3ea,
    ink: 0x16181f,
    ink_soft: 0x3b3e49,
    muted: 0x646875,
    accent: 0x5b3fd6,
    on_accent: 0xffffff,
    danger: 0xb4372a,
    success: 0x1d7a48,
    dark: false,
};

pub const DARK: Palette = Palette {
    page: 0x0d0e13,
    plate_soft: 0x181a22,
    plate: 0x20222c,
    surface: 0x16181f,
    rule: 0x24262f,
    ink: 0xecedf3,
    ink_soft: 0xc5c7d1,
    muted: 0x8b8f9c,
    accent: 0x9a86ff,
    on_accent: 0x0d0e13,
    danger: 0xf07a68,
    success: 0x45c27a,
    dark: true,
};

/// The launcher's and the website's blocky look ("Blocks" in `docs/design-system.md`): stone-dark
/// (or lavender-light) neutrals with a violet cast, panels with a dark outline, a light bevel on top
/// and a darker lip below, and the violet `face` of the one big action. The game's menus keep
/// [`LIGHT`] and [`DARK`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Blocks {
    /// The window behind everything, tiled with a faint stone texture.
    pub page: u32,
    /// The top bar and the sidebar.
    pub side: u32,
    /// Panels: rows, cards, dialogs.
    pub panel: u32,
    /// A selected or hovered panel or row, the open tab.
    pub raised: u32,
    /// Buttons and the chosen segment.
    pub button: u32,
    /// A button's top bevel.
    pub button_bevel: u32,
    /// Sunk wells: fields, tracks, switch and checkbox boxes, segmented tracks.
    pub well: u32,
    /// The light top (and left) edge of a panel.
    pub bevel: u32,
    /// The darker bottom edge of a panel or button.
    pub lip: u32,
    /// The 2 px outline around panels, buttons and wells.
    pub outline: u32,
    /// Rules between rows inside a panel.
    pub rule: u32,
    pub ink: u32,
    pub ink_soft: u32,
    pub muted: u32,
    /// Links, focus, progress, the selection marker; text-safe on every background.
    pub accent: u32,
    /// The main action's face (Play, Install, Download), its top bevel and bottom lip.
    pub face: u32,
    pub face_bevel: u32,
    pub face_lip: u32,
    pub on_face: u32,
    pub danger: u32,
    pub success: u32,
    /// "Partly": a mod or pack that runs in part.
    pub warn: u32,
    pub dark: bool,
}

pub const BLOCKS_DARK: Blocks = Blocks {
    page: 0x0f0d17,
    side: 0x0b0a11,
    panel: 0x1a1727,
    raised: 0x221e34,
    button: 0x2b2641,
    button_bevel: 0x3d3560,
    well: 0x0c0b12,
    bevel: 0x342d52,
    lip: 0x07060b,
    outline: 0x05040a,
    rule: 0x120f1c,
    ink: 0xecedf3,
    ink_soft: 0xc5c7d1,
    muted: 0xa29cb8,
    accent: 0x9a86ff,
    face: 0x5b3fd6,
    face_bevel: 0x8a74ff,
    face_lip: 0x3a2694,
    on_face: 0xffffff,
    danger: 0xf07a68,
    success: 0x45c27a,
    warn: 0xf0b84f,
    dark: true,
};

pub const BLOCKS_LIGHT: Blocks = Blocks {
    page: 0xece7f7,
    side: 0xe2dbf2,
    panel: 0xf8f6fd,
    raised: 0xe9e3f8,
    button: 0xddd4f3,
    button_bevel: 0xf4f0fe,
    well: 0xf3f0fb,
    bevel: 0xffffff,
    lip: 0xc3b8e2,
    outline: 0x2b2244,
    rule: 0xe4ddf4,
    ink: 0x1b1530,
    ink_soft: 0x3b3354,
    muted: 0x585071,
    accent: 0x5333c9,
    face: 0x5b3fd6,
    face_bevel: 0x8069ef,
    face_lip: 0x3a2694,
    on_face: 0xffffff,
    danger: 0xa3301d,
    success: 0x136336,
    warn: 0x7a4f00,
    dark: false,
};

/// Pixel sizes of the blocky look: outlines, bevels and lips, the cut corner, and the stone tile.
pub const BLOCK_OUTLINE: f32 = 2.;
pub const BLOCK_BEVEL: f32 = 2.;
pub const BLOCK_LIP: f32 = 4.;
/// The stone texture: a 16 by 16 tile, each texel this many logical pixels.
pub const TILE_TEXELS: usize = 16;
pub const TILE_TEXEL: f32 = 4.;
/// The texture's shades: the page colour moved this many levels (of 255) lighter or darker.
pub const TILE_SHADES: [i16; 5] = [0, 2, -2, 5, -4];

/// How strongly the game's menus veil the live backdrop (the title panorama, or the world behind
/// the pause and options screens) with the page colour. Every text colour the menus use keeps
/// WCAG AA over it, even on a selected row's plate, with a backdrop as bright as
/// [`BRIGHT_BACKDROP`] (a test checks it).
pub const VEIL: f32 = 0.90;
/// The title's home page shows more of the world; its text is `ink` and `ink_soft` only.
pub const HOME_VEIL: f32 = 0.80;
/// Muted text over the veiled backdrop: the dark palette's `muted`, lifted so it holds AA there.
pub const BACKDROP_MUTED: u32 = 0x9fa3ae;
/// The brightest backdrop the veils are measured against: a light grey of luminance 0.77,
/// brighter than the snowiest title panorama measured (0.49) and a sunlit world.
pub const BRIGHT_BACKDROP: u32 = 0xe3e3e3;

impl Palette {
    /// Every colour with its token name, in the order the docs list them.
    pub fn named(&self) -> [(&'static str, u32); 12] {
        [
            ("page", self.page),
            ("plate-soft", self.plate_soft),
            ("plate", self.plate),
            ("surface", self.surface),
            ("rule", self.rule),
            ("ink", self.ink),
            ("ink-soft", self.ink_soft),
            ("muted", self.muted),
            ("accent", self.accent),
            ("on-accent", self.on_accent),
            ("danger", self.danger),
            ("success", self.success),
        ]
    }
}

impl Blocks {
    /// Every colour by its CSS name. The website reads the Blocks palettes; the first twelve keep
    /// the names of [`Palette::named`] (`plate-soft` is `raised`, `plate` is `button`, `surface`
    /// is `panel`, `on-accent` is `on_face`), so styles written for those still apply.
    pub fn named(&self) -> [(&'static str, u32); 25] {
        [
            ("page", self.page),
            ("plate-soft", self.raised),
            ("plate", self.button),
            ("surface", self.panel),
            ("rule", self.rule),
            ("ink", self.ink),
            ("ink-soft", self.ink_soft),
            ("muted", self.muted),
            ("accent", self.accent),
            ("on-accent", self.on_face),
            ("danger", self.danger),
            ("success", self.success),
            ("warn", self.warn),
            ("side", self.side),
            ("panel", self.panel),
            ("raised", self.raised),
            ("button", self.button),
            ("button-bevel", self.button_bevel),
            ("well", self.well),
            ("bevel", self.bevel),
            ("lip", self.lip),
            ("outline", self.outline),
            ("face", self.face),
            ("face-bevel", self.face_bevel),
            ("face-lip", self.face_lip),
        ]
    }
}

/// The WCAG 2 contrast ratio of two colours.
pub fn contrast(a: u32, b: u32) -> f32 {
    fn luminance(c: u32) -> f32 {
        let ch = |s: u32| {
            let v = ((c >> s) & 255) as f32 / 255.;
            if v <= 0.03928 {
                v / 12.92
            } else {
                ((v + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * ch(16) + 0.7152 * ch(8) + 0.0722 * ch(0)
    }
    let (x, y) = (luminance(a), luminance(b));
    (x.max(y) + 0.05) / (x.min(y) + 0.05)
}

/// `tokens.css`: every token as a CSS custom property, the light palette by
/// default, the dark one when the system asks for it or the page sets `data-theme="dark"`.
pub fn css() -> String {
    let mut s = String::new();
    let num = |v: f32| {
        let t = format!("{v:.3}");
        t.trim_end_matches('0').trim_end_matches('.').to_string()
    };
    // The website wears the launcher's Blocks look; the game's menus keep LIGHT and DARK.
    let colors = |s: &mut String, p: &Blocks| {
        for (name, c) in p.named() {
            let _ = writeln!(s, "  --{name}: #{c:06x};");
        }
        let _ = writeln!(s, "  --on-face: #{:06x};", p.on_face);
        let _ = writeln!(
            s,
            "  color-scheme: {};",
            if p.dark { "dark" } else { "light" }
        );
        // The launcher's night look, for pages that wear it.
        s.push_str(&if p.dark { night::NIGHT } else { night::DAWN }.css());
        // The launcher's glass look, the same in both appearances.
        s.push_str(&glass::Glass::default_look().css());
    };
    s.push_str(
        "/* Generated from crates/horizon-theme by `cargo run -p xtask -- tokens`; do not edit.\n   \
         docs/design-system.md describes the tokens. */\n\n:root {\n",
    );
    colors(&mut s, &BLOCKS_LIGHT);
    let _ = writeln!(
        s,
        "  --font: \"{FONT}\", ui-sans-serif, system-ui, sans-serif;\n  --mono: \"{MONO}\", ui-monospace, monospace;\n  --pixel: \"{PIXEL}\", \"{FONT}\", ui-sans-serif, sans-serif;\n  --night-font: \"{}\", \"{FONT}\", ui-sans-serif, sans-serif;\n  --night-display-font: \"{}\", \"{}\", ui-sans-serif, sans-serif;",
        night::FONT,
        night::DISPLAY_FONT,
        night::FONT
    );
    for (name, v) in [
        ("block-outline", BLOCK_OUTLINE),
        ("block-bevel", BLOCK_BEVEL),
        ("block-lip", BLOCK_LIP),
    ] {
        let _ = writeln!(s, "  --{name}: {}px;", num(v));
    }
    for step in TYPE.iter().chain(&night::TYPE) {
        let size = match step.web {
            Web::Fixed => format!("{}px", num(step.px)),
            Web::Fluid(min, max) => format!(
                "clamp({}px, calc({}px + {} * (100vw - 375px) / 1065), {}px)",
                num(min),
                num(min),
                num(max - min),
                num(max)
            ),
            Web::Viewport(vw) => format!("{}vw", num(vw)),
        };
        let n = step.name;
        let _ = writeln!(s, "  --text-{n}: {size};");
        let _ = writeln!(s, "  --text-{n}-line: {};", num(step.line));
        let _ = writeln!(s, "  --text-{n}-tracking: {}em;", num(step.tracking));
        let _ = writeln!(s, "  --text-{n}-weight: {};", step.weight);
    }
    for (i, v) in SPACE.iter().enumerate() {
        let _ = writeln!(s, "  --space-{}: {}px;", i + 1, num(*v));
    }
    for (name, v) in [
        ("gutter", GUTTER),
        ("gutter-narrow", GUTTER_NARROW),
        ("radius", RADIUS),
        ("radius-plate", RADIUS_PLATE),
        ("radius-pill", RADIUS_PILL),
        ("rule-width", RULE),
        ("control", CONTROL),
        ("control-large", CONTROL_LARGE),
        ("segment", SEGMENT),
        ("field", FIELD),
        ("row", ROW_WEB),
        ("index", INDEX),
        ("plate-bleed", PLATE_BLEED),
        ("rise", RISE),
        ("night-pixel-corner", night::PIXEL_CORNER),
        ("night-radius-control", night::RADIUS_CONTROL),
        ("night-play", night::PLAY),
    ] {
        let _ = writeln!(s, "  --{name}: {}px;", num(v));
    }
    let _ = writeln!(s, "  --measure: {}ch;", num(MEASURE));
    for m in MOTION {
        let [a, b, c, d] = m.curve;
        let _ = writeln!(s, "  --{}: {}ms;", m.name, m.ms);
        let _ = writeln!(
            s,
            "  --ease-{}: cubic-bezier({}, {}, {}, {});",
            m.name,
            num(a),
            num(b),
            num(c),
            num(d)
        );
    }
    let _ = writeln!(s, "  --stagger: {}ms;", STAGGER.as_millis());
    s.push_str(
        "}\n\n@media (prefers-color-scheme: dark) {\n  :root:not([data-theme=\"light\"]) {\n",
    );
    let mut dark = String::new();
    colors(&mut dark, &BLOCKS_DARK);
    for line in dark.lines() {
        let _ = writeln!(s, "  {line}");
    }
    s.push_str("  }\n}\n\n:root[data-theme=\"dark\"] {\n");
    s.push_str(&dark);
    s.push_str("}\n");
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every text colour on every background it is drawn on reaches WCAG AA (4.5:1).
    #[test]
    fn text_meets_wcag_aa_in_both_palettes() {
        for p in [LIGHT, DARK] {
            let backgrounds = [
                ("page", p.page),
                ("plate_soft", p.plate_soft),
                ("plate", p.plate),
                ("surface", p.surface),
            ];
            let texts = [
                ("ink", p.ink),
                ("ink_soft", p.ink_soft),
                ("muted", p.muted),
                ("accent", p.accent),
                ("danger", p.danger),
                ("success", p.success),
            ];
            for (bn, bg) in backgrounds {
                for (tn, fg) in texts {
                    let ratio = contrast(fg, bg);
                    assert!(
                        ratio >= 4.5,
                        "{} {tn} on {bn}: {ratio:.2}",
                        if p.dark { "dark" } else { "light" }
                    );
                }
            }
            // Text on the accent, and page-coloured text on an ink segment.
            assert!(contrast(p.on_accent, p.accent) >= 4.5);
            assert!(contrast(p.page, p.ink) >= 4.5);
        }
    }

    /// The blocky look: every text colour on every background it is drawn on reaches AA, the
    /// main action's label too, and the focus ring (the accent) reaches 3:1 (WCAG 1.4.11).
    #[test]
    fn blocks_text_meets_wcag_aa_in_both_appearances() {
        for p in [
            BLOCKS_DARK,
            BLOCKS_LIGHT,
            night::NIGHT.blocks(),
            night::DAWN.blocks(),
        ] {
            let theme = if p.dark { "dark" } else { "light" };
            let backgrounds = [
                ("page", p.page),
                ("side", p.side),
                ("panel", p.panel),
                ("raised", p.raised),
                ("button", p.button),
                ("well", p.well),
            ];
            let texts = [
                ("ink", p.ink),
                ("ink_soft", p.ink_soft),
                ("muted", p.muted),
                ("accent", p.accent),
                ("danger", p.danger),
                ("success", p.success),
                ("warn", p.warn),
            ];
            for (bn, bg) in backgrounds {
                for (tn, fg) in texts {
                    let ratio = contrast(fg, bg);
                    assert!(ratio >= 4.5, "{theme} {tn} on {bn}: {ratio:.2}");
                }
                // The texture's lightest shade under the page's text.
                let lifted = TILE_SHADES.iter().max().copied().unwrap_or(0) as i32;
                let shade = [16, 8, 0].iter().fold(0, |out, s| {
                    let c = (((p.page >> s) & 255) as i32 + lifted).clamp(0, 255) as u32;
                    out | c << s
                });
                for (tn, fg) in texts {
                    let ratio = contrast(fg, shade);
                    assert!(
                        ratio >= 4.5,
                        "{theme} {tn} on the stone texture: {ratio:.2}"
                    );
                }
            }
            let label = contrast(p.on_face, p.face);
            assert!(label >= 4.5, "{theme} the main action's label: {label:.2}");
        }
    }

    #[test]
    fn every_icon_is_a_pixel_svg() {
        for (name, svg) in icons::ALL {
            let svg = std::str::from_utf8(svg).unwrap();
            assert!(svg.contains("viewBox=\"0 0 24 24\""), "{name}");
            assert!(svg.contains("currentColor"), "{name}");
        }
        assert!(icons::get("play").is_some());
    }

    /// The game's menus over the brightest backdrop: the page colour veiled over it, and a
    /// selected row's plate (ink at 8 %) on top.
    #[test]
    fn menu_text_meets_wcag_aa_over_a_bright_backdrop() {
        let mix = |a: u32, b: u32, t: f32| {
            [16, 8, 0].iter().fold(0, |out, s| {
                let (x, y) = (((a >> s) & 255) as f32, ((b >> s) & 255) as f32);
                out | ((x * (1. - t) + y * t).round() as u32) << s
            })
        };
        let p = DARK;
        for (veil, texts) in [
            (
                VEIL,
                &[p.ink, p.ink_soft, BACKDROP_MUTED, p.danger, p.success][..],
            ),
            (HOME_VEIL, &[p.ink, p.ink_soft][..]),
        ] {
            let bg = mix(BRIGHT_BACKDROP, p.page, veil);
            let plate = mix(bg, p.ink, 0.08);
            for &text in texts {
                for under in [bg, plate] {
                    let ratio = contrast(text, under);
                    assert!(ratio >= 4.5, "#{text:06x} over veil {veil}: {ratio:.2}");
                }
            }
        }
        assert!(contrast(BACKDROP_MUTED, p.page) >= 4.5);
    }

    #[test]
    fn contrast_matches_known_values() {
        assert!((contrast(0x000000, 0xffffff) - 21.).abs() < 0.01);
        assert!((contrast(0x777777, 0xffffff) - 4.48).abs() < 0.01);
    }

    #[test]
    fn bezier_eases_out() {
        assert_eq!(bezier(MOVE.curve, 0.), 0.);
        assert!((bezier(MOVE.curve, 1.) - 1.).abs() < 1e-4);
        assert!(MOVE.ease(0.25) > 0.6, "{}", MOVE.ease(0.25));
        assert!((bezier([0., 0., 1., 1.], 0.3) - 0.3).abs() < 1e-3);
    }

    /// `tokens.css` (beside Cargo.toml, read by the website's build) is what [`css`] writes;
    /// regenerate it with `cargo run -p xtask -- tokens`.
    #[test]
    fn tokens_css_is_up_to_date() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tokens.css");
        let written = std::fs::read_to_string(path).unwrap_or_default();
        assert!(
            written == css(),
            "crates/horizon-theme/tokens.css is out of date: run `cargo run -p xtask -- tokens`"
        );
    }
}
