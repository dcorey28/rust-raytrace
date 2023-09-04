use num;
use std::ops;

pub trait Number:
    num::Num
    + num::NumCast
    + num::FromPrimitive
    + Clone
    + Copy
    + ops::Add
    + ops::AddAssign
    + ops::Sub
    + ops::SubAssign
    + ops::Mul
    + ops::MulAssign
    + ops::Div
    + ops::DivAssign
    + ops::Neg
{
}

impl<N> Number for N where
    N: num::Num
        + num::NumCast
        + num::FromPrimitive
        + Clone
        + Copy
        + ops::Add
        + ops::AddAssign
        + ops::Sub
        + ops::SubAssign
        + ops::Mul
        + ops::MulAssign
        + ops::Div
        + ops::DivAssign
        + ops::Neg
{
}

#[derive(PartialEq, Debug)]
pub struct Vec3<N> {
    pub x: N,
    pub y: N,
    pub z: N,
}

type Point<N> = Vec3<N>;

impl<N: Number> Vec3<N> {
    /// Calculates the magnitude of the vector.
    pub fn magnitude(&self) -> N {
        let f = self.magnitude_squared().to_f64().unwrap();
        N::from_f64(f.sqrt()).unwrap()
    }

    /// Calculates the squared magnitude of the vector.
    fn magnitude_squared(&self) -> N {
        self * self
    }

    /// Calculates the cross product of two vectors.
    fn cross(lhs: &Self, rhs: &Self) -> Self {
        Vec3 {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }

    /// Calculates the unit vector of the vector.
    fn unit(&self) -> Self {
        self / self.magnitude()
    }
}

// Add two vectors
impl<N: Number> ops::Add for Vec3<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// Add another vector to current vector
impl<N: Number> ops::AddAssign for Vec3<N> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

// Substract one vector from another
impl<N: Number> ops::Sub for Vec3<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// Subtract a vector from current vector
impl<N: Number> ops::SubAssign for Vec3<N> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

// Divide a vector by a scalar
impl<N: Number> ops::Div<N> for Vec3<N> {
    type Output = Self;

    fn div(self, scalar: N) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

// Divide a vector by a scalar
impl<N: Number> ops::Div<N> for &Vec3<N> {
    type Output = Vec3<N>;

    fn div(self, scalar: N) -> Self::Output {
        Self::Output {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

// Divide the current vector by a scalar
impl<N: Number> ops::DivAssign<N> for Vec3<N> {
    fn div_assign(&mut self, scalar: N) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

// Multiply a vector by a scalar
impl<N: Number> ops::Mul<N> for Vec3<N> {
    type Output = Self;

    fn mul(self, scalar: N) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

// Multiply the current vector by a scalar
impl<N: Number> ops::MulAssign<N> for Vec3<N> {
    fn mul_assign(&mut self, scalar: N) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

// Multiply a vector by a vector as the dot product
impl<N: Number> ops::Mul for &Vec3<N> {
    type Output = N;

    fn mul(self, other: &Vec3<N>) -> N {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

// Negate the vector
impl<N: ops::Neg<Output = N>> ops::Neg for Vec3<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Vec3 { x: 1, y: 2, z: 3 };
        let b = Vec3 { x: 4, y: 5, z: 6 };

        let expected = Vec3 { x: 5, y: 7, z: 9 };

        let actual = a + b;

        assert_eq!(expected, actual, "Vec3 add should work as expected");
    }

    #[test]
    fn test_add_assign() {
        let mut a = Vec3 { x: 1, y: 2, z: 3 };
        let b = Vec3 { x: 4, y: 5, z: 6 };

        let expected = Vec3 { x: 5, y: 7, z: 9 };

        a += b;

        assert_eq!(expected, a, "Vec3 add_assign should work as expected");
    }

    #[test]
    fn test_sub() {
        let a = Vec3 { x: 3, y: 2, z: 1 };
        let b = Vec3 { x: 4, y: 5, z: 6 };

        let expected = Vec3 {
            x: -1,
            y: -3,
            z: -5,
        };

        let actual = a - b;

        assert_eq!(expected, actual, "Vec3 sub should work as expected");
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Vec3 { x: 3, y: 2, z: 1 };
        let b = Vec3 { x: 4, y: 5, z: 6 };

        let expected = Vec3 {
            x: -1,
            y: -3,
            z: -5,
        };

        a -= b;

        assert_eq!(expected, a, "Vec3 sub_assign should work as expected");
    }

    #[test]
    fn test_mul_scalar() {
        let a = Vec3 { x: 3, y: 2, z: 1 };

        let expected = Vec3 { x: 6, y: 4, z: 2 };

        let actual = a * 2;

        assert_eq!(
            expected, actual,
            "Vec3 multiplied by scalar should multiply each element by the scalar"
        );
    }

    #[test]
    fn test_mul_assign_scalar() {
        let mut a = Vec3 { x: 3, y: 2, z: 1 };

        let expected = Vec3 { x: 6, y: 4, z: 2 };

        a *= 2;

        assert_eq!(
            expected, a,
            "Vec3 multiplied by scalar should multiply each element by the scalar"
        );
    }

    #[test]
    fn test_div_scalar() {
        let a = Vec3 { x: 6, y: 4, z: 2 };

        let expected = Vec3 { x: 3, y: 2, z: 1 };

        let actual = a / 2;

        assert_eq!(
            expected, actual,
            "Vec3 divided by scalar should divide each element by the scalar"
        );
    }

    #[test]
    fn test_div_assign_scalar() {
        let mut a = Vec3 { x: 6, y: 4, z: 2 };

        let expected = Vec3 { x: 3, y: 2, z: 1 };

        a /= 2;

        assert_eq!(
            expected, a,
            "Vec3 divided by scalar should divide each element by the scalar"
        );
    }

    #[test]
    fn test_mul_by_vector() {
        let a = Vec3 { x: 1, y: 2, z: 3 };
        let b = Vec3 { x: 4, y: 5, z: 6 };

        assert_eq!(
            &a * &b,
            32,
            "Vec3 multiplied by a Vec3 should perform the dot product of the two vectors"
        );
    }

    #[test]
    fn test_magnitude() {
        let a = Vec3 { x: 3, y: 2, z: 1 };

        assert_eq!(
            a.magnitude(),
            (14 as f64).sqrt() as i32,
            "Vec3 magnitude should correctly calculate the magnitude"
        );
    }

    #[test]
    fn test_negate_vector() {
        let a = Vec3 { x: 3, y: -2, z: 0 };

        let expected = Vec3 { x: -3, y: 2, z: 0 };

        assert_eq!(
            -a, expected,
            "vector negation should negate each element of the vector"
        )
    }

    #[test]
    fn test_cross_product() {
        let a = Vec3 { x: 1, y: 2, z: 3 };
        let b = Vec3 { x: 4, y: 5, z: 6 };

        let expected = Vec3 { x: -3, y: 6, z: -3 };

        assert_eq!(
            Vec3::cross(&a, &b),
            expected,
            "cross product of two 3d vectors should work as expected"
        )
    }

    #[test]
    fn test_unit_vector() {
        let a = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let u = 1.0 / (3.0 as f64).sqrt();

        let expected = Vec3 { x: u, y: u, z: u };

        assert_eq!(
            a.unit(),
            expected,
            "should calculate the unit vector correctly"
        )
    }
}
