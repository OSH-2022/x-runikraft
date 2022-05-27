// SPDX-License-Identifier: BSD-3-Clause
// errno.rs
// Authors: 陈建绿 <2512674094@qq.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

pub use Errno::*;

pub enum Errno {
    /// Operation not permitted
    EPerm,
    /// No such file or directory
    ENoEnt,
    /// No such process
    ESrch,
    /// Interrupted system call
    EIntr,
    /// Input/output error
    EIO,
    /// Device not configured
    ENXIo,
    /// Argument list too long
    E2Big,
    /// Exec format error
    ENoExec,
    /// Bad file descriptor
    EBadF,
    /// No child processes
    EChild,
    /// Resource deadlock avoided
    EDeadLk,
    /// Cannot allocate memory
    ENoMem,
    /// Permission denied
    EAccess,
    /// Bad address
    EFault,
    /// Block device required
    ENotBlk,
    /// Device busy
    EBusy,
    /// File exists
    EExist,
    /// Cross-device link
    EXDev,
    /// Operation not supported by device
    ENoDev,
    /// Not a directory
    ENotDir,
    /// Is a directory
    EIsDir,
    /// Invalid argument
    EInval,
    /// Too many open files in system
    ENFile,
    /// Too many open files
    EMFile,
    /// Inappropriate ioctl for device
    ENoTty,
    /// Text file busy
    ETxtBsy,
    /// File too large
    EFBig,
    /// No space left on device
    ENoSpc,
    /// Illegal seek
    ESPipe,
    /// Read-only file system
    EROFS,
    /// Too many links
    EMLink,
    /// Broken pipe
    EPipe,
    /// Numerical argument out of domain
    EDom,
    /// Result too large
    ERange,
    /// Resource temporarily unavailable
    EAgain,
    /// Operation now in progress
    EInProgress,
    /// Operation already in progress
    EAlready,
    /// Socket operation on non-socket
    ENotSock,
    /// Destination address required
    EDestAddrReq,
    /// Message too long
    EMsgSize,
    /// Protocol wrong type for socket
    EProtoType,
    /// Protocol not available
    ENoProtoOpt,
    /// Protocol not supported
    EProtoNoSupport,
    /// Socket type not supported
    ESocketNoSupport,
    /// Operation not supported on socket
    EOpNotSupp,
    /// Protocol family not supported
    EPFNoSupport,
    /// Address family not supported by protocol family
    EAFNoSupport,
    /// Address already in use
    EAddrInUse,
    /// Can't assign requested address
    EAddrNotAvail,
    /// Network is down
    ENetDown,
    /// Network is unreachable
    ENetUnreach,
    /// Network dropped connection on reset
    ENetReset,
    /// Software caused connection abort
    EConnAborted,
    /// Connection reset by peer
    EConnReset,
    /// No buffer space available
    ENoBufS,
    /// Socket is already connected
    EIsConn,
    /// Socket is not connected
    ENotConn,
    /// Can't send after socket shutdown
    EShutdown,
    /// Operation timed out
    ETimedOut,
    /// Connection refused
    EConnRefused,
    /// Too many levels of symbolic links
    ELoop,
    /// File name too long
    ENameTooLong,
    /// Host is down
    EHostDown,
    /// No route to host
    EHostUnreach,
    /// Directory not empty
    ENotEmpty,
    /// Too many processes
    EProcLim,
    /// Too many users
    EUsers,
    /// Disc quota exceeded
    EDQuot,
    /// Stale NFS file handle
    EStale,
    /// RPC struct is bad
    EBadRPC,
    /// RPC version wrong
    ERPCMisMatch,
    /// RPC prog
    EProgUnavail,
    /// Program version wrong
    EProgMisMatch,
    /// Bad procedure for program
    EProcUnavail,
    /// No locks available
    ENoLck,
    /// Function not implemented
    ENoSys,
    /// * Inappropriate file type or format
    EFType,
    /// Authentication error
    EAuth,
    /// Need authenticator
    ENeedAuth,
    /// Identifier removed
    EIDRm,
    /// No message of desired type
    ENoMsg,
    /// Value too large to be stored in data type
    EOverFlow,
    /// Operation canceled
    ECanceled,
    /// Illegal byte sequence
    EIlSeq,
    /// Attribute not found
    ENoAttr,
    /// Programming error
    EDoofus,
    /// Bad message
    EBadMsg,
    /// Multihop attempted
    EMultihop,
    /// Link has been severed
    ENoLink,
    /// Protocol error
    EProto,
    /// Capabilities insufficient
    ENotCapable,
    /// Not permitted in capability mode
    ECapMode,
    /// State not recoverable
    ENotRecoverable,
    /// Previous owner died
    EOwnerDead,
    /// Not supported
    ENotSup,
    // Some compatibility definitions
    /// equals EDeadLk
    EDeadLock,
    /// Operation would block, equals EAgain
    EWouldBlock
}
