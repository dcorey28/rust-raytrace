use crate::vectors::{Number, Point, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Ray<N> {
    pub orig: Point<N>,
    pub dir: Vec3<N>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vectors;

    #[test]
    fn test_origin() {
        let expected_orig = vectors::Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let r = Ray {
            orig: expected_orig,
            dir: Vec3::zero(),
        };

        assert_eq!(r.origin(), expected_orig, "origin should match expected");
    }

    #[test]
    fn test_direction() {
        let expected_dir = vectors::Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let r = Ray {
            orig: Vec3::zero(),
            dir: expected_dir,
        };

        assert_eq!(r.direction(), expected_dir, "origin should match expected");
    }

    #[test]
    fn test_at() {
        let r = Ray {
            orig: Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            dir: Vec3 {
                x: 1.0,
                y: 0.5,
                z: -1.0,
            },
        };

        assert_eq!(
            r.at(0.0),
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            "at(0.0) should return the origin"
        );
        assert_eq!(
            r.at(1.0),
            Vec3 {
                x: 2.0,
                y: 2.5,
                z: 2.0,
            },
            "at(1.0) should advance a full step"
        );
        assert_eq!(
            r.at(0.5),
            Vec3 {
                x: 1.5,
                y: 2.25,
                z: 2.5,
            },
            "at(0.5) should advance a half step"
        );
        assert_eq!(
            r.at(-1.0),
            Vec3 {
                x: 0.0,
                y: 1.5,
                z: 4.0,
            },
            "at(-1.0) should regress a full step"
        );
    }
}
