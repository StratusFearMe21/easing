// Copyright 2016 John Ward under MIT

use num_traits::{Float, FloatConst, FromPrimitive, Num, NumAssignOps, ToPrimitive};

macro_rules! easer {
    ($ty:tt, $s:tt, $f:ident, $t:ident, $e:expr) => {
        pub struct $t<$ty: Float + FromPrimitive + FloatConst, $s: Num + ToPrimitive + NumAssignOps + PartialOrd + Copy>
        {
            start: $ty,
            dist: $ty,
            step: $s,
            steps: $s,
        }

        pub fn $f<$ty: Float + FromPrimitive + FloatConst, $s: Num + ToPrimitive + NumAssignOps + PartialOrd + Copy>(
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
                $s: Num + ToPrimitive + NumAssignOps + PartialOrd + Copy,
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
    let y = x - T::from(1.).unwrap();
    y * y * y + T::from(1.).unwrap()
});
easer!(T, S, cubic_inout, CubicInOut, |x: T| {
    if x < T::from(0.5).unwrap() {
        T::from(4.).unwrap() * x * x * x
    } else {
        let y = x.mul_add(T::from(2.).unwrap(), T::from(-2.).unwrap());
        (y * y * y).mul_add(T::from(0.5).unwrap(), T::from(1.).unwrap())
    }
});
easer!(T, S, quartic_in, QuarticIn, |x: T| { x * x * x * x });
easer!(T, S, quartic_out, QuarticOut, |x: T| {
    let y = x - T::from(1.).unwrap();
    (y * y * y).mul_add(T::from(1.).unwrap() - x, T::from(1.).unwrap())
});
easer!(T, S, quartic_inout, QuarticInOut, |x: T| {
    if x < T::from(0.5).unwrap() {
        T::from(8.).unwrap() * x * x * x * x
    } else {
        let y = x - T::from(1.).unwrap();
        (y * y * y * y).mul_add(T::from(-8.).unwrap(), T::from(1.).unwrap())
    }
});
easer!(T, S, sin_in, SinIn, |x: T| {
    let y = (x - T::from(1.).unwrap()) * T::FRAC_PI_2();
    y.sin() + T::from(1.).unwrap()
});
easer!(T, S, sin_out, SinOut, |x: T| { (x * T::FRAC_PI_2()).sin() });
easer!(T, S, sin_inout, SinInOut, |x: T| {
    if x < T::from(0.5).unwrap() {
        T::from(0.5).unwrap()
            * (T::from(1.).unwrap()
                - (x * x)
                    .mul_add(T::from(-4.).unwrap(), T::from(1.).unwrap())
                    .sqrt())
    } else {
        T::from(0.5).unwrap()
            * ((x.mul_add(T::from(-2.).unwrap(), T::from(3.).unwrap())
                * x.mul_add(T::from(2.).unwrap(), T::from(-1.).unwrap()))
            .sqrt()
                + T::from(1.).unwrap())
    }
});
easer!(T, S, exp_in, ExpIn, |x: T| {
    if x == T::from(0.).unwrap() {
        T::from(0.).unwrap()
    } else {
        (T::from(10.).unwrap() * (x - T::from(1.).unwrap())).exp2()
    }
});
easer!(T, S, exp_out, ExpOut, |x: T| {
    if x == T::from(1.).unwrap() {
        T::from(1.).unwrap()
    } else {
        T::from(1.).unwrap() - (T::from(-10.).unwrap() * x).exp2()
    }
});
easer!(T, S, exp_inout, ExpInOut, |x: T| {
    if x == T::from(1.).unwrap() {
        T::from(1.).unwrap()
    } else if x == T::from(0.).unwrap() {
        T::from(0.).unwrap()
    } else if x < T::from(0.5).unwrap() {
        x.mul_add(T::from(20.).unwrap(), T::from(-10.).unwrap())
            .exp2()
            * T::from(0.5).unwrap()
    } else {
        x.mul_add(T::from(-20.).unwrap(), T::from(10.).unwrap())
            .exp2()
            .mul_add(T::from(-0.5).unwrap(), T::from(1.).unwrap())
    }
});
