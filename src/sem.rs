//! A rewrite of uapi/linux/sem.h

pub const SEM_UNDO: ::c_int = 0x1000; //TODO Check this to make sure it's a c_int

///Get sempid
pub const GETPID: ::c_int = 11;

///Get semval
pub const GETVAL: ::c_int = 12;

///Get all semvals
pub const GETALL: ::c_int = 13;

///Get semncnt
pub const GETNCNT: ::c_int = 14;

///Get semzcnt
pub const GETZCNT: ::c_int = 15;

///Set semval
pub const SETVAL: ::c_int = 16;

///Set all semvals
pub const SETALL: ::c_int = 17;

pub const SEM_STAT: ::c_int = 18;

pub const SEM_INFO: ::c_int = 19;

/*///Obsolete, used for backwards compatibility and libc5 compiles
pub struct semid_ds {
    
    pub sem_perm: ::ipc_perm,
    
    pub sem_otime: ::__kernel_time_t,
    
    pub sem_ctime: ::__kernel_time_t,
    
    pub sem_base: *mut ::sem,
    
    pub sem_pending: *mut ::sem_queue,
    
    pub sem_pending_last: *mut ::sem_queue,
    
    pub undo: *mut ::sem_undo,
    
    pub sem_nsems: ::c_ushort
    
} TODO: Complete enough to enable this */

///The semop system call takes an array of these
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct sembuf {

    ///The semaphore's index in the array
    pub sem_num: ::c_ushort,
    
    ///The semaphore operation
    pub sem_op: ::c_short,
    
    ///The operation's flags
    pub sem_flg: ::c_short

}

///The argument for semctl system calls
#[repr(C)]
pub enum semun {

    val(::c_int),
    //buf(*mut semid_ds),
    array(*mut ::c_ushort),
    __buf(*mut seminfo),
    __pad

}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct seminfo {

    pub semmap: ::c_int,
    
    pub semmni: ::c_int,
    
    pub semmns: ::c_int,
    
    pub semmnu: ::c_int,
    
    pub semmsl: ::c_int,
    
    pub semopm: ::c_int,
    
    pub semume: ::c_int,
    
    pub semusz: ::c_int,
    
    pub semvmx: ::c_int,
    
    pub semaem: ::c_int

}

/// <= IPCMNI: Max number of semaphore identifiers
pub const SEMMNI: ::c_int = 32000;

/// <= INT_MAX: Max number of semaphores per ID
pub const SEMMSL: ::c_int = 32000;

/// <= INT_MAX: Max nubmer of semaphores in the system
pub const SEMMNS: ::c_int = SEMMNI * SEMMSL;

/// <= 1000: Max number of operations per semop call
pub const SEMOPM: ::c_int = 500;

/// <= 32767: Semaphore maximum value
pub const SEMVMX: ::c_int = 32767;

/// Adjust on exit max value
pub const SEMAEM: ::c_int = SEMVMX;

///Max number of undo entries per process
pub const SEMUME: ::c_int = SEMOPM;

///Number of undo structures system wide
pub const SEMMNU: ::c_int = SEMMNS;

///Number of entries in the semaphore map
pub const SEMMAP: ::c_int = SEMMNS;

///Sizeof struct sem_undo
pub const SEMUSZ: ::c_int = 20;