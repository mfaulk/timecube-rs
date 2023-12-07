//! Simple proof that 4 harmonic corner days rotate simultaneously around squared equator and cubed Earth

/// Proof that 4 harmonic corner days rotate simultaneously around squared equator and cubed Earth
pub const proofs: [Proof; 4] = [
    Proof {
        day: Day::SUN_UP,
        time: Time::MID_DAY,
        corner: Corner::SOUTH_WEST,
        rotation: Rotation::CLOCKWISE,
    },
    Proof {
        day: Day::MID_DAY,
        time: Time::SUN_DOWN,
        corner: Corner::NORTH_WEST,
        rotation: Rotation::CLOCKWISE,
    },
    Proof {
        day: Day::SUN_DOWN,
        time: Time::MID_NIGHT,
        corner: Corner::NORTH_EAST,
        rotation: Rotation::CLOCKWISE,
    },
    Proof {
        day: Day::MID_NIGHT,
        time: Time::SUN_UP,
        corner: Corner::SOUTH_EAST,
        rotation: Rotation::CLOCKWISE,
    },
];

use crate::corner::Corner;
use crate::Day;
use crate::rotation::Rotation;
use crate::time::Time;

pub struct Proof {
    pub day: Day,
    pub time: Time,
    pub corner: Corner,
    pub rotation: Rotation,
}