use crate::point::Point;

#[derive(Debug)]
pub struct Edge<T> {
    pub p1: Point<T>,
    pub p2: Point<T>,
}
