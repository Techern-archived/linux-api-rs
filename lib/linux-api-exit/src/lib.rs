//! Bindings to kernel/exit.c
//!
//! Currently only provides access to exit() and exit_group()

#![cfg(target_os = "linux")]

extern crate linux_api;

use linux_api::{c_int};

extern {

    ///Exits the current process with the specified error code
    ///
    ///You should use either linux_api::EXIT_SUCCESS or linux_api::EXIT_FAILURE
    pub fn exit(error_code: c_int);

    ///Exits every process in the current process group with the specified error code
    ///
    ///You should use either linux_api::EXIT_SUCCESS or linux_api::EXIT_FAILURE
    pub fn exit_group(error_code: c_int);

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