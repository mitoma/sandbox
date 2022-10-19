use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Point {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Point {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Triangle(Point, Point, Point);

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        Self(p1, p2, p3)
    }

    pub fn in_triangle(&self, p: &Point) -> bool {
        let ab = self.0 - self.1;
        let bc = self.1 - self.2;
        let ca = self.2 - self.0;

        let ap = self.0 - *p;
        let bp = self.1 - *p;
        let cp = self.2 - *p;

        let o1 = ab.x * bp.y - ab.y * bp.x;
        let o2 = bc.x * cp.y - bc.y * cp.x;
        let o3 = ca.x * ap.y - ca.y * ap.x;

        (o1 > 0.0 && o2 > 0.0 && o3 > 0.0) || (o1 < 0.0 && o2 < 0.0 && o3 < 0.0)
    }

    #[inline]
    fn outer_prod(a: Point, b: Point) -> f32 {
        a.x * b.y - a.y * b.x
    }

    pub fn in_besie(&self, p: &Point) -> bool {
        let ab = self.0 - self.1;
        let bc = self.1 - self.2;
        let ca = self.2 - self.0;
        let ac = self.0 - self.2;

        let ap = self.0 - *p;
        let bp = self.1 - *p;
        let cp = self.2 - *p;

        let total_area = Self::outer_prod(ab, ac).abs();

        let s = Self::outer_prod(ab, ap).abs() / total_area;
        let t = Self::outer_prod(bc, bp).abs() / total_area;
        let r = Self::outer_prod(ca, cp).abs() / total_area;

        // 丸め誤差
        if -0.00001 <= (s + t + r)
            && (s + t + r) <= 1.00001
            && (0.0..=1.0).contains(&s)
            && (0.0..=1.0).contains(&t)
            && (0.0..=1.0).contains(&r)
        {
            return (s / 2.0 + t).powi(2) < t;
        }
        false
    }
}
