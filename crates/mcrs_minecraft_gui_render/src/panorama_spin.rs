use bevy::prelude::*;
use bevy::time::Real;
use mcrs_minecraft_render::{PanoramaDraw, PanoramaView};

const FRAME_MS: f32 = 50.0;
const LONG_FRAME: f32 = 7.0;
const LONG_FRAME_STEP: f32 = 0.5;
const DEGREES_PER_STEP: f32 = 0.1;

pub fn panorama_shows(view: Res<PanoramaView>) -> bool {
    view.draw != PanoramaDraw::Hidden
}

pub(crate) fn spin_panorama(time: Res<Time<Real>>, view: ResMut<PanoramaView>) {
    let spin_degrees = spin(view.spin_degrees, time.delta_secs() * 1000.0);
    view.map_unchanged(|view| &mut view.spin_degrees)
        .set_if_neq(spin_degrees);
}

fn spin(degrees: f32, frame_ms: f32) -> f32 {
    let steps = frame_ms / FRAME_MS;
    let steps = if steps > LONG_FRAME {
        LONG_FRAME_STEP
    } else {
        steps
    };
    (degrees + DEGREES_PER_STEP * steps + 180.0).rem_euclid(360.0) - 180.0
}
