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
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,

    ab: Point,
    bc: Point,
    ca: Point,

    total_area: f32,
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        let xs = [a.x, b.x, c.x];
        let min_x = xs.into_iter().reduce(f32::min).unwrap();
        let max_x = xs.into_iter().reduce(f32::max).unwrap();
        let xy = [a.y, b.y, c.y];
        let min_y = xy.into_iter().reduce(f32::min).unwrap();
        let max_y = xy.into_iter().reduce(f32::max).unwrap();

        Self {
            a,
            b,
            c,
            ab: a - b,
            bc: b - c,
            ca: c - a,
            total_area: Self::outer_prod(a - b, a - c).abs(),
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    /// 点が三角形の内部にあるかどうかを判定する
    pub fn in_triangle(&self, p: &Point) -> bool {
        if !self.in_rect(p) {
            return false;
        }

        let ap = self.a - *p;
        let bp = self.b - *p;
        let cp = self.c - *p;

        let o1 = self.ab.x * bp.y - self.ab.y * bp.x;
        let o2 = self.bc.x * cp.y - self.bc.y * cp.x;
        let o3 = self.ca.x * ap.y - self.ca.y * ap.x;

        (o1 > 0.0 && o2 > 0.0 && o3 > 0.0) || (o1 < 0.0 && o2 < 0.0 && o3 < 0.0)
    }

    /// 外積を計算する
    #[inline]
    fn outer_prod(a: Point, b: Point) -> f32 {
        a.x * b.y - a.y * b.x
    }

    /// 点が三角形を内包する矩形の内部にあるかどうかを判定する
    #[inline]
    fn in_rect(&self, p: &Point) -> bool {
        (self.min_x..=self.max_x).contains(&p.x) && (self.min_y..=self.max_y).contains(&p.y)
    }

    pub fn in_besie(&self, p: &Point) -> bool {
        // 矩形内になければそもそもすべてベジエ曲線のの範囲の外
        if !self.in_rect(p) {
            return false;
        }

        let ap = self.a - *p;
        let bp = self.b - *p;
        let cp = self.c - *p;

        let s = Self::outer_prod(self.ab, ap).abs() / self.total_area;
        let t = Self::outer_prod(self.bc, bp).abs() / self.total_area;
        let r = Self::outer_prod(self.ca, cp).abs() / self.total_area;

        // 丸め誤差
        let zero = -0.00001;
        let one = 1.00001;
        let zero_one_range = zero..=one;

        if zero <= (s + t + r)
            && (s + t + r) <= one
            && zero_one_range.contains(&s)
            && zero_one_range.contains(&t)
            && zero_one_range.contains(&r)
        {
            return (s / 2.0 + t).powi(2) < t;
        }
        false
    }
}
