//! Bindings to various unistd.h files, including /uapi/asm-generic/unistd.h as a default where no customized version was found

#[cfg(not(any(target_arch = "alpha",
              target_arch = "arm",
              target_arch = "avr32",
              target_arch = "blackfin",
              target_arch = "cris",
              target_arch = "frv",
              target_arch = "ia64",
              target_arch = "m32r",
              target_arch = "m68k",
              target_arch = "microblaze",
              target_arch = "mips",
              target_arch = "mn10300",
              target_arch = "parisc",
              target_arch = "powerpc",
              target_arch = "s390",
              target_arch = "sh",
              target_arch = "sparc")))]
pub mod common_unistd {
    pub const __NR_IO_SETUP: ::c_long = 0;
    pub const __NR_IO_DESTROY: ::c_long = 1;
    pub const __NR_IO_SUBMIT: ::c_long = 2;
    pub const __NR_IO_CANCEL: ::c_long = 3;
    pub const __NR_IO_GETEVENTS: ::c_long = 4;
    
    pub const __NR_SETXADDR: ::c_long = 5;
    pub const __NR_LSETXADDR: ::c_long = 6;
    pub const __NR_FSETXADDR: ::c_long = 7;
    pub const __NR_GETXADDR: ::c_long = 8;
    pub const __NR_LGETXADDR: ::c_long = 9;
    pub const __NR_FGETXADDR: ::c_long = 10;
    pub const __NR_LISTXADDR: ::c_long = 11;
    pub const __NR_LLISTXADDR: ::c_long = 12;
    pub const __NR_FLISTXADDR: ::c_long = 13;
    pub const __NR_REMOVEXADDR: ::c_long = 14;
    pub const __NR_LREMOVEXADDR: ::c_long = 15;
    pub const __NR_FREMOVEXADDR: ::c_long = 16;
    
    pub const __NR_GETCWD: ::c_long = 17;
    
    pub const __NR_LOOKUP_DCOOKIE: ::c_long = 18;
    
    pub const __NR_EVENTFD2: ::c_long = 19;
    
    pub const __NR_EPOLL_CREATE1: ::c_long = 20;
    pub const __NR_EPOLL_CTL: ::c_long = 21;
    pub const __NR_EPOLL_PWAIT: ::c_long = 22;
    
    pub const __NR_DUP: ::c_long = 23;
    pub const __NR_DUP3: ::c_long = 24;
    pub const __NR3264_FCNTL: ::c_long = 25; //Whoa, was somebody high when they defined this?
    
    pub const __NR_INOTIFY_INIT1: ::c_long = 26;
    pub const __NR_INOTIFY_ADD_WATCH: ::c_long = 27;
    pub const __NR_INOTIFY_RM_WATCH: ::c_long = 28;
    
    pub const __NR_IOCTL: ::c_long = 29; //We'll need this later :)
    
    pub const __NR_IOPRIO_SET: ::c_long = 30;
    pub const __NR_IOPRIO_GET: ::c_long = 31;
    
    pub const __NR_FLOCK: ::c_long = 32;
    
    pub const __NR_MKNODAT: ::c_long = 33;
    pub const __NR_MKDIRAT: ::c_long = 34;
    pub const __NR_UNLINKAT: ::c_long = 35;
    pub const __NR_SYMLINKAT: ::c_long = 36;
    pub const __NR_LINKAT: ::c_long = 37;
    pub const __NR_RENAMEAT: ::c_long = 38;
    
    pub const __NR_UMOUNT2: ::c_long = 39;
    pub const __NR_MOUNT: ::c_long = 40;
    pub const __NR_PIVOT_ROOT: ::c_long = 41;
    
    pub const __NR_NFSSERVCTL: ::c_long = 42;
    
    pub const __NR3264_STATFS: ::c_long = 43;
    pub const __NR3264_FSTATFS: ::c_long = 44;
    pub const __NR3264_TRUNCATE: ::c_long = 45;
    pub const __NR3264_FTRUNCATE: ::c_long = 46;
    
    pub const __NR_FALLOCATE: ::c_long = 47;
    pub const __NR_FACCESSAT: ::c_long = 48;
    pub const __NR_CHDIR: ::c_long = 49;
    pub const __NR_FCHDIR: ::c_long = 50;
    pub const __NR_CHROOT: ::c_long = 51;
    pub const __NR_FCHMOD: ::c_long = 52;
    pub const __NR_FCHMODAT: ::c_long = 53;
    pub const __NR_FCHOWNAT: ::c_long = 54;
    pub const __NR_FCHOWN: ::c_long = 55;
    pub const __NR_OPENAT: ::c_long = 56;
    pub const __NR_CLOSE: ::c_long = 57;
    pub const __NR_VHANGUP: ::c_long = 58;
    
