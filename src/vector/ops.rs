use crate::vector::Vec3;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub};

// A + B
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Vec3 {
        Self::from_array(&[self[0] + other[0], self[1] + other[1], self[2] + other[2]])
    }
}
// A - B
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Self) -> Vec3 {
        Self::from_array(&[self[0] - other[0], self[1] - other[1], self[2] - other[2]])
    }
}

// A * B
impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Vec3 {
        Self::from_array(&[self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]])
    }
}

// c * A
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs[0], self * rhs[1], self * rhs[2])
    }
}
// A / c
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0. {
            panic!("Cannot divide by zero");
        } else {
            let new_vec = [1. / rhs * self[0], 1. / rhs * self[1], 1. / rhs * self[2]];
            Vec3::from_array(&new_vec)
        }
    }
}

// A[index]
impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &f64 {
        return &self.vec3[index];
    }
}
// - A
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -1. * self
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec3[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::Vec3;
    #[test]
    fn add_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        let result = v1 + v2;

        assert_eq!(result.x(), 5.0);
        assert_eq!(result.y(), 7.0);
        assert_eq!(result.z(), 9.0);
    }

    #[test]
    fn sub_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        let result = v1 - v2;

        assert_eq!(result.x(), -3.0);
        assert_eq!(result.y(), -3.0);
        assert_eq!(result.z(), -3.0);
    }

    #[test]
    fn mul_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        let result = v1 * v2;

        assert_eq!(result.x(), 4.0);
        assert_eq!(result.y(), 10.0);
        assert_eq!(result.z(), 18.0);
    }

    #[test]
    fn div_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = f64::from(4.0);

        let result = v1 / v2;

        assert_eq!(result.x(), 0.25);
        assert_eq!(result.y(), 0.5);
        assert_eq!(result.z(), 0.75);
    }

    #[test]
    fn neg_vec3() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        let result = -v;

        assert_eq!(result.x(), -1.0);
        assert_eq!(result.y(), -2.0);
        assert_eq!(result.z(), -3.0);
    }

    #[test]
    fn index_vec3() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        let result = v[0];

        assert_eq!(result, 1.0);
    }
}
