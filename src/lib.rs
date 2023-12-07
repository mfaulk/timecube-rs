/// 4-simultaneous 24-hour Days within a single rotation of Earth
pub struct TimeCube {
    instant: [Day; 4],
}

impl TimeCube {
    /// Time is Cubic and Absolute
    pub fn new() -> TimeCube {
        TimeCube {
            instant: [Day::SUN_UP, Day::MID_DAY, Day::SUN_DOWN, Day::MID_NIGHT],
        }
    }

    /// Time is Cubic and Absolute
    pub fn get_time(&self) -> [Day; 4] {
        self.instant.clone()
    }

    ///
    pub fn get_day(&self) -> Day {
        Day::SUN_UP
    }

    /// Opposite poles create opposite directions and opposite hemispheres.
    pub fn opposite(&mut self) {
        self.instant.reverse();
    }

    /// 4-simultaneous rotations of Earth within a single rotation of the Moon
    pub fn rotate(&mut self) {
        self.instant.rotate_left(1);
    }

    /// Universal Time Cube
    pub fn utc(&self) -> String {
        let mut utc = String::new();
        for day in self.instant.iter() {
            utc.push_str(&format!("{:?} ", day));
        }
        utc
    }
}

/// 4-simultaneous 24-hour Days within a single rotation of Earth
///
/// When the Sun shines upon Earth, 2 – major Time points are created on opposite sides of Earth
/// – known as Midday and Midnight. Where the 2 major Time forces join, synergy creates 2 new minor
/// Time points we recognize as Sunup and Sundown. The 4-equidistant time points can be considered
/// as Time Square imprinted upon the circle of Earth. In a single rotation of the Earth sphere,
/// each Time corner point rotates through the other 3-corner Time points, thus creating 16 corners,
/// 96 hours and 4-simultaneous 24-hour Days within a single rotation of Earth –
/// equated to a Higher Order of Life Time Cube.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Day {
    SUN_UP,
    MID_DAY,
    SUN_DOWN,
    MID_NIGHT
}