    pub const __NR_PIPE2: ::c_long = 59;
    
    pub const __NR_QUOTACTL: ::c_long = 60;
    
    pub const __NR_GETDENTS64: ::c_long = 61;
    
    pub const __NR3264_LSEEK: ::c_long = 62;
    pub const __NR_READ: ::c_long = 63;
    pub const __NR_WRITE: ::c_long = 64;
    pub const __NR_READV: ::c_long = 65;
    pub const __NR_WRITEV: ::c_long = 66;
    pub const __NR_PREAD64: ::c_long = 67;
    pub const __NR_PWRITE64: ::c_long = 68;
    pub const __NR_PREADV: ::c_long = 69;
    pub const __NR_PWRITEV: ::c_long = 70; //I wonder if I could generate music based on line lengths in various .rs files... I need to look into that one day. This would make a good chord if I did it right
    
    pub const __NR3264_SENDFILE: ::c_long = 71;
    
    pub const __NR_PSELECT6: ::c_long = 72;
    pub const __NR_PPOLL: ::c_long = 73;
    
    pub const __NR_SIGNALFD4: ::c_long = 74;
    
    pub const __NR_VMSPLICE: ::c_long = 75;
    pub const __NR_SPLICE: ::c_long = 76;
    pub const __NR_TEE: ::c_long = 77; //Fore!
    
    pub const __NR_READLINKAT: ::c_long = 78;
    pub const __NR3264_FSTATAT: ::c_long = 79;
    pub const __NR3264_FSTAT: ::c_long = 80;
    
    pub const __NR_SYNC: ::c_long = 81;
    pub const __NR_FSYNC: ::c_long = 82;
    pub const __NR_FDATASYNC: ::c_long = 83;
    pub const __NR_SYNC_FILE_RANGE: ::c_long = 84;
    pub const __NR_SYNC_FILE_RANGE2: ::c_long = __NR_SYNC_FILE_RANGE; //#ifdef __ARCH_WANT_SYNC_FILE_RANGE2 - WELL, THEN. We'll do this in the implementation somehow
    
    pub const __NR_TIMERFD_CREATE: ::c_long = 85;
    pub const __NR_TIMERFD_SETTIME: ::c_long = 86;
    pub const __NR_TIMERFD_GETTIME: ::c_long = 87;
    
    pub const __NR_UTIMENSAT: ::c_long = 88;
    
    pub const __NR_ACCT: ::c_long = 89;
    
    pub const __NR_CAPGET: ::c_long = 90;
    pub const __NR_CAPSET: ::c_long = 91;
    
    pub const __NR_PERSONALITY: ::c_long = 92; //Ooh, I need to look at this. I never knew we could have a personality
    
    pub const __NR_EXIT: ::c_long = 93;
    pub const __NR_EXIT_GROUP: ::c_long = 94;
    pub const __NR_WAITID: ::c_long = 95;
    
    pub const __NR_SET_TID_ADDRESS: ::c_long = 96;
    pub const __NR_UNSHARE: ::c_long = 97;
    
    pub const __NR_FUTEX: ::c_long = 98;
    pub const __NR_SET_ROBUST_LIST: ::c_long = 99;
    pub const __NR_GET_ROBUST_LIST: ::c_long = 100;
    
    pub const __NR_NANOSLEEP: ::c_long = 101;
    
    pub const __NR_GETITIMER: ::c_long = 102;
    pub const __NR_SETITIMER: ::c_long = 103;
    
    pub const __NR_KEXEC_LOAD: ::c_long = 104;
    
    pub const __NR_INIT_MODULE: ::c_long = 105;
    pub const __NR_DELETE_MODULE: ::c_long = 106;
    
    pub const __NR_TIMER_CREATE: ::c_long = 107;
    pub const __NR_TIMER_GETTIME: ::c_long = 108;
    pub const __NR_TIMER_GETOVERRUN: ::c_long = 109;
    pub const __NR_TIMER_SETTIME: ::c_long = 110;
    pub const __NR_TIMER_DELETE: ::c_long = 111;
    pub const __NR_CLOCK_SETTIME: ::c_long = 112;
    pub const __NR_CLOCK_GETTIME: ::c_long = 113;
    pub const __NR_CLOCK_GETRES: ::c_long = 114;
    pub const __NR_CLOCK_NANOSLEEP: ::c_long = 115;
    
