use super::sqrt;
use super::sqrtf::sqrtf;
use super::{fabs, get_high_word, get_low_word, sqrt, with_set_low_word};
use super::fabsf::fabsf;
use super::sqrt::sqrt;
use super::fabs;
use core::f64;

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

const PIO2: f64 = 1.570796326794896558e+00;
const P_S0: f32 = 1.6666586697e-01;
const P_S1: f32 = -4.2743422091e-02;
const P_S2: f32 = -8.6563630030e-03;
const Q_S1: f32 = -7.0662963390e-01;

#[inline]
pub fn asinf(mut x: f32) -> f32{

    let x1p_120 = f64::from_bits(0x3870000000000000);

    let hx = x.to_bits();
    let ix = hx & 0x7fffffff;

    if ix >= 0x3f800000{
        if ix == 0x3f800000{
            return ((x as f64) * PIO2 + x1p_120) as f32;
        }
        return 0. / (x - x);
    }

    if ix < 0x3f000000{
        if (ix < 0x39800000) && (ix >= 0x00800000) {
            return x;
        }
        return x + x * r(x * x);
    }

    let z = (1. - fabsf(x)) * 0.5;
    let s = sqrt(z as f64);
    x = (PIO2 - 2. * (s + s * (r(z) as f64))) as f32;
    if (hx >> 31) != 0 {
        -x
    } else {
        x
    }
}

const ATANHI: [f64; 4] = [
    4.63647609000806093515e-01,
    7.85398163397448278999e-01,
    9.82793723247329054082e-01,
    1.57079632679489655800e+00,
];

const ATANLO: [f64; 4] = [
    2.26987774529616870924e-17,
    3.06161699786838301793e-17,
    1.39033110312309984516e-17,
    6.12323399573676603587e-17,
];

const AT: [f64; 11] = [
    3.33333333333329318027e-01,
    -1.99999999998764832476e-01,
    1.42857142725034663711e-01,
    -1.11111104054623557880e-01,
    9.09088713343650656196e-02,
    -7.69187620504482999495e-02,
    6.66107313738753120669e-02,
    -5.83357013379057348645e-02,
    4.97687799461593236017e-02,
    -3.65315727442169155270e-02,
    1.62858201153657823623e-02,
];

#[inline]
pub fn atan(x: f64) -> f64{

    let mut x = x;
    let mut ix = (x.to_bits() >> 32) as u32;
    let sign = ix >> 31;
    ix &= 0x7fff_ffff;
    if ix >= 0x4410_0000{
        if x.is_nan(){
            return x;
        }
        let z = ATANHI[3] + f64::from_bits(0x0380_0000); // 0x1p-120f
        return if sign != 0 { -z } else { z };
    }

    let id = if ix < 0x3fdc_0000{
        if ix < 0x3e40_0000{
            if ix < 0x0010_0000{
                force_eval!(x as f32);
            }
            return x;
        }
        -1
    } else {
        x = fabs(x);
        if ix < 0x3ff30000{
            if ix < 0x3fe60000{
                x = (2. * x - 1.) / (2. + x);
                0
            } else {
                x = (x - 1.) / (x + 1.);
                1
            }
        } else {
            if ix < 0x40038000{
                x = (x - 1.5) / (1. + 1.5 * x);
                2
            } else {
                x = -1. / x;
                3
            }
        }
    };

    let z = x * x;
    let w = z * z;
    
    let s1 = z * (AT[0] + w * (AT[2] + w * (AT[4] + w * (AT[6] + w * (AT[8] + w * AT[10])))));
    let s2 = w * (AT[1] + w * (AT[3] + w * (AT[5] + w * (AT[7] + w * AT[9]))));

    if id < 0 {
        return x - x * (s1 + s2);
    }

    let z = ATANHI[id as usize] - (x * (s1 + s2) - ATANLO[id as usize] - x);

    if sign != 0 {
        -z
    } else {
        z
    }
    
}