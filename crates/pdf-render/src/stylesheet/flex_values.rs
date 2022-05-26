use serde::Deserialize;
use ts_rs::TS;


#[derive(TS, Deserialize, Clone, Copy, PartialEq, Debug)]
#[ts(export)]
pub enum Direction {
    Column,
    Row,
}

#[derive(TS, Deserialize, Clone, Copy, PartialEq, Debug)]
#[ts(export)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(TS, Deserialize, Clone, Copy, PartialEq, Debug)]
#[ts(export)]
pub enum FlexAlign {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}