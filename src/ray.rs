use crate::vectors::{Number, Point, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Ray<N> {
    orig: Point<N>,
    dir: Vec3<N>,
}

impl<N: Number> Ray<N> {
    pub fn origin(self) -> Point<N> {
        self.orig
    }

    pub fn direction(&self) -> Vec3<N> {
        self.dir
    }

    pub fn at(self, t: N) -> Vec3<N> {
        self.orig + self.dir * t
    }
}
