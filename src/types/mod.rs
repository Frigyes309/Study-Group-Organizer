pub mod convert;

#[derive(Clone)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Clone)]
enum Color {
    Yellow,
    Blue,
    Black,
    Red,
    White,
}

#[derive(Clone)]
pub enum Class {
    Standard,
    Imsc,
    German,
    English,
}

enum Coordinate {
    X(u16),
    Y(u16),
}

pub struct GTBStudent {
    neptun: String,
    room_senior: String,
    card_senior: String,
    color: String,
}

pub struct DormitoryStudent {
    neptun: String,
    room: String,
    color: String,
}

pub struct DOStudent {
    neptun: String,
    name: String,
    zip: u32,
    gender: Gender,
    class: Class,
    double_passive: bool,
}

pub struct SeniorGroup {
    course_code: String,
    // senior: Vec<Color>, Nobody cares
    preferred_color: Color,
}

impl GTBStudent {
    pub fn new(
        neptun: String,
        room_senior: String,
        card_senior: String,
        color: String,
    ) -> GTBStudent {
        GTBStudent {
            neptun,
            room_senior,
            card_senior,
            color,
        }
    }
}
