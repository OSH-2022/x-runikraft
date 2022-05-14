use runikraft::list::List;

pub type sigset = u64;

pub union Sahandler {
    pub sa_handler: fn(i32),
    pub sa_sigaction: fn(i32, *mut Siginfo, *mut u8),
}

pub struct Siginfo {
    pub si_signo: i32,
    pub si_code: i32,
    pub si_pid: i32
}

pub struct Sigaction {
    pub sa_handler: Sahandler,
    pub sa_mask: sigset,
    pub sa_flags: i32,
    pub sa_restorer: fn()
}

pub struct Thread;

pub struct Signal<'a> {
    pub info: Siginfo,
    pub list_node: List<'a, u8>
}

pub struct ProcSig<'a> {
    pub pending: sigset,
    pub pending_signals: [Siginfo; 32],
    pub sigaction: [Sigaction; 32],
    pub list_node: List<'a, u8>
}

pub enum RkSigWaiting {
    SigNotWaiting = 0,
    SigWaiting = 1,
    SigWaitingSched = 2,
}

pub struct ThreadSigWait {
    pub status: RkSigWaiting,
    pub awaited: sigset,
    pub received_signal: Siginfo
}

pub struct ThreadSig<'a> {
    pub mask: sigset,
    pub pending: sigset,
    pub pending_signals: List<'a, u8>,
    pub wait: ThreadSigWait,
    pub list_node: List<'a, u8>
}
