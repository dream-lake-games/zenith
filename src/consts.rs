use bevy::prelude::*;

pub const IDEAL_WIDTH: u32 = 320;
pub const IDEAL_HEIGHT: u32 = 180;
pub const IDEAL_VEC: UVec2 = UVec2::new(IDEAL_WIDTH, IDEAL_HEIGHT);

#[allow(nonstandard_style)]
pub const IDEAL_WIDTH_f32: f32 = IDEAL_WIDTH as f32;
#[allow(nonstandard_style)]
pub const IDEAL_HEIGHT_f32: f32 = IDEAL_HEIGHT as f32;
#[allow(nonstandard_style)]
pub const IDEAL_VEC_f32: Vec2 = Vec2::new(IDEAL_WIDTH_f32, IDEAL_HEIGHT_f32);

/// How much bigger is the window then our ideal?
pub const MENU_GROWTH: u32 = 4;
/// How much bigger is the window then our ideal? (float)
#[allow(nonstandard_style)]
pub const MENU_GROWTH_f32: f32 = MENU_GROWTH as f32;

pub const MENU_WIDTH: u32 = IDEAL_WIDTH * MENU_GROWTH;
pub const MENU_HEIGHT: u32 = IDEAL_HEIGHT * MENU_GROWTH;
pub const MENU_VEC: UVec2 = UVec2::new(MENU_WIDTH, MENU_HEIGHT);

#[allow(nonstandard_style)]
pub const MENU_WIDTH_f32: f32 = MENU_WIDTH as f32;
#[allow(nonstandard_style)]
pub const MENU_HEIGHT_f32: f32 = MENU_HEIGHT as f32;
#[allow(nonstandard_style)]
pub const MENU_VEC_f32: Vec2 = Vec2::new(MENU_WIDTH_f32, MENU_HEIGHT_f32);

/// To avoid pixels that are too obviously seen, render to a quad this much bigger than ideal,
/// and then zoom the camera out by this amount so it still is "ideal" size
pub const DETAIL_GROWTH: u32 = 4;
#[allow(nonstandard_style)]
pub const DETAIL_GROWTH_f32: f32 = DETAIL_GROWTH as f32;

// Helpful to have all the z-indices here for rendor order shenanigans
// Remember, each of these zix only effect interactions within the same layer, between layers, order is determined
pub const ZIX_BACKGROUND: f32 = -400.0;
pub const ZIX_DEBUG: f32 = 350.0;
pub const ZIX_MENU: f32 = 300.0;
pub const ZIX_PARTICLES: f32 = -300.0;
pub const ZIX_PAUSE: f32 = 200.0;
pub const ZIX_PLANET: f32 = 10.0;
pub const ZIX_SHIP: f32 = 400.0;
pub const ZIX_TRANSITION: f32 = 500.0;
pub const ZIX_MIN: f32 = -600.0; // Anything further back than this gets culled by the camera(s)
pub const ZIX_MAX: f32 = 600.0; // Anything further forward than this gets culled by the camera(s)

pub const DEFAULT_ANIMATION_FPS: f32 = 24.0;

pub const FRAMERATE: f32 = 36.0;