    pub const __NR_SYSLOG: ::c_long = 116;
    
    pub const __NR_PTRACE: ::c_long = 117;
    
    pub const __NR_SCHED_SETPARAM: ::c_long = 118;
    pub const __NR_SCHED_SETSCHEDULER: ::c_long = 119;
    pub const __NR_SCHED_GETSCHEDULER: ::c_long = 120;
    pub const __NR_SCHED_GETPARAM: ::c_long = 121;
    pub const __NR_SCHED_SETAFFINITY: ::c_long = 122;
    pub const __NR_SCHED_GETAFFINITY: ::c_long = 123;
    pub const __NR_SCHED_YIELD: ::c_long = 124;
    pub const __NR_SCHED_GET_PRIORITY_MAX: ::c_long = 125;
    pub const __NR_SCHED_GET_PRIORITY_MIN: ::c_long = 126;
    pub const __NR_SCHED_RR_GET_INTERVAL: ::c_long = 127;
    
    pub const __NR_RESTART_SYSCALL: ::c_long = 128;
    pub const __NR_KILL: ::c_long = 129;
    pub const __NR_TKILL: ::c_long = 130;
    pub const __NR_TGKILL: ::c_long = 131;
    pub const __NR_SIGNALTSTACK: ::c_long = 132;
    pub const __NR_RT_SIGSUSPEND: ::c_long = 133;
    pub const __NR_RT_SIGACTION: ::c_long = 134;
    pub const __NR_RT_SIGPROCMASK: ::c_long = 135;
    pub const __NR_RT_SIGPENDING: ::c_long = 136;
    pub const __NR_RT_SIGTIMEDWAIT: ::c_long = 137;
    pub const __NR_RT_SIGQUEUEINFO: ::c_long = 138;
    pub const __NR_RT_SIGRETURN: ::c_long = 139;
    
    pub const __NR_SETPRIORITY: ::c_long = 140;
    pub const __NR_GETPRIORITY: ::c_long = 141;
    pub const __NR_REBOOT: ::c_long = 142;
    pub const __NR_SETREGID: ::c_long = 143;
    pub const __NR_SETGID: ::c_long = 144;
    pub const __NR_SETREUID: ::c_long = 145;
    pub const __NR_SETUID: ::c_long = 146;
    pub const __NR_SETRESUID: ::c_long = 147;
    pub const __NR_GETRESUID: ::c_long = 148;
    pub const __NR_SETRESGID: ::c_long = 149;
    pub const __NR_GETRESGID: ::c_long = 150;
    pub const __NR_SETFSUID: ::c_long = 151;
    pub const __NR_SETFSGID: ::c_long = 152;
    pub const __NR_TIMES: ::c_long = 153;
    pub const __NR_SETPGID: ::c_long = 154;
    pub const __NR_GETPGID: ::c_long = 155;
    pub const __NR_GETSID: ::c_long = 156;
    pub const __NR_SETSID: ::c_long = 157;
    pub const __NR_GETGROUPS: ::c_long = 158;
    pub const __NR_SETGROUPS: ::c_long = 159;
    pub const __NR_UNAME: ::c_long = 160;
    pub const __NR_SETHOSTNAME: ::c_long = 161;
    pub const __NR_SETDOMAINNAME: ::c_long = 162;
    pub const __NR_GETRLIMIT: ::c_long = 163;
    pub const __NR_SETRLIMIT: ::c_long = 164;
    pub const __NR_GETRUSAGE: ::c_long = 165;
    pub const __NR_UMASK: ::c_long = 166;
    pub const __NR_PRCTL: ::c_long = 167;
    pub const __NR_GETCPU: ::c_long = 168;
    
    pub const __NR_GETTIMEOFDAY: ::c_long = 169;
    pub const __NR_SETTIMEOFDAY: ::c_long = 170;
    pub const __NR_ADJTIMEX: ::c_long = 171;
    
    pub const __NR_GETPID: ::c_long = 172;
    pub const __NR_GETPPID: ::c_long = 173;
    pub const __NR_GETUID: ::c_long = 174;
    pub const __NR_GETEUID: ::c_long = 175;
    pub const __NR_GETGID: ::c_long = 176;
    pub const __NR_GETEGID: ::c_long = 177;
    pub const __NR_GETTID: ::c_long = 178;
    pub const __NR_SYSINFO: ::c_long = 179;
    
