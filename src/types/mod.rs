enum Gender {
    Male,
    Female,
    Other,
}

enum Color {
    Yellow,
    Blue,
    Black,
    Red,
    White,
}

enum Class {
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
    roomSenior: String,
    cardSenior: String,
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
    doublePassive: bool,
}

pub struct SeniorGroup {
    courseCode: String,
    // senior: Vec<Color>, Nobody cares
    preferredColor: Color,
}

impl GTBStudent {
    pub fn new(
        neptun: String,
        roomSenior: String,
        cardSenior: String,
        color: String,
    ) -> GTBStudent {
        GTBStudent {
            neptun,
            roomSenior,
            cardSenior,
            color,
        }
    }
}
