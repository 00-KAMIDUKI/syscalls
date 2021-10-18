//! Syscalls for the mips architecture.

// This file is automatically generated. Do not edit!

syscall_enum! {
    pub enum Sysno {
        syscall = 4000,
        exit = 4001,
        fork = 4002,
        read = 4003,
        write = 4004,
        open = 4005,
        close = 4006,
        waitpid = 4007,
        creat = 4008,
        link = 4009,
        unlink = 4010,
        execve = 4011,
        chdir = 4012,
        time = 4013,
        mknod = 4014,
        chmod = 4015,
        lchown = 4016,
        r#break = 4017,
        unused18 = 4018,
        lseek = 4019,
        getpid = 4020,
        mount = 4021,
        umount = 4022,
        setuid = 4023,
        getuid = 4024,
        stime = 4025,
        ptrace = 4026,
        alarm = 4027,
        unused28 = 4028,
        pause = 4029,
        utime = 4030,
        stty = 4031,
        gtty = 4032,
        access = 4033,
        nice = 4034,
        ftime = 4035,
        sync = 4036,
        kill = 4037,
        rename = 4038,
        mkdir = 4039,
        rmdir = 4040,
        dup = 4041,
        pipe = 4042,
        times = 4043,
        prof = 4044,
        brk = 4045,
        setgid = 4046,
        getgid = 4047,
        signal = 4048,
        geteuid = 4049,
        getegid = 4050,
        acct = 4051,
        umount2 = 4052,
        lock = 4053,
        ioctl = 4054,
        fcntl = 4055,
        mpx = 4056,
        setpgid = 4057,
        ulimit = 4058,
        unused59 = 4059,
        umask = 4060,
        chroot = 4061,
        ustat = 4062,
        dup2 = 4063,
        getppid = 4064,
        getpgrp = 4065,
        setsid = 4066,
        sigaction = 4067,
        sgetmask = 4068,
        ssetmask = 4069,
        setreuid = 4070,
        setregid = 4071,
        sigsuspend = 4072,
        sigpending = 4073,
        sethostname = 4074,
        setrlimit = 4075,
        getrlimit = 4076,
        getrusage = 4077,
        gettimeofday = 4078,
        settimeofday = 4079,
        getgroups = 4080,
        setgroups = 4081,
        reserved82 = 4082,
        symlink = 4083,
        unused84 = 4084,
        readlink = 4085,
        uselib = 4086,
        swapon = 4087,
        reboot = 4088,
        readdir = 4089,
        mmap = 4090,
        munmap = 4091,
        truncate = 4092,
        ftruncate = 4093,
        fchmod = 4094,
        fchown = 4095,
        getpriority = 4096,
        setpriority = 4097,
        profil = 4098,
        statfs = 4099,
        fstatfs = 4100,
        ioperm = 4101,
        socketcall = 4102,
        syslog = 4103,
        setitimer = 4104,
        getitimer = 4105,
        stat = 4106,
        lstat = 4107,
        fstat = 4108,
        unused109 = 4109,
        iopl = 4110,
        vhangup = 4111,
        idle = 4112,
        vm86 = 4113,
        wait4 = 4114,
        swapoff = 4115,
        sysinfo = 4116,
        ipc = 4117,
        fsync = 4118,
        sigreturn = 4119,
        clone = 4120,
        setdomainname = 4121,
        uname = 4122,
        modify_ldt = 4123,
        adjtimex = 4124,
        mprotect = 4125,
        sigprocmask = 4126,
        create_module = 4127,
        init_module = 4128,
        delete_module = 4129,
        get_kernel_syms = 4130,
        quotactl = 4131,
        getpgid = 4132,
        fchdir = 4133,
        bdflush = 4134,
        sysfs = 4135,
        personality = 4136,
        afs_syscall = 4137,
        setfsuid = 4138,
        setfsgid = 4139,
        _llseek = 4140,
        getdents = 4141,
        _newselect = 4142,
        flock = 4143,
        msync = 4144,
        readv = 4145,
        writev = 4146,
        cacheflush = 4147,
        cachectl = 4148,
        sysmips = 4149,
        unused150 = 4150,
        getsid = 4151,
        fdatasync = 4152,
        _sysctl = 4153,
        mlock = 4154,
        munlock = 4155,
        mlockall = 4156,
        munlockall = 4157,
        sched_setparam = 4158,
        sched_getparam = 4159,
        sched_setscheduler = 4160,
        sched_getscheduler = 4161,
        sched_yield = 4162,
        sched_get_priority_max = 4163,
        sched_get_priority_min = 4164,
        sched_rr_get_interval = 4165,
        nanosleep = 4166,
        mremap = 4167,
        accept = 4168,
        bind = 4169,
        connect = 4170,
        getpeername = 4171,
        getsockname = 4172,
        getsockopt = 4173,
        listen = 4174,
        recv = 4175,
        recvfrom = 4176,
        recvmsg = 4177,
        send = 4178,
        sendmsg = 4179,
        sendto = 4180,
        setsockopt = 4181,
        shutdown = 4182,
        socket = 4183,
        socketpair = 4184,
        setresuid = 4185,
        getresuid = 4186,
        query_module = 4187,
        poll = 4188,
        nfsservctl = 4189,
        setresgid = 4190,
        getresgid = 4191,
        prctl = 4192,
        rt_sigreturn = 4193,
        rt_sigaction = 4194,
        rt_sigprocmask = 4195,
        rt_sigpending = 4196,
        rt_sigtimedwait = 4197,
        rt_sigqueueinfo = 4198,
        rt_sigsuspend = 4199,
        pread64 = 4200,
        pwrite64 = 4201,
        chown = 4202,
        getcwd = 4203,
        capget = 4204,
        capset = 4205,
        sigaltstack = 4206,
        sendfile = 4207,
        getpmsg = 4208,
        putpmsg = 4209,
        mmap2 = 4210,
        truncate64 = 4211,
        ftruncate64 = 4212,
        stat64 = 4213,
        lstat64 = 4214,
        fstat64 = 4215,
        pivot_root = 4216,
        mincore = 4217,
        madvise = 4218,
        getdents64 = 4219,
        fcntl64 = 4220,
        reserved221 = 4221,
        gettid = 4222,
        readahead = 4223,
        setxattr = 4224,
        lsetxattr = 4225,
        fsetxattr = 4226,
        getxattr = 4227,
        lgetxattr = 4228,
        fgetxattr = 4229,
        listxattr = 4230,
        llistxattr = 4231,
        flistxattr = 4232,
        removexattr = 4233,
        lremovexattr = 4234,
        fremovexattr = 4235,
        tkill = 4236,
        sendfile64 = 4237,
        futex = 4238,
        sched_setaffinity = 4239,
        sched_getaffinity = 4240,
        io_setup = 4241,
        io_destroy = 4242,
        io_getevents = 4243,
        io_submit = 4244,
        io_cancel = 4245,
        exit_group = 4246,
        lookup_dcookie = 4247,
        epoll_create = 4248,
        epoll_ctl = 4249,
        epoll_wait = 4250,
        remap_file_pages = 4251,
        set_tid_address = 4252,
        restart_syscall = 4253,
        fadvise64 = 4254,
        statfs64 = 4255,
        fstatfs64 = 4256,
        timer_create = 4257,
        timer_settime = 4258,
        timer_gettime = 4259,
        timer_getoverrun = 4260,
        timer_delete = 4261,
        clock_settime = 4262,
        clock_gettime = 4263,
        clock_getres = 4264,
        clock_nanosleep = 4265,
        tgkill = 4266,
        utimes = 4267,
        mbind = 4268,
        get_mempolicy = 4269,
        set_mempolicy = 4270,
        mq_open = 4271,
        mq_unlink = 4272,
        mq_timedsend = 4273,
        mq_timedreceive = 4274,
        mq_notify = 4275,
        mq_getsetattr = 4276,
        vserver = 4277,
        waitid = 4278,
        add_key = 4280,
        request_key = 4281,
        keyctl = 4282,
        set_thread_area = 4283,
        inotify_init = 4284,
        inotify_add_watch = 4285,
        inotify_rm_watch = 4286,
        migrate_pages = 4287,
        openat = 4288,
        mkdirat = 4289,
        mknodat = 4290,
        fchownat = 4291,
        futimesat = 4292,
        fstatat64 = 4293,
        unlinkat = 4294,
        renameat = 4295,
        linkat = 4296,
        symlinkat = 4297,
        readlinkat = 4298,
        fchmodat = 4299,
        faccessat = 4300,
        pselect6 = 4301,
        ppoll = 4302,
        unshare = 4303,
        splice = 4304,
        sync_file_range = 4305,
        tee = 4306,
        vmsplice = 4307,
        move_pages = 4308,
        set_robust_list = 4309,
        get_robust_list = 4310,
        kexec_load = 4311,
        getcpu = 4312,
        epoll_pwait = 4313,
        ioprio_set = 4314,
        ioprio_get = 4315,
        utimensat = 4316,
        signalfd = 4317,
        timerfd = 4318,
        eventfd = 4319,
        fallocate = 4320,
        timerfd_create = 4321,
        timerfd_gettime = 4322,
        timerfd_settime = 4323,
        signalfd4 = 4324,
        eventfd2 = 4325,
        epoll_create1 = 4326,
        dup3 = 4327,
        pipe2 = 4328,
        inotify_init1 = 4329,
        preadv = 4330,
        pwritev = 4331,
        rt_tgsigqueueinfo = 4332,
        perf_event_open = 4333,
        accept4 = 4334,
        recvmmsg = 4335,
        fanotify_init = 4336,
        fanotify_mark = 4337,
        prlimit64 = 4338,
        name_to_handle_at = 4339,
        open_by_handle_at = 4340,
        clock_adjtime = 4341,
        syncfs = 4342,
        sendmmsg = 4343,
        setns = 4344,
        process_vm_readv = 4345,
        process_vm_writev = 4346,
        kcmp = 4347,
        finit_module = 4348,
        sched_setattr = 4349,
        sched_getattr = 4350,
        renameat2 = 4351,
        seccomp = 4352,
        getrandom = 4353,
        memfd_create = 4354,
        bpf = 4355,
        execveat = 4356,
        userfaultfd = 4357,
        membarrier = 4358,
        mlock2 = 4359,
        copy_file_range = 4360,
        preadv2 = 4361,
        pwritev2 = 4362,
        pkey_mprotect = 4363,
        pkey_alloc = 4364,
        pkey_free = 4365,
        statx = 4366,
        rseq = 4367,
        io_pgetevents = 4368,
        semget = 4393,
        semctl = 4394,
        shmget = 4395,
        shmctl = 4396,
        shmat = 4397,
        shmdt = 4398,
        msgget = 4399,
        msgsnd = 4400,
        msgrcv = 4401,
        msgctl = 4402,
        clock_gettime64 = 4403,
        clock_settime64 = 4404,
        clock_adjtime64 = 4405,
        clock_getres_time64 = 4406,
        clock_nanosleep_time64 = 4407,
        timer_gettime64 = 4408,
        timer_settime64 = 4409,
        timerfd_gettime64 = 4410,
        timerfd_settime64 = 4411,
        utimensat_time64 = 4412,
        pselect6_time64 = 4413,
        ppoll_time64 = 4414,
        io_pgetevents_time64 = 4416,
        recvmmsg_time64 = 4417,
        mq_timedsend_time64 = 4418,
        mq_timedreceive_time64 = 4419,
        semtimedop_time64 = 4420,
        rt_sigtimedwait_time64 = 4421,
        futex_time64 = 4422,
        sched_rr_get_interval_time64 = 4423,
        pidfd_send_signal = 4424,
        io_uring_setup = 4425,
        io_uring_enter = 4426,
        io_uring_register = 4427,
        open_tree = 4428,
        move_mount = 4429,
        fsopen = 4430,
        fsconfig = 4431,
        fsmount = 4432,
        fspick = 4433,
        pidfd_open = 4434,
        clone3 = 4435,
        close_range = 4436,
        openat2 = 4437,
        pidfd_getfd = 4438,
        faccessat2 = 4439,
        process_madvise = 4440,
        epoll_pwait2 = 4441,
        mount_setattr = 4442,
        quotactl_path = 4443,
        landlock_create_ruleset = 4444,
        landlock_add_rule = 4445,
        landlock_restrict_self = 4446,
    }
    LAST: landlock_restrict_self;
}
