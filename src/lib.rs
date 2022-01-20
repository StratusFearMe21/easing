// Copyright 2016 John Ward under MIT

use num_traits::{Float, FloatConst, FromPrimitive, NumAssignOps, One, ToPrimitive, Zero};

macro_rules! easer {
    ($ty:tt, $s:tt, $f:ident, $t:ident, $e:expr) => {
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
                    let x = $ty::from(self.step).unwrap() / $ty::from(self.steps).unwrap();
                    Some($e(x).mul_add(self.dist, self.start))
                }
            }
        }
    };
}

easer!(T, S, linear, Linear, |x: T| { x });
easer!(T, S, quad_in, QuadIn, |x: T| { x * x });
easer!(T, S, quad_out, QuadOut, |x: T| {
    -(x * (x - T::from(2.).unwrap()))
});
easer!(T, S, quad_inout, QuadInOut, |x: T| {
    if x < T::from(0.5).unwrap() {
        T::from(2.).unwrap() * x * x
    } else {
        (T::from(-2.).unwrap() * x * x) + x.mul_add(T::from(4.).unwrap(), T::from(-1.).unwrap())
    }
});
easer!(T, S, cubic_in, CubicIn, |x: T| { x * x * x });
easer!(T, S, cubic_out, CubicOut, |x: T| {
    let y = x - T::one();
    y * y * y + T::one()
});
easer!(T, S, cubic_inout, CubicInOut, |x: T| {
    if x < T::from(0.5).unwrap() {
        T::from(4.).unwrap() * x * x * x
    } else {
        let y = x.mul_add(T::from(2.).unwrap(), T::from(-2.).unwrap());
        (y * y * y).mul_add(T::from(0.5).unwrap(), T::one())
    }
});
easer!(T, S, quartic_in, QuarticIn, |x: T| { x * x * x * x });
easer!(T, S, quartic_out, QuarticOut, |x: T| {
    let y = x - T::one();
    (y * y * y).mul_add(T::one() - x, T::one())
});
easer!(T, S, quartic_inout, QuarticInOut, |x: T| {
    if x < T::from(0.5).unwrap() {
        T::from(8.).unwrap() * x * x * x * x
    } else {
        let y = x - T::one();
        (y * y * y * y).mul_add(T::from(-8.).unwrap(), T::one())
    }
});
easer!(T, S, sin_in, SinIn, |x: T| {
    let y = (x - T::one()) * T::FRAC_PI_2();
    y.sin() + T::one()
});
easer!(T, S, sin_out, SinOut, |x: T| { (x * T::FRAC_PI_2()).sin() });
easer!(T, S, sin_inout, SinInOut, |x: T| {
    if x < T::from(0.5).unwrap() {
        T::from(0.5).unwrap() * (T::one() - (x * x).mul_add(T::from(-4.).unwrap(), T::one()).sqrt())
    } else {
        T::from(0.5).unwrap()
            * ((x.mul_add(T::from(-2.).unwrap(), T::from(3.).unwrap())
                * x.mul_add(T::from(2.).unwrap(), -T::one()))
            .sqrt()
                + T::one())
    }
});
easer!(T, S, exp_in, ExpIn, |x: T| {
    if x.is_zero() {
        T::zero()
    } else {
        (T::from(10.).unwrap() * (x - T::one())).exp2()
    }
});
easer!(T, S, exp_out, ExpOut, |x: T| {
    if x.is_one() {
        T::one()
    } else {
        T::one() - (T::from(-10.).unwrap() * x).exp2()
    }
});
easer!(T, S, exp_inout, ExpInOut, |x: T| {
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
