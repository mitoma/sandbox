use cgmath::AbsDiffEq;
use cgmath::InnerSpace;
use cgmath::Point2;
use cgmath::Vector2;

/// 以下のサイトで提示されている 3 次ベジエ → 2 次ベジエへの 変換を実装している
/// http://nutsu.com/blog/2008/021520_as_bezierconvert.html

pub struct QuadraticBezier {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,

    pub cx0: f32,
    pub cy0: f32,
}

impl QuadraticBezier {
    pub fn calc_point(&self, t: f32) -> Option<Point2<f32>> {
        if !(0.0..=1.0).contains(&t) {
            return None;
        }

        let t_rest = 1.0 - t;

        let mx0 = self.x0 * t + (self.cx0 * t_rest);
        let my0 = self.y0 * t + (self.cy0 * t_rest);

        let mx1 = self.cx0 * t + (self.x1 * t_rest);
        let my1 = self.cy0 * t + (self.y1 * t_rest);

        Some(Point2 {
            x: mx0 * t + mx1 * t_rest,
            y: my0 * t + my1 * t_rest,
        })
    }

    fn diff(&self, t: f32) -> Vector2<f32> {
        Vector2::new(
            2.0 * (t * (self.x0 + self.x1 - 2.0 * self.cx0) - self.x0 + self.cx0),
            2.0 * (t * (self.y0 + self.y1 - 2.0 * self.cy0) - self.y0 + self.cy0),
        )
    }
    fn length(&self) -> f32 {
        let kx = self.x0 + self.x1 - 2.0 * self.cx0;
        let ky = self.y0 + self.y1 - 2.0 * self.cy0;
        let ax = -self.x0 + self.cx0;
        let ay = -self.y0 + self.cy0;
        if kx == 0.0 && ky == 0.0 {
            return 0.0;
        }

        let xy = kx * kx + ky * ky;
        let b = (ax * kx + ay * ky) / xy;
        let c = (ax * ax + ay * ay) / xy - b * b;
        let (c, cs, cs2) = if (c > 1e-10) {
            let cs = (c).sqrt();
            let cs2 = 0.0;
            (c, cs, cs2)
        } else {
            let cs = 1.0;
            let cs2 = 1.0;
            (0.0, cs, cs2)
        };

        //長さ
        let init = Self::integrate_f(0.0, b, c, xy, cs, cs2);
        let length = Self::integrate(1.0, init, b, c, xy, cs, cs2);
        length
    }

    fn integrate(t: f32, init: f32, b: f32, c: f32, xy: f32, cs: f32, cs2: f32) -> f32 {
        Self::integrate_f(t, b, c, xy, cs, cs2) - init
    }

    fn integrate_f(t: f32, b: f32, c: f32, xy: f32, cs: f32, cs2: f32) -> f32 {
        let bt: f32 = b + t;
        let bts: f32 = (bt * bt + c).sqrt();
        return (xy).sqrt() * (bts * bt + c * ((bt + bts) / cs + cs2).log10());
    }
}

pub struct CubicBezier {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,

    pub cx0: f32,
    pub cy0: f32,
    pub cx1: f32,
    pub cy1: f32,
}

impl CubicBezier {
    pub fn calc_point(&self, t: f32) -> Option<Point2<f32>> {
        if !(0.0..=1.0).contains(&t) {
            return None;
        }

        let t_rest = 1.0 - t;

        let mx0 = self.x0 * t + (self.cx0 * t_rest);
        let my0 = self.y0 * t + (self.cy0 * t_rest);

        let mx1 = self.cx0 * t + (self.cx1 * t_rest);
        let my1 = self.cy0 * t + (self.cy1 * t_rest);

        let mx2 = self.cx1 * t + (self.x1 * t_rest);
        let my2 = self.cy1 * t + (self.y1 * t_rest);

        let nx0 = mx0 * t + (mx1 * t_rest);
        let ny0 = my0 * t + (my1 * t_rest);

        let nx1 = mx1 * t + (mx2 * t_rest);
        let ny1 = my1 * t + (my2 * t_rest);

        Some(Point2 {
            x: nx0 * t + nx1 * t_rest,
            y: ny0 * t + ny1 * t_rest,
        })
    }

