// Copyright 2016 John Ward under MIT

use num_traits::{Float, FloatConst, FromPrimitive, NumAssignOps, One, ToPrimitive, Zero};

macro_rules! easer {
    ($ty:tt, $s:tt, $f:ident, $g:ident, $t:ident, $e:expr) => {
        pub struct $t<$ty: Float + FromPrimitive + FloatConst, $s: ToPrimitive + One + Zero + NumAssignOps + PartialOrd + Copy>
        {
            start: $ty,
            dist: $ty,
            step: $s,
            steps: $s,
        }

        pub fn $f<$ty: Float + FromPrimitive + FloatConst, $s: ToPrimitive + One + Zero + NumAssignOps + PartialOrd + Copy>(
            start: $ty,
            end: $ty,
            steps: $s,
            ) -> $t<$ty, $s> {
            $t {
                start: start,
                dist: end - start,
                step: $s::zero(),
                steps: steps,
            }
        }

        impl<
            $ty: Float + FromPrimitive + FloatConst,
            $s: ToPrimitive + One + Zero + NumAssignOps + PartialOrd + Copy,
            > Iterator for $t<$ty, $s>
            {
                type Item = $ty;

                fn next(&mut self) -> Option<$ty> {
                    self.step += $s::one();
                    if self.step > self.steps {
                        None
                    } else {
                        Some($e($ty::from(self.step).unwrap() / $ty::from(self.steps).unwrap()).mul_add(self.dist, self.start))
                    }
                }
            }

        impl<
            $ty: Float + FromPrimitive + FloatConst,
            $s: ToPrimitive + One + Zero + NumAssignOps + PartialOrd + Copy,
            > $t<$ty, $s> {
                pub fn get_step(&mut self, step: $s) -> Option<$ty> {
                    self.step += step;
                    if self.step > self.steps {
                        None
                    } else {
                        Some($e($ty::from(self.step).unwrap() / $ty::from(self.steps).unwrap()).mul_add(self.dist, self.start))
                    }
                }
            }

        pub fn $g <
            $ty: Float + FromPrimitive + FloatConst,
            $s: ToPrimitive + One + Zero + NumAssignOps + PartialOrd + Copy,
            >(start: $ty, end: $ty, steps: $s, step: $s) -> $ty {

                $e($ty::from(step).unwrap() / $ty::from(steps).unwrap()).mul_add(start - end, start)
            }
    };
}

easer!(T, S, linear, get_step_linear, Linear, |x: T| { x });
easer!(T, S, quad_in, get_step_quad_in, QuadIn, |x: T| {
    x.powi(2)
});
easer!(T, S, quad_out, get_step_quad_out, QuadOut, |x: T| {
    -(x * (x - T::from(2.).unwrap()))
});
easer!(T, S, quad_inout, get_step_quad_inout, QuadInOut, |x: T| {
    if x < T::from(0.5).unwrap() {
        T::from(2.).unwrap() * x.powi(2)
    } else {
        (T::from(-2.).unwrap() * x.powi(2)) + x.mul_add(T::from(4.).unwrap(), -T::one())
    }
});
easer!(T, S, cubic_in, get_step_cubic_in, CubicIn, |x: T| {
    x.powi(3)
});
easer!(T, S, cubic_out, get_step_cubic_out, CubicOut, |x: T| {
    (x - T::one()).powi(3) + T::one()
});
easer!(
    T,
    S,
    cubic_inout,
    get_step_cubic_inout,
    CubicInOut,
    |x: T| {
        if x < T::from(0.5).unwrap() {
            T::from(4.).unwrap() * x.powi(3)
        } else {
            x.mul_add(T::from(2.).unwrap(), T::from(-2.).unwrap())
                .powi(3)
                .mul_add(T::from(0.5).unwrap(), T::one())
        }
    }
);
easer!(T, S, quartic_in, get_step_quartic_in, QuarticIn, |x: T| {
    x.powi(4)
});
easer!(
    T,
    S,
    quartic_out,
    get_step_quartic_out,
    QuarticOut,
    |x: T| { (x - T::one()).powi(3).mul_add(T::one() - x, T::one()) }
);
easer!(
    T,
    S,
    quartic_inout,
    get_step_quartic_inout,
    QuarticInOut,
    |x: T| {
        if x < T::from(0.5).unwrap() {
            T::from(8.).unwrap() * x.powi(4)
        } else {
            (x - T::one())
                .powi(4)
                .mul_add(T::from(-8.).unwrap(), T::one())
        }
    }
);
easer!(T, S, sin_in, get_step_sin_in, SinIn, |x: T| {
    ((x - T::one()) * T::FRAC_PI_2()).sin() + T::one()
});
easer!(T, S, sin_out, get_step_sin_out, SinOut, |x: T| {
    (x * T::FRAC_PI_2()).sin()
});
easer!(T, S, sin_inout, get_step_sin_inout, SinInOut, |x: T| {
    if x < T::from(0.5).unwrap() {
        T::from(0.5).unwrap()
            * (T::one() - x.powi(2).mul_add(T::from(-4.).unwrap(), T::one()).sqrt())
    } else {
        T::from(0.5).unwrap()
            * ((x.mul_add(T::from(-2.).unwrap(), T::from(3.).unwrap())
                * x.mul_add(T::from(2.).unwrap(), -T::one()))
            .sqrt()
                + T::one())
    }
});
easer!(T, S, exp_in, get_step_exp_in, ExpIn, |x: T| {
    if x.is_zero() {
        T::zero()
    } else {
        (T::from(10.).unwrap() * (x - T::one())).exp2()
    }
});
easer!(T, S, exp_out, get_step_exp_out, ExpOut, |x: T| {
    if x.is_one() {
        T::one()
    } else {
        T::one() - (T::from(-10.).unwrap() * x).exp2()
    }
});
easer!(T, S, exp_inout, get_step_exp_inout, ExpInOut, |x: T| {
    if x.is_one() {
        T::one()
    } else if x.is_zero() {
        T::zero()
    } else if x < T::from(0.5).unwrap() {
        x.mul_add(T::from(20.).unwrap(), T::from(-10.).unwrap())
            .exp2()
            * T::from(0.5).unwrap()
    } else {
        x.mul_add(T::from(-20.).unwrap(), T::from(10.).unwrap())
            .exp2()
            .mul_add(T::from(-0.5).unwrap(), T::one())
    }
});

