//! All angles in radians with 0 pointing North
//! 

pub mod vector;

use vector::Vec2d;
use std::f32::consts::PI;


/// Wind vectors have x coordinate pointing North
pub struct Wind {
    velocity: Vec2d,
}

pub struct Sail {
}

pub struct Sailboat {
}

pub struct SailboatState {
}


impl Wind {
    pub fn new(velocity: Vec2d) -> Wind {
        Wind {velocity}
    }

    /// Wind direction in degres 0ᵒ = North, 90ᵒ = East
    pub fn direction(&self) -> f32 {
        let phi =  self.velocity.angle(&Vec2d::new(1.0, 0.));
        let alpha = phi / PI * 180.0;
        let dir = if self.velocity.y > 0. { alpha } else { -alpha };
        dir
    }
}

impl Sailboat {
    pub fn new() -> Sailboat {
        Sailboat { }
    }
}


// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn wind_dir() {
        assert_abs_diff_eq!(Wind::new(Vec2d::new(1., 0.)).direction(), 0.);
        assert_abs_diff_eq!(Wind::new(Vec2d::new(3., 3.)).direction(), 45., epsilon=0.1);
        assert_abs_diff_eq!(Wind::new(Vec2d::new(-3., -3.)).direction(), -135., epsilon=0.1);
    }
}
