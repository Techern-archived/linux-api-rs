//! Defines types found in various posix_types.h files.
//! 
//! If unsure, check include/uapi/asm-generic/posix_types.h **first**, then /arch/*/include/uapi/asm/posix_types.h
//!
//! For the best result, use http://lxr.free-electrons.com



#[cfg(any(target_arch="sparc", target_arch="parisc"))]
///A type that defines microseconds. Since you're using either SPARC or (pa)RISC, it is a c_int
pub type suseconds_t = ::c_int;

#[cfg(not(any(target_arch="sparc", target_arch="parisc")))]
///A type that defines microseconds. You're using a "regular" processor, so it is a c_long
pub type suseconds_t = ::c_long;

///A type that defines a clock ID. As of 4.1, this is always c_int
pub type clockid_t = ::c_int;

///A type that defines a Process or Thread Group ID
pub type pid_t = ::c_int;

///A type that defines a user ID
pub type uid_t = ::c_uint;

///A type that defines a group ID
pub type gid_t = ::c_uint;

#[cfg(target_arch="x86")]
///A kernel type for long, set to c_longlong snice you're on x86
pub type __kernel_long_t = ::c_longlong;

#[cfg(not(target_arch="x86"))]
///A kernel type for long, set to c_long since you're not on x86
pub type __kernel_long_t = ::c_long;

#[cfg(target_arch="x86")]
///A kernel type for ulong, set to c_ulonglong snice you're on x86
pub type __kernel_ulong_t = ::c_ulonglong;

#[cfg(not(target_arch="x86"))]
///A kernel type for ulong, set to c_ulong since you're not on x86
pub type __kernel_ulong_t = ::c_ulong;