    fn diff(&self, t: f32) -> Vector2<f32> {
        Vector2::new(
            3.0 * (self.x1 - self.x0 - 3.0 * self.cx1 + 3.0 * self.cx0) * t * t
                + 6.0 * (self.x0 + self.cx1 - 2.0 * self.cx0) * t
                - 3.0 * self.x0
                + 3.0 * self.cx0,
            3.0 * (self.y1 - self.y0 - 3.0 * self.cy1 + 3.0 * self.cy0) * t * t
                + 6.0 * (self.y0 + self.cy1 - 2.0 * self.cy0) * t
                - 3.0 * self.y0
                + 3.0 * self.cy0,
        )
    }

    const LENGTH_CAL_STEP: f32 = 100.0;

    fn length(&self) -> f32 {
        let k = 1.0 / Self::LENGTH_CAL_STEP;
        let mut px0 = self.x0;
        let mut py0 = self.y0;
        let mut length = 0.0;
        (0..=100).for_each(|idx| {
            let t = idx as f32 * k;
            let tp = 1.0 - t;
            let px1 = self.x0 * tp * tp * tp
                + 3.0 * self.cx0 * t * tp * tp
                + 3.0 * self.cx1 * t * t * tp
                + self.x1 * t * t * t;
            let py1 = self.y0 * tp * tp * tp
                + 3.0 * self.cy0 * t * tp * tp
                + 3.0 * self.cy1 * t * t * tp
                + self.y1 * t * t * t;
            let dx = px1 - px0;
            let dy = py1 - py0;
            length += (dx * dx + dy * dy).sqrt();
            px0 = px1;
            py0 = py1;
        });
        length
    }
}

impl CubicBezier {
    pub fn to_quadratic(&self) -> Vec<QuadraticBezier> {
        let err: f32 = 0.3 * 0.3;

        let diff = self.diff(0.5);

        // 単純置換
        if let Some(qb) = self.convert_quadratic_bezier() {
            if Self::compare_diff(diff, qb.diff(0.5)) && (self.length() - qb.length()).abs() < err {
                return vec![qb];
            }
        }

        // 2分割
        {
            let (q0, q1) = self.devide_quadratic_bezier();
            if Self::compare_diff(diff, q0.diff(1.0))
                && (self.length() - q0.length() - q1.length()).abs() < err
            {
                return vec![q0, q1];
            }
        }

        // 4分割
        {
            let (c0, c1) = self.split(0.5).unwrap();
            let (q0, q1) = c0.devide_quadratic_bezier();
            let (q2, q3) = c1.devide_quadratic_bezier();
            if Self::compare_diff(c0.diff(0.5), q0.diff(1.0))
                && Self::compare_diff(c1.diff(0.5), q2.diff(1.0))
                && (self.length() - q0.length() - q1.length() - q2.length() - q3.length()).abs()
                    < err
            {
                return vec![q0, q1, q2, q3];
            }
        }

        // 6分割
        {
            let (c0, c_rest) = self.split(0.333).unwrap();
            let (c1, c2) = c_rest.split(0.5).unwrap();
            let (q0, q1) = c0.devide_quadratic_bezier();
            let (q2, q3) = c1.devide_quadratic_bezier();
            let (q4, q5) = c1.devide_quadratic_bezier();
            if Self::compare_diff(c0.diff(0.5), q0.diff(1.0))
                && Self::compare_diff(c1.diff(0.5), q2.diff(1.0))
                && Self::compare_diff(c2.diff(0.5), q4.diff(1.0))
                && (self.length()
                    - q0.length()
                    - q1.length()
                    - q2.length()
                    - q3.length()
                    - q4.length()
                    - q5.length())
                .abs()
                    < err
            {
                return vec![q0, q1, q2, q3, q4, q5];
            }
        }

        // 8分割
        {
            let (c_rest0, c_rest1) = self.split(0.5).unwrap();
            let (c0, c1) = c_rest0.split(0.5).unwrap();
            let (c2, c3) = c_rest1.split(0.5).unwrap();
            let (q0, q1) = c0.devide_quadratic_bezier();
            let (q2, q3) = c1.devide_quadratic_bezier();
            let (q4, q5) = c2.devide_quadratic_bezier();
            let (q6, q7) = c3.devide_quadratic_bezier();
            vec![q0, q1, q2, q3, q4, q5, q6, q7]
        }
    }

