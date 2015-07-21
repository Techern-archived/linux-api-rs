#![cfg(target_os="linux")]
//! Bindings to kernel/sys.c

//First, let's use linux-api stuff
extern crate linux_api;
use linux_api::{pid_t, uid_t};

//Okay, let's do this!
extern "C" {

    ///Returns the thread group ID of the current process
    pub fn getpid() -> pid_t;
    
    ///Returns the parent process's thread group ID
    pub fn getppid() -> pid_t;
    
    ///Returns the current user's ID
    pub fn getuid() -> uid_t;
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

}