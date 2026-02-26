#[cxx::bridge]
mod ffi {
    struct BoundingSphere {
        center: [f64; 3],
        radius: f64,
    }

    unsafe extern "C++" {
        include!("miniball_rs.h");

        fn compute_miniball_cpp(points: &[[f64; 3]]) -> BoundingSphere;
    }
}

pub use crate::ffi::BoundingSphere;

impl BoundingSphere {
    pub fn from_points(points: &[[f64; 3]]) -> Self {
        ffi::compute_miniball_cpp(points)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5, "{a} != {b}");
    }

    #[test]
    fn test_two_points_diameter() {
        let points = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
        let value = BoundingSphere::from_points(&points);
        assert_close(value.center[0], 1.0);
        assert_close(value.center[1], 0.0);
        assert_close(value.center[2], 0.0);
        assert_close(value.radius, 1.0);
    }

    #[test]
    fn test_single_point() {
        let points = [[-3.5, 2.0, 7.25]];
        let value = BoundingSphere::from_points(&points);
        assert_close(value.center[0], -3.5);
        assert_close(value.center[1], 2.0);
        assert_close(value.center[2], 7.25);
        assert_close(value.radius, 0.0);
    }

    #[test]
    fn test_offset_diameter() {
        let points = [[1.0, -3.0, 2.0], [9.0, -3.0, 2.0]];
        let value = BoundingSphere::from_points(&points);
        assert_close(value.center[0], 5.0);
        assert_close(value.center[1], -3.0);
        assert_close(value.center[2], 2.0);
        assert_close(value.radius, 4.0);
    }

    #[test]
    fn test_empty_points() {
        let points: [[f64; 3]; 0] = [];
        let value = BoundingSphere::from_points(&points);
        assert_close(value.center[0], 0.0);
        assert_close(value.center[1], 0.0);
        assert_close(value.center[2], 0.0);
        assert_close(value.radius, 0.0);
    }
}
