use std::fmt::Error;

use enum_iterator::{all, Sequence};
use int_enum::{IntEnum, IntEnumError};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Sequence, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value() as u32
}

pub fn value_to_color_string(value: u32) -> String {
    ResistorColor::from_int(value as u8).map_or_else(
        |colour| format!("{colour:?}"),
        |_| String::from("value out of range"),
    )
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
