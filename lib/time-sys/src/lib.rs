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

#[cfg(test)]
mod test {
    use super::*;
    use linux_api::*;

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

}