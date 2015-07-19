//!Bindings and rewrites of include/linux/time.h and include/linux/time64.h

extern crate linux_api;

use linux_api::*;

#[link(name="rt")]
extern "C" {
    //Dummy section to link RT, but probably shouldn't link everything to it
}

//First, we do external functions
extern "C" {
    
    ///Gets the current time as seconds from the UNIX epoch
    pub fn time(t: *mut time_t) -> time_t;
    
    ///Calculates the difference, in seconds, between two values of time_t
    pub fn difftime(time1: time_t, time0: time_t) -> c_double;
    
    ///Gets the time of day in the specified timezone and place it into the timeval
    ///
    ///Please use std::ptr::null_mut() for tz on Linux!
    pub fn gettimeofday(tv: *mut timeval, tz: *mut timezone) -> c_int;
    
    ///Sets the time of day from the timeval and applies rules from the timezone to it
    ///
    ///Please use std::ptr::null() for tz on Linux!
    pub fn settimeofday(tv: *const timeval, tz: *const timezone) -> c_int;
    
    ///Finds the resolution of the specified clock, and if non-null, stores it in the timespec.
    pub fn clock_getres(clk_id: clockid_t, res: *mut timespec) -> c_int;
    
    ///Gets the time according to the specified clock ID and places it into the timespec pointer
    pub fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int;
    
    ///Takes the time from the specified timespec and attempts to apply it to the clock based on ID
    pub fn clock_settime(clk_id: clockid_t, tp: *const timespec) -> c_int;
    
}

///Checks to see if two timespecs are equal
pub fn timespec_equal(a: &timespec, b: &timespec) -> c_int {
    return ((a.tv_sec == b.tv_sec) && (a.tv_nsec == b.tv_nsec)) as c_int;
}

///Compares two timespecs
///
///If lhs < rhs, return < 0  
///If lhs == rhs, return 0  
///If lhs > rhs, return > 0
pub fn timespec_compare(lhs: &timespec, rhs: &timespec) -> c_int {
    if lhs.tv_sec < rhs.tv_sec { return -1; }
    if lhs.tv_sec > rhs.tv_sec { return 1; }
    return (lhs.tv_nsec - rhs.tv_nsec) as c_int;
}

///Compares two timevals
///
///If lhs < rhs, return < 0  
///If lhs == rhs, return 0  
///If lhs > rhs, return > 0
pub fn timeval_compare(lhs: &timeval, rhs: &timeval) -> c_int {
    if lhs.tv_sec < rhs.tv_sec { return -1; }
    if lhs.tv_sec > rhs.tv_sec { return 1; }
    return (lhs.tv_usec - rhs.tv_usec) as c_int;
}

//TODO: mktime?

///Sets the data contained in the borrowed `timespec` ts to normalized values
pub fn set_normalized_timespec(ts: &mut timespec, sec: time_t, nsec: i64) {
    let mut realsec = sec;
    let mut realnsec = nsec;
    
    while realnsec >= NSEC_PER_SEC {
        //TODO: Check to see if this gets optimized and, if so, force it not to as per kernel definition
        realnsec -= NSEC_PER_SEC;
        realsec += 1;
    }
    
    while realnsec < 0 {
        //Ditto
        realnsec += NSEC_PER_SEC;
        realsec -= 1;
    }
    
    ts.tv_sec = realsec;
    ts.tv_nsec = realnsec;
}

//TODO: timespec_add_safe

///Adds two timespecs together, returning the result as a new timespec
pub fn timespec_add(lhs: &timespec, rhs: &timespec) -> timespec {
    let mut ts_delta: timespec = timespec::new();
    set_normalized_timespec(&mut ts_delta, lhs.tv_sec + rhs.tv_sec, lhs.tv_nsec + rhs.tv_nsec);
    ts_delta
}

///Subtracts rhs from lhs, returning the difference as a new timespec
pub fn timespec_sub(lhs: &timespec, rhs: &timespec) -> timespec {
    let mut ts_delta: timespec = timespec::new();
    set_normalized_timespec(&mut ts_delta, lhs.tv_sec - rhs.tv_sec, lhs.tv_nsec - rhs.tv_nsec);
    ts_delta
}

///Checks to see if a timespec is valid
pub fn timespec_valid(ts: &timespec) -> bool {
    //Dates before 1970 are bogus
    if ts.tv_sec < 0 {
        return false;
    }
    
    //Can't have more nanoseconds than a second
    if ts.tv_nsec >= NSEC_PER_SEC {
        return false;
    }
    
    //Okay, all looks good
    return true;
}

///Checks to see if a timespec is valid according to stricter rules
pub fn timespec_valid_strict(ts: &timespec) -> bool {
    if !timespec_valid(ts) { 
        return false; 
    }
    
    //Okay, we've validated it quickly
    //Disallow values that could overflow ktime_t
    if ts.tv_sec >= KTIME_SEC_MAX { 
        return false;
    }
    
    return true;
}