    fn devide_quadratic_bezier(&self) -> (QuadraticBezier, QuadraticBezier) {
        let temp_point = 1.0 - 0.25;

        let f0x = self.cx0 * temp_point + self.x0 / 4.0;
        let f0y = self.cy0 * temp_point + self.y0 / 4.0;
        let f1x = self.cx1 * temp_point + self.x1 / 4.0;
        let f1y = self.cy1 * temp_point + self.y1 / 4.0;
        let f2x = f1x * 0.5 + f0x * 0.5;
        let f2y = f1y * 0.5 + f0y * 0.5;

        let b1 = QuadraticBezier {
            x0: self.x0,
            y0: self.y0,
            x1: f2x,
            y1: f2y,
            cx0: f0x,
            cy0: f0y,
        };
        let b2 = QuadraticBezier {
            x0: f2x,
            y0: f2y,
            x1: self.x1,
            y1: self.y1,
            cx0: f1x,
            cy0: f1y,
        };
        (b1, b2)
    }

    fn convert_quadratic_bezier(&self) -> Option<QuadraticBezier> {
        let vx0 = self.cx0 - self.x0;
        let vy0 = self.cy0 - self.y0;
        let vx1 = self.cx1 - self.x1;
        let vy1 = self.cy1 - self.y1;
        let c = vx0 * vy1 - vx1 * vy0;

        if c == 0.0 {
            return None;
        }

        let a = (vx1 * self.y0 - vx1 * self.y1 + vy1 * self.x1 - vy1 * self.x0) / c;
        let b = (a * vx0 + self.x0 - self.x1) / vx1;

        if a > 0.0 && b > 0.0 {
            return Some(QuadraticBezier {
                x0: self.x0,
                y0: self.y0,
                x1: self.x1,
                y1: self.y1,
                cx0: self.x0 + a * vx0,
                cy0: self.y0 + a * vy0,
            });
        } else {
            return None;
        }
    }

    fn compare_diff(dc: Vector2<f32>, dp: Vector2<f32>) -> bool {
        let dc = dc.normalize();
        let dp = dp.normalize();
        (dp.x - dc.x).abs() < 0.01 && (dp.y - dc.y).abs() < 0.01
    }

    fn split(&self, t: f32) -> Option<(CubicBezier, CubicBezier)> {
        if !(0.0..=1.0).contains(&t) {
            return None;
        }

        let tp: f32 = 1.0 - t;
        let mx: f32 = self.cx0 * tp + self.cx1 * t;
        let my: f32 = self.cy0 * tp + self.cy1 * t;
        let ax0: f32 = self.x0 * tp + self.cx0 * t;
        let ay0: f32 = self.y0 * tp + self.cy0 * t;
        let ax1: f32 = ax0 * tp + mx * t;
        let ay1: f32 = ay0 * tp + my * t;
        let bx1: f32 = self.cx1 * tp + self.x1 * t;
        let by1: f32 = self.cy1 * tp + self.y1 * t;
        let bx0: f32 = mx * tp + bx1 * t;
        let by0: f32 = my * tp + by1 * t;
        let px: f32 = ax1 * tp + bx0 * t;
        let py: f32 = ay1 * tp + by0 * t;
        Some((
            CubicBezier {
                x0: self.x0,
                y0: self.y0,
                x1: px,
                y1: py,
                cx0: ax0,
                cy0: ay0,
                cx1: ax1,
                cy1: ay1,
            },
            CubicBezier {
                x0: px,
                y0: py,
                x1: self.x1,
                y1: self.y1,
                cx0: bx0,
                cy0: by0,
                cx1: bx1,
                cy1: by1,
            },
        ))
    }
}
