//!Bindings and rewrites of include/linux/time.h and include/linux/time64.h

extern crate linux_api;

use linux_api::*;

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

//TODO: mktime64
//TODO: mktime
//TODO: set_normalized_timespec

//TODO: timespec_add_safe, timespec_add, timespec_sub

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

#[cfg(test)]
mod test {
    use super::*;
    use linux_api::*;
    
    #[test]
    fn test_timespec_valid() {
        let atime: timespec = timespec { tv_sec: 0, tv_nsec: 4262 };
        
        assert!(timespec_valid(&atime));
    }
    
    #[test]
    fn test_timespec_too_many_seconds_valid() {
        let atime: timespec = timespec { tv_sec: KTIME_SEC_MAX + 365, tv_nsec: 4262 };
        
        assert!(!timespec_valid_strict(&atime));
    }
    
    #[test]
    fn test_timespec_before_1970_invalid() {
        let atime: timespec = timespec { tv_sec: -63, tv_nsec: 4262 };
        
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
        let atime: timeval = timeval { tv_sec: -63, tv_usec: 4262 };
        
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
        let one = timespec { tv_sec: 10, tv_nsec: 0 };
        let two = timespec { tv_sec: 10, tv_nsec: 0 };
        
        assert_eq!(0, timespec_compare(&one, &two));
    }
    
    #[test]
    fn test_timespec_compare_larger() {
        let one = timespec { tv_sec: 11, tv_nsec: 0 };
        let two = timespec { tv_sec: 10, tv_nsec: 0 };
        
        assert_eq!(1, timespec_compare(&one, &two));
    }
    
    #[test]
    fn test_timespec_compare_smaller() {
        let one = timespec { tv_sec: 9, tv_nsec: 0 };
        let two = timespec { tv_sec: 10, tv_nsec: 0 };
        
        assert_eq!(-1, timespec_compare(&one, &two));
    }
    
    #[test]
    fn test_timeval_compare_same() {
        let one = timeval { tv_sec: 10, tv_usec: 0 };
        let two = timeval { tv_sec: 10, tv_usec: 0 };
        
        assert_eq!(0, timeval_compare(&one, &two));
    }
    
    #[test]
    fn test_timeval_compare_larger() {
        let one = timeval { tv_sec: 11, tv_usec: 0 };
        let two = timeval { tv_sec: 10, tv_usec: 0 };
        
        assert_eq!(1, timeval_compare(&one, &two));
    }
    
    #[test]
    fn test_timeval_compare_smaller() {
        let one = timeval { tv_sec: 9, tv_usec: 0 };
        let two = timeval { tv_sec: 10, tv_usec: 0 };
        
        assert_eq!(-1, timeval_compare(&one, &two));
    }

}