    pub const __NR_MQ_OPEN: ::c_long = 180;
    pub const __NR_MQ_UNLINK: ::c_long = 181;
    pub const __NR_MQ_TIMEDSEND: ::c_long = 182;
    pub const __NR_MQ_TIMEDRECEIVE: ::c_long = 183;
    pub const __NR_MQ_NOTIFY: ::c_long = 184;
    pub const __NR_MQ_GETSETATTR: ::c_long = 185;
    
    pub const __NR_MSGGET: ::c_long = 186;
    pub const __NR_MSGCTL: ::c_long = 187;
    pub const __NR_MSGRCV: ::c_long = 188;
    pub const __NR_MSGSND: ::c_long = 189;
    
    pub const __NR_SEMGET: ::c_long = 190;
    pub const __NR_SEMCTL: ::c_long = 191;
    pub const __NR_SEMTIMEDOP: ::c_long = 192;
    pub const __NR_SEMOP: ::c_long = 193;
    
    pub const __NR_SHMGET: ::c_long = 194;
    pub const __NR_SHMCTL: ::c_long = 195;
    pub const __NR_SHMAT: ::c_long = 196;
    pub const __NR_SHMDT: ::c_long = 197;
    
    pub const __NR_SOCKET: ::c_long = 198;
    pub const __NR_SOCKETPAIR: ::c_long = 199;
    pub const __NR_BIND: ::c_long = 200;
    pub const __NR_LISTEN: ::c_long = 201;
    pub const __NR_ACCEPT: ::c_long = 202;
    pub const __NR_CONNECT: ::c_long = 203;
    pub const __NR_GETSOCKNAME: ::c_long = 204;
    pub const __NR_GETPEERNAME: ::c_long = 205;
    pub const __NR_SENDTO: ::c_long = 206;
    pub const __NR_RECVFROM: ::c_long = 207;
    pub const __NR_SETSOCKOPT: ::c_long = 208;
    pub const __NR_GETSOCKOPT: ::c_long = 209;
    pub const __NR_SHUTDOWN: ::c_long = 210;
    pub const __NR_SENDMSG: ::c_long = 211;
    pub const __NR_RECVMSG: ::c_long = 212;
    
    pub const __NR_READAHEAD: ::c_long = 213;
    
    pub const __NR_BRK: ::c_long = 214;
    pub const __NR_MUNMAP: ::c_long = 215;
    pub const __NR_MREMAP: ::c_long = 216;
    
    pub const __NR_ADD_KEY: ::c_long = 217;
    pub const __NR_REQUEST_KEY: ::c_long = 218;
    pub const __NR_KEYCTL: ::c_long = 219;
    
    pub const __NR_CLONE: ::c_long = 220;
    pub const __NR_EXECVE: ::c_long = 221;
    
    pub const __NR3264_MMAP: ::c_long = 222;
    pub const __NR3264_FADVISE64: ::c_long = 223;
    
    //IFNDEF __ARCH_NOMMU - We will probably implement this in code
    
    pub const __NR_SWAPON: ::c_long = 224;
    pub const __NR_SWAPOFF: ::c_long = 225;
    pub const __NR_MPROTECT: ::c_long = 226;
    pub const __NR_MSYNC: ::c_long = 227;
    pub const __NR_MLOCK: ::c_long = 228;
    pub const __NR_MUNLOCK: ::c_long = 229;
    pub const __NR_MLOCKALL: ::c_long = 230;
    pub const __NR_MUNLOCKALL: ::c_long = 231;
    pub const __NR_MINCORE: ::c_long = 232;
    pub const __NR_MADVISE: ::c_long = 233;
    pub const __NR_REMAP_FILE_PAGES: ::c_long = 234;
    pub const __NR_MBIND: ::c_long = 235;
    pub const __NR_GET_MEMPOLICY: ::c_long = 236;
    pub const __NR_SET_MEMPOLICY: ::c_long = 237;
    pub const __NR_MIGRATE_PAGES: ::c_long = 238;
    pub const __NR_MOVE_PAGES: ::c_long = 239;
    
    //ENDIF
    
    pub const __NR_RT_TGSIGQUEUEINFO: ::c_long = 240;
    pub const __NR_PERF_EVENT_OPEN: ::c_long = 241;
    pub const __NR_ACCEPT4: ::c_long = 242;
    pub const __NR_RECVMMSG: ::c_long = 243;
    
    pub const __NR_ARCH_SPECIFIC_SYSCALL: ::c_long = 244;
    
