//! Bindings to include/uapi/linux/ipc.h

pub const IPC_PRIVATE: ::__kernel_key_t = 0;

///This structure is obsolute, used only for backwards compatibility and libc5 compilation
pub struct ipc_perm {

    pub key: ::__kernel_key_t,
    
    pub uid: ::__kernel_uid_t,
    
    pub gid: ::__kernel_gid_t,
    
    pub cuid: ::__kernel_uid_t,
    
    pub cgid: ::__kernel_gid_t,
    
    pub mode: ::__kernel_mode_t,
    
    pub seq: ::c_ushort

}

///Create if the key is nonexistent
pub const IPC_CREAT: ::key_t = 00001000;

///Fail if the key exists
pub const IPC_EXCL: ::key_t = 00002000;

///Return an error on wait
pub const IPC_NOWAIT: ::key_t = 00004000;

///"Make it distributed"
pub const IPC_DIPC: ::key_t = 00010000;

///This machine is the DIPC owner
pub const IPC_OWN: ::key_t = 00020000;



///Remove resource
pub const IPC_RMID: ::c_int = 0;

///Set ipc_perm options
pub const IPC_SET: ::c_int = 1;

///Get ipc_perm options
pub const IPC_GET: ::c_int = 2;

///See ipcs
pub const IPC_INFO: ::c_int = 3;



///Old version (No 32-bit UID support on many architectures)
pub const IPC_OLD: ::c_int = 0;

///New version (support for 32-bit UIDs, bigger message sizes, etc)
pub const IPC_64: ::c_int = 0x0100;



pub const SEMOP: ::c_int = 1;

pub const SEMGET: ::c_int = 2;

pub const SEMCTL: ::c_int = 3;

pub const SEMTIMEDOP: ::c_int = 4;

pub const MSGSND: ::c_int = 11;

pub const MSGRCV: ::c_int = 12;

pub const MSGGET: ::c_int = 13;

pub const MSGCTL: ::c_int = 14;

pub const SHMAT: ::c_int = 21;

pub const SHMDT: ::c_int = 22;

pub const SHMGET: ::c_int = 23;

pub const SHMCTL: ::c_int = 24;

///Used by the DIPC package, please try to not use it
pub const DIPC: ::c_int = 25;

///Gets the IPCCALL number for a version and operation
///
///I should replace this with a macro at some stage
#[allow(non_snake_case)]
pub fn IPCCALL(version: ::c_int, op: ::c_int) -> ::c_int {

    (version << 16) | op

}