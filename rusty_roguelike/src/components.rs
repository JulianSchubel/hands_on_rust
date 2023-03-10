use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy{}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render{
    /* ColorPair is a helper class from bracket-lib that stores both a foreground and background
     * colour in a struct */
    pub color: ColorPair,
    /* FontCharType is defined in bracket-lib to store a single character or glyph */
    pub glyph: FontCharType,
}
