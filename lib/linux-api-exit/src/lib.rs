//! Bindings to kernel/exit.c
//!
//! Currently only provides access to exit() and exit_group()

#![cfg(target_os = "linux")]

extern crate linux_api;

use linux_api::{c_int};

extern {

    ///Exits the current process with the specified exit code
    ///
    ///You should use either linux_api::EXIT_SUCCESS or linux_api::EXIT_FAILURE
    pub fn exit(exit_code: c_int);

    //pub fn exit_group(exit_code: c_int); I was advised to remove this since it should never be called in user space, but handled by libc while using exit()

}

///Exits the current process while indicating a successful operation
pub fn exit_as_success() {
    unsafe { exit(linux_api::EXIT_SUCCESS); }
}

///Exits the current process while indicating a failed operation as the cause of exit
pub fn exit_as_failure() {
    unsafe { exit(linux_api::EXIT_FAILURE); }
}

#[cfg(test)]
mod test {

    use linux_api::{EXIT_SUCCESS, EXIT_FAILURE};

    #[test]
    fn ensure_exit_values_exist() {
        assert_eq!(0, EXIT_SUCCESS);
        assert_eq!(1, EXIT_FAILURE);
    }

}