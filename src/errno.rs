//! Bindings to various errno.h definitions.

///Error numbers defined in errno-base.h, with some exceptions which are marked accordingly
pub mod errno_base {

    ///Operation not permitted
    pub const EPERM: ::c_int = 1;
    
    ///No such file or directory
    pub const ENOENT: ::c_int = 2;
    
    ///No such process
    pub const ESRCH: ::c_int = 3;
    
    ///Interrupted system call. Oops
    pub const EINTR: ::c_int = 4;
    
    ///I/O error.
    pub const EIO: ::c_int = 5;
    
    ///No such device or address
    pub const ENXIO: ::c_int = 6;
    
    ///Argument list too long.
    ///
    ///You thought it was error too big, didn't you?
    pub const E2BIG: ::c_int = 7;
    
    ///Exec format error
    pub const ENOEXEC: ::c_int = 8;
    
    ///Bad file number
    pub const EBADF: ::c_int = 9;
    
    ///No child processes
    ///
    ///Alternatively, your name is Anthony Dinozzo Jr or Richard Castle
    pub const ECHILD: ::c_int = 10;
    
    ///Try again, I'm not really an error, just an annoyance
    pub const EAGAIN: ::c_int = 11;
    
    ///Out of memory
    pub const ENOMEM: ::c_int = 12;
    
    ///Permission denied!
    pub const EACCES: ::c_int = 13;
    
    ///Bad address
    pub const EFAULT: ::c_int = 14;
    
    ///Block device required but not found
    pub const ENOTBLK: ::c_int = 15;
    
    ///Device or resource is too busy to listen to you right now
    pub const EBUSY: ::c_int = 16;
    
    ///File already exists
    pub const EEXIST: ::c_int = 17;
    
    ///Cross-device link. Sorry, but quantum entanglement is not working in storage drives yet
    pub const EXDEV: ::c_int = 18;
    
    ///No such device.
    ///
    ///Alternatively, you have been robbed by a Jedi knight. "This is not the device you were searching for"
    pub const ENODEV: ::c_int = 19;
    
    ///That's... not a directory
    pub const ENOTDIR: ::c_int = 20;
    
    ///That's... not a file. That's a directory
    pub const EISDIR: ::c_int = 21;
    
    ///Your argument is invalid!
    pub const EINVAL: ::c_int = 22;
    
    ///File table overflowed
    pub const ENFILE: ::c_int = 23;
    
    ///Too many files are already open
    pub const EMFILE: ::c_int = 24;
    
    ///You are not a typewriter
    ///
    ///Does this expand to you are not a terminal? What a about a smart kettle? Or a WIFI-enabled microwave? Hey, somebody should go patent that
    pub const ENOTTTY: ::c_int = 25;
    
    ///Text file is too busy to listen to you right now
    pub const ETXTBSY: ::c_int = 26; //Ooooh, what are you writing?
    
    ///File is too large. What are you doing, downloading a car?
    pub const EFBIG: ::c_int = 27; //Okay, really, how?
    
    ///You don't have any (or at least enough) space left on the specified device. You might need a new hard drive.
    ///
    ///Might I suggest a SSD? They cost more but the performance gain is worth it
    pub const ENOSPC: ::c_int = 28; //I'm so sorry
    
    ///Illegal seek
    ///
    ///What the hell are you doing?
    pub const ESPIPE: ::c_int = 29;
    
    ///You're trying to write to a read-only file system.
    pub const EROFS: ::c_int = 30; //Haha, nice try
    
    ///Too many links exist
    pub const EMLINK: ::c_int = 31;
    
    ///Broken pipe
    pub const EPIPE: ::c_int = 32;
    
    ///Math argument is out of the function's domain
    ///
    ///??? I need more coffee to get this. Is it like trying to check if an int is NaN?
    pub const EDOM: ::c_int = 33;
    
    ///Math result could not be represented
    ///
    ///Stop trying to divide by ze--- ```EACCES```
    pub const ERANGE: ::c_int = 34;

}

pub use self::errno_base::*;