#[cfg(test)]
mod test {
    use super::*;

    // Rounds a float to five places after the decimal.
    // We can't use the exact value because of floating point
    // problems (13 != 12.999999999999999), and five decimal
    // points of precision is fine for this.
    fn round_5(x: f64) -> f64 {
        (x * 10E+5).round() / 10E+5
    }

    #[test]
    fn linear_test() {
        let model = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
        let try_: Vec<f64> = linear(0f64, 1f64, 10).collect();
        assert_eq!(try_, model);
    }

    #[test]
    fn quad_in_test() {
        let model = vec![
            100., 400., 900., 1600., 2500., 3600., 4900., 6400., 8100., 10000.,
        ];
        let try_: Vec<f64> = quad_in(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try_, model);
    }

    #[test]
    fn quad_out_test() {
        let model = vec![
            1900., 3600., 5100., 6400., 7500., 8400., 9100., 9600., 9900., 10000.,
        ];
        let try_: Vec<f64> = quad_out(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try_, model);
    }

    #[test]
    fn quad_inout_test() {
        let model = vec![
            200., 800., 1800., 3200., 5000., 6800., 8200., 9200., 9800., 10000.,
        ];
        let try_: Vec<f64> = quad_inout(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try_, model);
    }

    #[test]
    fn cubic_in_test() {
        let model = vec![
            10., 80., 270., 640., 1250., 2160., 3430., 5120., 7290., 10000.,
        ];
        let try_: Vec<f64> = cubic_in(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try_, model);
    }

    #[test]
    fn cubic_out_test() {
        let model = vec![
            2710., 4880., 6570., 7840., 8750., 9360., 9730., 9920., 9990., 10000.,
        ];
        let try_: Vec<f64> = cubic_out(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try_, model);
    }

    #[test]
    fn quartic_in_test() {
        let model = vec![1., 16., 81., 256., 625., 1296., 2401., 4096., 6561., 10000.];
        let try_: Vec<f64> = quartic_in(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try_, model);
    }

    #[test]
    fn quartic_out_test() {
        let model = vec![
            3439., 5904., 7599., 8704., 9375., 9744., 9919., 9984., 9999., 10000.,
        ];
        let try_: Vec<f64> = quartic_out(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try_, model);
    }

    #[test]
    fn quartic_inout_test() {
        let model = vec![
            8., 128., 648., 2048., 5000., 7952., 9352., 9872., 9992., 10000.,
        ];
        let try_: Vec<f64> = quartic_inout(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try_, model);
    }

    #[test]
    fn sin_in_test() {
        let model = vec![
            123.116594,
            489.434837,
            1089.934758,
            1909.830056,
            2928.932188,
            4122.147477,
            5460.095003,
            6909.830056,
            8435.655350,
            10000.,
        ];
        let try_: Vec<f64> = sin_in(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try_, model);
    }

    #[test]
    fn sin_out_test() {
        let model = vec![
            1564.344650,
            3090.169944,
            4539.904997,
            5877.852523,
            7071.067812,
            8090.169944,
            8910.065242,
            9510.565163,
            9876.883406,
            10000.,
        ];
        let try_: Vec<f64> = sin_out(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try_, model);
    }

    #[test]
    fn sin_inout_test() {
        let model = vec![
            101.020514,
            417.424305,
            1000.,
            2000.,
            5000.,
            8000.,
            9000.,
            9582.575695,
            9898.979486,
            10000.,
        ];
        let try_: Vec<f64> = sin_inout(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try_, model);
    }

    #[test]
    fn exp_in_test() {
        let model = vec![
            19.53125, 39.0625, 78.125, 156.25, 312.5, 625., 1250., 2500., 5000., 10000.,
        ];
        let try_: Vec<f64> = exp_in(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try_, model);
    }

    #[test]
    fn exp_out_test() {
        let model = vec![
            5000., 7500., 8750., 9375., 9687.5, 9843.75, 9921.875, 9960.9375, 9980.46875, 10000.,
        ];
        let try_: Vec<f64> = exp_out(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try_, model);
    }

    #[test]
    fn exp_inout_test() {
        let model = vec![
            19.53125, 78.125, 312.5, 1250., 5000., 8750., 9687.5, 9921.875, 9980.46875, 10000.,
        ];
        let try_: Vec<f64> = exp_inout(0f64, 10000f64, 10).map(round_5).collect();
        assert_eq!(try_, model);
    }
}