    pub const __NR_WAIT4: ::c_long = 260;
    pub const __NR_PRLIMIT64: ::c_long = 261;
    pub const __NR_FANOTIFY_INIT: ::c_long = 262;
    pub const __NR_FANOTIFY_MARK: ::c_long = 263;
    pub const __NR_NAME_TO_HANDLE_AT: ::c_long = 264;
    pub const __NR_OPEN_BY_HANDLE_AT: ::c_long = 265;
    pub const __NR_CLOCK_ADJTIME: ::c_long = 266;
    pub const __NR_SYNCFS: ::c_long = 267;
    pub const __NR_SETNS: ::c_long = 268;
    pub const __NR_SENDMMSG: ::c_long = 269;
    pub const __NR_PROCESS_VM_READV: ::c_long = 270;
    pub const __NR_PROCESS_VM_WRITEV: ::c_long = 271;
    pub const __NR_KCMP: ::c_long = 272;
    pub const __NR_FINIT_MODULE: ::c_long = 273;
    pub const __NR_SCHED_SETATTR: ::c_long = 274;
    pub const __NR_SCHED_GETATTR: ::c_long = 275;
    pub const __NR_RENAMEAT2: ::c_long = 276;
    pub const __NR_SECCOMP: ::c_long = 277;
    pub const __NR_GETRANDOM: ::c_long = 278;
    pub const __NR_MEMFD_CREATE: ::c_long = 279;
    pub const __NR_BPF: ::c_long = 280;
    pub const __NR_EXECVEAT: ::c_long = 281;
    
    //TODO: Every few months, check kernel to see if these are being removed. They are slated to be removed "eventually"
    
    pub const __NR_OPEN: ::c_long = 1024;
    pub const __NR_LINK: ::c_long = 1025;
    pub const __NR_UNLINK: ::c_long = 1026;
    pub const __NR_MKNOD: ::c_long = 1027;
    pub const __NR_CHMOD: ::c_long = 1028;
    pub const __NR_CHOWN: ::c_long = 1029;
    pub const __NR_MKDIR: ::c_long = 1030;
    pub const __NR_RMDIR: ::c_long = 1031;
    pub const __NR_LCHOWN: ::c_long = 1032;
    pub const __NR_ACCESS: ::c_long = 1033;
    pub const __NR_RENAME: ::c_long = 1034;
    pub const __NR_READLINK: ::c_long = 1035;
    pub const __NR_SYMLINK: ::c_long = 1036;
    pub const __NR_UTIMES: ::c_long = 1037;
    pub const __NR3264_STAT: ::c_long = 1038;
    pub const __NR3264_LSTAT: ::c_long = 1039;
    
    pub const __NR_PIPE: ::c_long = 1040;
    pub const __NR_DUP2: ::c_long = 1041;
    pub const __NR_EPOLL_CREATE: ::c_long = 1042;
    pub const __NR_INOTIFY_INIT: ::c_long = 1043;
    pub const __NR_EVENTFD: ::c_long = 1044;
    pub const __NR_SIGNALFD: ::c_long = 1045;
    
    pub const __NR_SENDFILE: ::c_long = 1046;
    pub const __NR_FTRUNCATE: ::c_long = 1047;
    pub const __NR_TRUNCATE: ::c_long = 1048;
    pub const __NR_STST: ::c_long = 1049;
    pub const __NR_LSTAT: ::c_long = 1050;
    pub const __NR_FSTAT: ::c_long = 1051;
    pub const __NR_FCNTL: ::c_long = 1052;
    pub const __NR_FADVISE64: ::c_long = 1053;
    pub const __NR_FNEWFSTATAT: ::c_long = 1054;
    pub const __NR_FSTATFS: ::c_long = 1055;
    pub const __NR_STATFS: ::c_long = 1056;
    pub const __NR_LSEEK: ::c_long = 1057;
    pub const __NR_MMAP: ::c_long = 1058;
    
    pub const __NR_ALARM: ::c_long = 1059;
    pub const __NR_GETPGRP: ::c_long = 1060;
    pub const __NR_PAUSE: ::c_long = 1061;
    pub const __NR_TIME: ::c_long = 1062;
    pub const __NR_UTIME: ::c_long = 1063;
    