///Checks to see if a timeval is valid
pub fn timeval_valid(tv: &timeval) -> bool {
    //Dates before 1970 are bogus
    if tv.tv_sec < 0 {
        return false;
    }
    
    //Can't have more microseconds than a second
    if tv.tv_usec >= USEC_PER_SEC {
        return false;
    }
    
    //Okay, all looks good
    return true;
}

///Creates a time64_t from the supplied year, month, day, hour, minute, and second
pub fn mktime64(year0: c_uint, mon0: c_uint, day: c_uint, hour: c_uint, min: c_uint, sec: c_uint) -> time64_t {
    
    let mut mon = mon0;
    let mut year = year0;
    
    //The kernel tells us to do this. It's basically treating Jan and Feb as months 11 and 12, then pulling the others up by two. Leap year, perhaps?
    
    let mut _mon = mon as c_int;
    
    _mon -= 2;
    
    if _mon <= 0 {
        mon += 10; //Because we didn't take the two off)
        year -= 1;
    } else {
        mon -= 2; //Because we can
    }
    
    //I'm trusting you here, kernel developers! This is ugly but it had better work
    
    let mut result: time64_t = 0;
    
    result = (year/4 - year/100 + year/400 + 367*mon/12 + day) as i64;
    result += year as i64 * 365;
    result -= 719499;
    result *= 24;
    result += hour as i64;
    result *= 60;
    result += min as i64;
    result *= 60;
    result += sec as i64;
    
    //result += (((((year / 4) - (year / 100) + (year / 400) + (367*mon/12) + day + (year * 365)) * 24 + hour) * 60 + min) * 60 + sec) as time64_t;
    
    return result;
    
}

#[cfg(test)]
mod test {
    use super::*;
    use linux_api::*;
    
    #[test]
    fn test_clock_times() {
        let mut spec: timespec = timespec::new();
        
        let raw_spec = &mut spec as *mut timespec;
        
        unsafe { clock_gettime(CLOCK_REALTIME, raw_spec); }
        
        assert!(spec.tv_sec >= 1437311973); //Time of testing
        
        unsafe { clock_gettime(CLOCK_TAI, raw_spec); } //Ooooh, atoms *Waves hands around making scary noises*
        
        assert!(spec.tv_sec >= 1437312050);
    }
    
    #[test]
    fn test_clock_res() {
        let mut spec: timespec = timespec::new();
        
        let raw_spec = &mut spec as *mut timespec;
        
        unsafe { clock_getres(CLOCK_REALTIME, raw_spec); }
        
        assert_eq!(1, spec.tv_nsec);
    }
    
    #[test]
    fn test_time_ffi() {
        let mut atime: time_t = 0;
        
        unsafe { atime = time(::std::ptr::null_mut()); }
        
        assert!(atime >= 1437309103); //Time of first testing
    }
    
    #[test]
    fn test_difftime_ffi() {
        let mut atime: time_t = 0;
        
        unsafe { atime = time(::std::ptr::null_mut()); }
        
        let anothertime: time_t = atime - 100;
        
        let mut result: c_double = 0f64;
        
        unsafe { result = difftime(atime, anothertime); }
        
        assert_eq!(100f64, result);
        
        unsafe { result = difftime(anothertime, atime); }
        
        assert_eq!(-100f64, result);
    }
    
    #[test]
    fn test_gettimeofday_ffi() {
        let mut time = timeval::new();
        
        let raw_time = &mut time as *mut timeval;
        
        unsafe {
            let result: c_int = gettimeofday(raw_time, ::std::ptr::null_mut());
            assert_eq!(0, result);
        }
        
        assert!(!raw_time.is_null());
        
        assert!(time.tv_sec >= 1437310400); //At time of testing
        
    }
    
    #[test]
    fn test_timespec_valid() {
        let atime: timespec = timespec { tv_sec: 0, tv_nsec: 4262 };
        
        assert!(timespec_valid(&atime));
    }
    
    #[test]
    fn test_timespec_too_many_seconds_valid() {
        let atime: timespec = timespec::from_seconds(KTIME_SEC_MAX + 365);
        
        assert!(!timespec_valid_strict(&atime));
    }
    
    #[test]
    fn test_timespec_before_1970_invalid() {
        let atime: timespec = timespec::from_seconds(-63);
        
        assert!(!timespec_valid(&atime));
    }
    
    #[test]
    fn test_timespec_too_many_nanoseconds_invalid() {
        let atime: timespec = timespec { tv_sec: 0, tv_nsec: NSEC_PER_SEC + 2 };
        
        assert!(!timespec_valid(&atime));
    }
    
    #[test]
    fn test_timeval_valid() {
        let atime: timeval = timeval { tv_sec: 0, tv_usec: 4262 };
        
        assert!(timeval_valid(&atime));
    }
    
    #[test]
    fn test_timeval_before_1970_invalid() {
        let atime: timeval = timeval::from_seconds(-42562);
        
        assert!(!timeval_valid(&atime));
    }
    
