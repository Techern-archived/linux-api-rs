//! This is a very early work-in-progress binding to various Linux Kernel APIs.

#![cfg(target_os="linux")]

//Start allowing warnings that are unavoidable
#![allow(non_camel_case_types)]
#![allow(dead_code)]

//Okay, let's import our internal modules
pub mod cstdlib;
pub mod posix_types;
pub mod time;
pub mod unistd;

pub use std::os::*;
pub use std::os::raw::*;
pub use std::os::linux::raw::*; //Needed for time_t, dev_t, etc

//And re-export our modules
pub use cstdlib::*;
pub use posix_types::*;
pub use time::*;
pub use unistd::*;

#[cfg(test)]
mod tests {

    #[allow(unused_variables)]
    #[test]
    fn ensure_types_exist() {
        let schar: ::c_schar = -3;
        let uchar: ::c_uchar = 2;
        let achar: ::c_char = 62;
        
        let ashort: ::c_short = -5162;
        let ushort: ::c_ushort = 65000;
        
        let aint: ::c_int = 26327;
        let uint: ::c_uint = 20000026;
        
        let long: ::c_long = 75473765327;
        let ulong: ::c_ulong = 26294762868676748;
        
        let float: ::c_float = 2462347.426f32;
        let double: ::c_double = 2694237684327.4326237637f64;
        
        let dev: ::dev_t = 12467;
        let mode: ::mode_t = 1365236;
        let time: ::time_t = 134062;
        
        assert!(true);//Well, we haven't crashed! I guess it worked
    }

}
