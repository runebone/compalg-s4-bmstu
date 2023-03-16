#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}
pub type Points<T> = Vec<Point<T>>;