    #[test]
    fn test_timeval_too_many_nanoseconds_invalid() {
        let atime: timeval = timeval { tv_sec: 0, tv_usec: USEC_PER_SEC + 2 };
        
        assert!(!timeval_valid(&atime));
    }

    #[test]
    fn test_timespec_equal_when_equal() {
        let one: timespec = timespec { tv_sec: 77, tv_nsec: 77 };
        let two: timespec = timespec { tv_sec: 77, tv_nsec: 77 };
        
        assert_eq!(1, timespec_equal(&one, &two));
    }

    #[test]
    fn test_timespec_equal_when_inequal() {
        let one: timespec = timespec { tv_sec: 0, tv_nsec: 266 };
        let two: timespec = timespec { tv_sec: 4, tv_nsec: 0 };
        
        assert_eq!(0, timespec_equal(&one, &two));
    }
    
    #[test]
    fn test_timespec_compare_same() {
        let one = timespec::from_seconds(10);
        let two = timespec::from_seconds(10);
        
        assert_eq!(0, timespec_compare(&one, &two));
    }
    
    #[test]
    fn test_timespec_compare_larger() {
        let one = timespec::from_seconds(11);
        let two = timespec::from_seconds(10);
        
        assert_eq!(1, timespec_compare(&one, &two));
    }
    
    #[test]
    fn test_timespec_compare_smaller() {
        let one = timespec::from_seconds(9);
        let two = timespec::from_seconds(10);
        
        assert_eq!(-1, timespec_compare(&one, &two));
    }
    
    #[test]
    fn test_timeval_compare_same() {
        let one = timeval::from_seconds(10);
        let two = timeval::from_seconds(10);
        
        assert_eq!(0, timeval_compare(&one, &two));
    }
    
    #[test]
    fn test_timeval_compare_larger() {
        let one = timeval::from_seconds(11);
        let two = timeval::from_seconds(10);
        
        assert_eq!(1, timeval_compare(&one, &two));
    }
    
    #[test]
    fn test_timeval_compare_smaller() {
        let one = timeval::from_seconds(9);
        let two = timeval::from_seconds(10);
        
        assert_eq!(-1, timeval_compare(&one, &two));
    }
    
    #[test]
    fn test_normalized_timespec_many_nanos() {
        let mut spec: timespec = timespec::new();
        
        set_normalized_timespec(&mut spec, 10, NSEC_PER_SEC * 20);
        
        assert_eq!(30, spec.tv_sec);
        assert_eq!(0, spec.tv_nsec);
    }
    
    #[test]
    fn test_normalized_timespec_negative_nanos() {
        let mut spec: timespec = timespec::new();
        
        set_normalized_timespec(&mut spec, 5, -800);
        
        assert_eq!(4, spec.tv_sec);
        assert_eq!(NSEC_PER_SEC - 800, spec.tv_nsec);
    }
    
    #[test]
    fn test_timespec_add_simple() {
        let lhs = timespec { tv_sec: 80, tv_nsec: 500 };
        let rhs = timespec { tv_sec: 30, tv_nsec: 580 };
        
        let delta = timespec_add(&lhs, &rhs);
        
        assert_eq!(110, delta.tv_sec);
        assert_eq!(1080, delta.tv_nsec);
    }
    
    #[test]
    fn test_timespec_add_manynanos() {
        let lhs = timespec { tv_sec: 80, tv_nsec: NSEC_PER_SEC / 2 };
        let rhs = timespec { tv_sec: 30, tv_nsec: (NSEC_PER_SEC / 2) + 400 };
        
        let delta = timespec_add(&lhs, &rhs);
        
        assert_eq!(111, delta.tv_sec);
        assert_eq!(400, delta.tv_nsec);
    }
    
    #[test]
    fn test_timespec_sub_simple() {
        let lhs = timespec { tv_sec: 80, tv_nsec: 500 };
        let rhs = timespec { tv_sec: 30, tv_nsec: 580 };
        
        let delta = timespec_sub(&lhs, &rhs);
        
        assert_eq!(49, delta.tv_sec);
        assert_eq!(NSEC_PER_SEC - 80, delta.tv_nsec);
    }
    
    #[test]
    fn test_timespec_sub_bad() {
        let lhs = timespec { tv_sec: 80, tv_nsec: 500 };
        let rhs = timespec { tv_sec: 1030, tv_nsec: 580 };
        
        let delta = timespec_sub(&lhs, &rhs);
        
        assert_eq!(-951, delta.tv_sec);
        assert_eq!(NSEC_PER_SEC - 80, delta.tv_nsec);
    }
    
    #[test]
    fn test_mktime64_known_dates() {
        let apocalypse2012: time64_t = 1356048000;
        let sequential: time64_t = 1234567890;
        
        assert_eq!(apocalypse2012, mktime64(2012, 12, 21, 00, 0, 0));
        assert_eq!(sequential, mktime64(2009, 2, 13, 23, 31, 30));
    }

}