use ggez::graphics::Rect;
use std::time::Duration;

pub fn seconds(dur: &Duration) -> f32 {
    dur.as_secs() as f32 + (dur.subsec_nanos() as f32 / 1000000000.0)
}

pub fn collides(r1: Rect, r2: Rect) -> bool {
    // Check if they are not horizontally intersecting
    if r1.x > r2.x + r2.w || r1.x + r1.w < r2.x {
        false
    }
    // Check if they are not vertically intersecting
    else if r1.y > r2.y + r2.h || r1.y + r1.h < r2.y {
        false
    }
    // If neither are true then it collides!
    else {
        true
    }
}
