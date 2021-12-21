use super::sqrt;
use super::sqrtf::sqrtf;
use super::{fabs, get_high_word, get_low_word, sqrt, with_set_low_word};

const PIO2_HI: f64 = 1.57079632679489655800e+00;
const PIO2_LO: f64 = 6.12323399573676603587e-17;
const PS0: f64 = 1.66666666666666657415e-01;
const PS1: f64 = -3.25565818622400915405e-01;
const PS2: f64 = 2.01212532134862925881e-01;
const PS3: f64 = -4.00555345006794114027e-02;
const PS4: f64 = 7.91534994289814532176e-04;
const PS5: f64 = 3.47933107596021167570e-05;
const QS1: f64 = -2.40339491173441421878e+00;
const QS2: f64 = 2.02094576023350569471e+00;
const QS3: f64 = -6.88283971605453293030e-01;
const QS4: f64 = 7.70381505559019352791e-02;

#[inline]
pub fn acos(x: f64) -> f64{

    let x1p_120f = f64::from_bits(0x3870000000000000);
    let z: f64;
    let w: f64;
    let s: f64;
    let c: f64;
    let df: f64;
    let hx: u32;
    let ix: u32;

    hx = (x.to_bits() >> 32) as u32;
    ix = hx & 0x7fffffff;
    
    if ix >= 0x3ff00000{
        let lx: u32 = x.to_bits() as u32;
        if (ix - 0x3ff00000 | lx) == 0{
            if (hx >> 31) != 0 {
                return 2. * PIO2_HI + x1p_120f;
            }
            return 0.;
        }
        return 0. / (x - x);
    }

    if ix < 0x3fe00000{
        if ix <= 0x3c600000{
            return PIO2_HI + x1p_120f;
        }
        return PIO2_HI - (x - (PIO2_LO - x * r(x * x)));
    }
    
    if (hx >> 31) != 0{
        z = (1.0 + x) * 0.5;
        s = sqrt(z);
        w = r(z) * s - PIO2_LO;
        return 2. * (PIO2_HI - (s + w));
    }
    
    z = (1.0 - x) * 0.5;
    s = sqrt(z);
    
    df = f64::from_bits(s.to_bits() & 0xff_ff_ff_ff_00_00_00_00);

    c = (z - df * df) / (s + df);
    w = r(z) * s + c;
    return 2. * (df + w);
    
}

const PIO2_HI: f32 = 1.5707962513e+00;
const PIO2_LO: f32 = 7.5497894159e-08;
const P_S0: f32 = 1.6666586697e-01;
const P_S1: f32 = -4.2743422091e-02;
const P_S2: f32 = -8.6563630030e-03;
const Q_S1: f32 = -7.0662963390e-01;

#[inline]
pub fn acosf(x: f32) -> f32{

    let x1p_120 = f32::from_bits(0x03800000);

    let z: f32;
    let w: f32;
    let s: f32;

    let mut hx = x.to_bits();
    let ix = hx & 0x7fffffff;

    if ix >= 0x3f800000{
        if ix == 0x3f800000{
            if (hx >> 31) != 0{
                return 2. * PIO2_HI + x1p_120;
            }
            return 0.;
        }
        return 0. / (x - x);
    }

    if ix < 0x3f000000{
        if ix <= 0x32800000 {
            return PIO2_HI + x1p_120;
        }
        return PIO2_HI - (x - (PIO2_LO - x * r(x * x)));
    }
    
    if (hx >> 31) != 0{
        z = (1. + x) * 0.5;
        s = sqrtf(z);
        w = r(z) * s - PIO2_LO;
        return 2. * (PIO2_HI - (s + w));
    }
    
    z = (1. - x) * 0.5;
    s = sqrtf(z);
    hx = s.to_bits();
    let df = f32::from_bits(hx & 0xfffff000);
    let c = (z - df * df) / (s + df);
    w = r(z) * s + c;
    2. * (df + w)
}

const PIO2_HI: f64 = 1.57079632679489655800e+00;
const PIO2_LO: f64 = 6.12323399573676603587e-17;
const P_S0: f64 = 1.66666666666666657415e-01;
const P_S1: f64 = -3.25565818622400915405e-01;
const P_S2: f64 = 2.01212532134862925881e-01;
const P_S3: f64 = -4.00555345006794114027e-02;
const P_S4: f64 = 7.91534994289814532176e-04;
const P_S5: f64 = 3.47933107596021167570e-05;
const Q_S1: f64 = -2.40339491173441421878e+00;
const Q_S2: f64 = 2.02094576023350569471e+00;
const Q_S3: f64 = -6.88283971605453293030e-01;
const Q_S4: f64 = 7.70381505559019352791e-02;

#[inline]
pub fn asin(mut x: f64) -> f64{

    let z: f64;
    let r: f64;
    let s: f64;
    let hx: u32;
    let ix: u32;

    hx = get_high_word(x);
    ix = hx & 0x7fffffff;

    if ix >= 0x3ff00000{
        let lx: u32;
        lx = get_low_word(x);
        if (ix - 0x3ff00000 | lx) == 0{
            return x * PIO2_HI + f64::from_bits(0x3870000000000000);
        } else {
            return 0.0 / (x - x);
        }
    }
    
    if ix < 0x3fe00000{
        if ix < 0x3e500000 && ix >= 0x00100000{
            return x;
        } else {
            return x + x * comp_r(x * x);
        }
    }
    
    z = (1.0 - fabs(x)) * 0.5;
    s = sqrt(z);
    r = comp_r(z);
    if ix >= 0x3fef3333{
        x = PIO2_HI - (2. * (s + s * r) - PIO2_LO);
    } else {
        let f: f64;
        let c: f64;
        f = with_set_low_word(s, 0);
        c = (z - f * f) / (s + f);
        x = 0.5 * PIO2_HI - (2.0 * s * r - (PIO2_LO - 2.0 * c) - (0.5 * PIO2_HI - 2.0 * f));
    }
    if hx >> 31 != 0{
        return -x;
    } else {
        return x;
    }
}