    pub const __NR_CREAT: ::c_long = 1064;
    pub const __NR_GETDENTS: ::c_long = 1065;
    pub const __NR_FUTIMESAT: ::c_long = 1066;
    pub const __NR_SELECT: ::c_long = 1067;
    pub const __NR_POLL: ::c_long = 1068;
    pub const __NR_EPOLL_WAIT: ::c_long = 1069;
    pub const __NR_USTAT: ::c_long = 1070;
    pub const __NR_VFORK: ::c_long = 1071;
    pub const __NR_OLDWAIT4: ::c_long = 1072;
    pub const __NR_RECV: ::c_long = 1073;
    pub const __NR_SEND: ::c_long = 1074;
    pub const __NR_BDFLUSH: ::c_long = 1075;
    pub const __NR_UMOUNT: ::c_long = 1076;
    pub const __NR_USELIB: ::c_long = 1077;
    pub const __NR__SYSCTL: ::c_long = 1078;
    
    pub const __NR_FORK: ::c_long = 1079;
    
}

#[cfg(target_arch = "arc")]
pub mod arc_unistd_extensions {

    pub const __NR_CACHEFLUSH: ::c_long = __NR_ARCH_SPECIFIC_SYSCALL;// + 0 (What?)
    pub const __NR_ARC_SETTLS: ::c_long = __NR_ARCH_SPECIFIC_SYSCALL + 1;
    pub const __NR_ARC_GETTLS: ::c_long = __NR_ARCH_SPECIFIC_SYSCALL + 2;
    pub const __NR_SYSFS: ::c_long = __NR_ARCH_SPECIFIC_SYSCAL + 3;

}

#[cfg(target_arch = "c6x")]
pub mod c6x_unistd_extensions {

    pub const __NR_CACHE_SYNC: ::c_long = __NR_ARCH_SPECIFIC_SYSCALL;// + 0, why are they doing this?

}

#[cfg(target_arch = "metag")]
pub mod metag_unistd_extensions {

    pub const __NR_METAG_SETGLOBALBIT: ::c_long = __NR_ARCH_SPECIFIC_SYSCALL + 1; //Finally! Somebody that makes sense! :)
    pub const __NR_METAG_SET_FPU_FLAGS: ::c_long = __NR_ARCH_SPECIFIC_SYSCALL + 2;
    pub const __NR_METAG_SET_TLS: ::c_long = __NR_ARCH_SPECIFIC_SYSCALL + 3;
    pub const __NR_METAG_GET_TLS: ::c_long = __NR_ARCH_SPECIFIC_SYSCALL + 4;

}

#[cfg(target_arch = "nios2")]
pub mod nios2_unistd_extensions {

    pub const __NR_CACHEFLUSH: ::c_long = __NR_ARCH_SPECIFIC_SYSCALL;

}

#[cfg(target_arch = "openrisc")]
pub mod openrisc_unistd_extensions {

    pub const __NR_OR1K_ATOMIC: ::c_long = __NR_ARCH_SPECIFIC_SYSCALL;

}

#[cfg(target_arch = "tile")]
pub mod tile_unistd_extensions {

    pub const __NR_CACHEFLUSH: ::c_long = __NR_ARCH_SPECIFIC_SYSCALL + 1;
    
    pub const __NR_FAST_CMPXCHG: ::c_long = -1;
    pub const __NR_FAST_ATOMIC_UPDATE: ::c_long = -2;
    pub const __NR_FAST_CMPXCHG64: ::c_long = -3;
    
    pub const __NR_CMPXCHF_BADADDR: ::c_long = __NR_ARCH_SPECIFIC_SYSCALL; // Bloody + 0 again

}

#[cfg(not(any(target_arch = "alpha",
              target_arch = "arm",
              target_arch = "avr32",
              target_arch = "blackfin",
              target_arch = "cris",
              target_arch = "frv",
              target_arch = "ia64",
              target_arch = "m32r",
              target_arch = "m68k",
              target_arch = "microblaze",
              target_arch = "mips",
              target_arch = "mn10300",
              target_arch = "parisc",
              target_arch = "powerpc",
              target_arch = "s390",
              target_arch = "sh",
              target_arch = "sparc")))]
pub use self::common_unistd::*;

#[cfg(target_arch = "arc")]
pub use self::arc_unistd_extensions::*;

#[cfg(target_arch = "c6x")]
pub use self::c6x_unistd_extensions::*;

#[cfg(target_arch = "metag")]
pub use self::metag_unistd_extensions::*;

#[cfg(target_arch = "nios2")]
pub use self::nios2_unistd_extensions::*;

#[cfg(target_arch = "openrisc")]
pub use self::openrisc_unistd_extensions::*;