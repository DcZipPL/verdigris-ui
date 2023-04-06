pub mod grid;
pub mod space;
pub mod flex;
pub mod stack;
pub mod group;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Justify {
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
    SpaceBetween,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Align {
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Wrap {
    Wrap,
    Nowrap,
    WrapReverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Position {
    Left,
    Center,
    Right,
    Apart,
}