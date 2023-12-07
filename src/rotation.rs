//! Rotation of 4 harmonic corner days around squared equator and cubed Earth

pub enum Rotation {
    // 1 24-hour rotation of Earth
    // 4 simultaneous 24-hour days within 1 rotation of Earth
    // 4 harmonic corner days rotate simultaneously around squared equator and cubed Earth
    // 16 corners, 96 hours, 4 simultaneous 24 hour days
    // 32 moon phases, 4 corner harmonic times, 4 corner harmonic days
    CLOCKWISE,
    COUNTER_CLOCKWISE,
}