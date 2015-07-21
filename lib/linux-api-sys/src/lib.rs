#![cfg(target_os="linux")]
//! Bindings to kernel/sys.c

//First, let's use linux-api stuff
extern crate linux_api;
use linux_api::{pid_t, uid_t, gid_t};

//Okay, let's do this!
extern "C" {

    ///Returns the thread group ID of the current process
    pub fn getpid() -> pid_t;
    
    ///Returns the parent process's thread group ID
    pub fn getppid() -> pid_t;
    
    ///Returns the current user's ID
    pub fn getuid() -> uid_t;
    
    ///Returns the current effective user's ID
    pub fn geteuid() -> uid_t;
    
    ///Returns the current user's (primary?) group ID
    pub fn getgid() -> gid_t;
    
    ///Returns the current effective user's (primary?) group ID
    pub fn getegid() -> gid_t;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn ensure_not_on_process_zero() {
        unsafe { 
            assert!(getpid() >= 0); 
            assert!(getppid() >= 0); 
        }
    }
    
    #[test]
    fn ensure_group_same() {
        unsafe {
            assert_eq!(getgid(), getegid());
        }
    }
    
    #[test]
    fn ensure_user_same() {
        unsafe {
            assert_eq!(getuid(), geteuid());
        }
    }

}