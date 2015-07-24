//! Low-level bindings to libm

#![cfg(target_os = "linux")]

extern crate linux_api;

use linux_api::{c_double, c_float,
                c_long, c_longlong};

#[link(name = "m")] //For -lm
extern {

    //cos
    //sin
    //tan
    //acos
    //asin
    //atan
    //atan2
    
    //cosh
    //sinh
    //tanh
    //acosh
    //asinh
    //atanh
    
    //exp
    //frexp
    //ldexp
    //log
    //log10
    //modf
    //exp2
    //expm1
    //ilogb
    //log1p
    //log2
    //logb
    //scalbn
    //scalbln
    
    //pow
    //sqrt
    //cbrt
    //hopot
    
    //erf
    //erfc
    //tgamma
    //lgamma
    
    //ceil
    //floor
    //fmod
    //trunc
    //round
    //lround
    //llround

    ///Rounds to an integer value in c_double format
    ///
    ///If you want an integer format, use lrint instead
    pub fn rint(x: c_double) -> c_double;
    
    ///Rounds to an integer value in c_float format
    ///
    ///If you want an integer format, use lrintf instead
    pub fn rintf(x: c_float) -> c_float;
    
    ///Rounds a c_double to the nearest c_long
    pub fn lrint(x: c_double) -> c_long;
    
    ///Rounds a c_float to the nearest c_long
    pub fn lrintf(x: c_float) -> c_long;
    
    ///Rounds a c_double to the nearest c_longlong
    pub fn llrint(x: c_double) -> c_longlong;
    
    ///Rounds a c_float to the nearest c_longlong
    pub fn llrintf(x: c_float) -> c_longlong;

    ///Rounds to an integer value in c_double format
    ///
    ///If you want an integer format, use lrint instead
    pub fn nearbyint(x: c_double) -> c_double;
    
    ///Rounds to an integer value in c_float format
    ///
    ///If you want an integer format, use lrintf instead
    pub fn nearbyintf(x: c_float) -> c_float;
    
    ///Returns the floating-point remainder of numer / denom
    pub fn remainder(numer: c_double, denom: c_double) -> c_double;
    
    ///Returns the floating-point remainder of numer / denom
    pub fn remainderf(number: c_float, denom: c_float) -> c_float;
    
    //remquo
    
    //copysign
    //nan
    //nextafter
    //nexttoward
    
    //fdim
    //fmax
    //fmin
    
    //fabs
    //abs
    //fma

}

#[cfg(test)]
#[allow(unused_assignments)] // Unsafe functions
mod test {

    extern crate linux_api;
    
    use linux_api::{c_long, c_longlong, c_float, c_double};
    
    use super::*;
    
    #[test]
    fn test_rint() {
    
        let value = 246.246f64;
        
        let mut result = 0f64;
        
        unsafe { result = rint(value); }
        
        assert_eq!(246.0f64, result);
    }
    
    #[test]
    fn test_rintf() {
    
        let value = 2543637.536f32;
        
        let mut result = 0f32;
        
        unsafe { result = rintf(value); }
        
        assert_eq!(2543638.0f32, result);
    }
    
    #[test]
    fn test_nearbyint() {
    
        let value = 246.246f64;
        
        let mut result = 0f64;
        
        unsafe { result = nearbyint(value); }
        
        assert_eq!(246.0f64, result);
    }
    
    #[test]
    fn test_nearbyintf() {
    
        let value = 2543637.536f32;
        
        let mut result = 0f32;
        
        unsafe { result = nearbyintf(value); }
        
        assert_eq!(2543638.0f32, result);
    }
    
    #[test]
    fn test_lrint() {
        
        let value = 2464326.432f64;
        
        let mut result: c_long = 0;
        
        unsafe { result = lrint(value); }
        
        assert_eq!(2464326, result);
        
    }
    
    #[test]
    fn test_lrintf() {
        
        let value = 2464326.432f32;
        
        let mut result: c_long = 0;
        
        unsafe { result = lrintf(value); }
        
        assert_eq!(2464326, result);
        
    }
    
    #[test]
    fn test_llrint() {
        
        let value = 46873283884.432f64;
        
        let mut result: c_longlong = 0;
        
        unsafe { result = llrint(value); }
        
        assert_eq!(46873283884, result);
        
    }
    
    #[test]
    fn test_llrintf() {
        
        let value = 4687328.9732f32;
        
        let mut result: c_longlong = 0;
        
        unsafe { result = llrintf(value); }
        
        assert_eq!(4687329, result);
        
    }
    
    #[test]
    fn test_remainder() {
        let number: c_double = 18.5;
        let denominator: c_double = 4.2;
        
        unsafe {
            assert!(remainder(number, denominator).abs_sub(1.7) <= 0.0000001);
        }
    }
    
    #[test]
    fn test_remainderf() {
        let number: c_float = 5.3;
        let denominator: c_float = 2.0;
        
        unsafe {
            assert!(remainderf(number, denominator).abs_sub(0.7) <= 0.0000001);
        }
    }
    
}
