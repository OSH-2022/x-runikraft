use runikraft::list::List;

pub type sigset = u64;

union Sahandler {
    sa_handler: fn(i32),
    sa_sigaction: fn(i32, *mut Siginfo, *mut u8),
}

pub struct Siginfo {
    si_signo: i32,
    si_code: i32,
    si_pid: i32
}

pub struct Sigaction {
    sa_handler: Sahandler,
    sa_mask: sigset,
    sa_flags: i32,
    sa_restorer: fn()
}

pub struct Thread;

pub struct Signal<'a> {
    info: Siginfo,
    list_node: List<'a, u8>
}

pub struct ProcSig<'a> {
    pending: sigset,
    pending_signals: [Siginfo; 32],
    sigaction: [Sigaction; 32],
    list_node: List<'a, u8>
}

enum RkSigWaiting {
    SigNotWaiting = 0,
    SigWaiting = 1,
    SigWaitingSched = 2,
}

pub struct ThreadSigWait {
    status: RkSigWaiting,
    awaited: sigset,
    received_signal: Siginfo
}

pub struct ThreadSig<'a> {
    mask: sigset,
    pending: sigset,
    pending_signals: List<'a, u8>,
    wait: ThreadSigWait,
    list_node: List<'a, u8>
}
