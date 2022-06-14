// SPDX-License-Identifier: BSD-3-Clause
// errno.rs
// Authors: 陈建绿 <2512674094@qq.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

#[derive(Debug)]
#[repr(C)]
pub enum Errno {
    /// No error occurred
    Ok=0,
    /// Operation not permitted
    Perm=1,
    /// No such file or directory
    NoEnt=2,
    /// No such process
    Srch=3,
    /// Interrupted system call
    Intr=4,
    /// Input/output error
    IO=5,
    /// Device not configured
    NXIo=6,
    /// Argument list too long
    TooBig=7,
    /// Exec format error
    NoExec=8,
    /// Bad file descriptor
    BadF=9,
    /// No child processes
    Child=10,
    /// Resource deadlock avoided
    DeadLk=11,
    /// Cannot allocate memory
    NoMem=12,
    /// Permission denied
    Access=13,
    /// Bad address
    Fault=14,
    /// Block device required
    NotBlk=15,
    /// Device busy
    Busy=16,
    /// File exists
    Exist=17,
    /// Cross-device link
    XDev=18,
    /// Operation not supported by device
    NoDev=19,
    /// Not a directory
    NotDir=20,
    /// Is a directory
    IsDir=21,
    /// Invalid argument
    Inval=22,
    /// Too many open files in system
    NFile=23,
    /// Too many open files
    MFile=24,
    /// Inappropriate ioctl for device
    NoTty=25,
    /// Text file busy
    TxtBsy=26,
    /// File too large
    FBig=27,
    /// No space left on device
    NoSpc=28,
    /// Illegal seek
    SPipe=29,
    /// Read-only file system
    ROFS=30,
    /// Too many links
    MLink=31,
    /// Broken pipe
    Pipe=32,
    /// Numerical argument out of domain
    Dom=33,
    /// Result too large
    Range=34,
    /// Resource temporarily unavailable
    Again=35,
    /// Operation now in progress
    InProgress=36,
    /// Operation already in progress
    Already=37,
    /// Socket operation on non-socket
    NotSock=38,
    /// Destination address required
    DestAddrReq=39,
    /// Message too long
    MsgSize=40,
    /// Protocol wrong type for socket
    ProtoType=41,
    /// Protocol not available
    NoProtoOpt=42,
    /// Protocol not supported
    ProtoNoSupport=43,
    /// Socket type not supported
    SocketNoSupport=44,
    /// Operation not supported on socket
    OpNotSupp=45,
    /// Protocol family not supported
    PFNoSupport=46,
    /// Address family not supported by protocol family
    AFNoSupport=47,
    /// Address already in use
    AddrInUse=48,
    /// Can't assign requested address
    AddrNotAvail=49,
    /// Network is down
    NetDown=50,
    /// Network is unreachable
    NetUnreach=51,
    /// Network dropped connection on reset
    NetReset=52,
    /// Software caused connection abort
    ConnAborted=53,
    /// Connection reset by peer
    ConnReset=54,
    /// No buffer space available
    NoBufS=55,
    /// Socket is already connected
    IsConn=56,
    /// Socket is not connected
    NotConn=57,
    /// Can't send after socket shutdown
    Shutdown=58,
    /// Operation timed out
    TimedOut=60,
    /// Connection refused
    ConnRefused=61,
    /// Too many levels of symbolic links
    Loop=62,
    /// File name too long
    NameTooLong=63,
    /// Host is down
    HostDown=64,
    /// No route to host
    HostUnreach=65,
    /// Directory not empty
    NotEmpty=66,
    /// Too many processes
    ProcLim=67,
    /// Too many users
    Users=68,
    /// Disc quota exceeded
    DQuot=69,
    /// Stale NFS file handle
    Stale=70,
    /// RPC struct is bad
    BadRPC=72,
    /// RPC version wrong
    RPCMisMatch=73,
    /// RPC prog
    ProgUnavail=74,
    /// Program version wrong
    ProgMisMatch=75,
    /// Bad procedure for program
    ProcUnavail=76,
    /// No locks available
    NoLck=77,
    /// Function not implemented
    NoSys=78,
    /// * Inappropriate file type or format
    FType=79,
    /// Authentication error
    Auth=80,
    /// Need authenticator
    NeedAuth=81,
    /// Identifier removed
    IDRm=82,
    /// No message of desired type
    NoMsg=83,
    /// Value too large to be stored in data type
    OverFlow=84,
    /// Operation canceled
    Canceled=85,
    /// Illegal byte sequence
    IlSeq=86,
    /// Attribute not found
    NoAttr=87,
    /// Programming error
    Doofus=88,
    /// Bad message
    BadMsg=89,
    /// Multihop attempted
    Multihop=90,
    /// Link has been severed
    NoLink=91,
    /// Protocol error
    Proto=92,
    /// Capabilities insufficient
    NotCapable=93,
    /// Not permitted in capability mode
    CapMode=94,
    /// State not recoverable
    NotRecoverable=95,
    /// Previous owner died
    OwnerDead=96,
    /// Not supported
    NotSup=97,
    // 枚举中不能有同名成员
    // // Some compatibility definitions
    // /// equals EDeadLk
    // DeadLock=11,
    // /// Operation would block, equals EAgain
    // WouldBlock=35,
}
