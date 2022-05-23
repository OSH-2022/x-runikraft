// SPDX-License-Identifier: BSD-3-Clause
// signal.rs

// Authors: Mihai Pogonaru <pogonarumihai@gmail.com>
//          Teodora Serbanescu <teo.serbanescu16@gmail.com>
//          Felipe Huici <felipe.huici@neclab.eu>
//          Bernard Rizzo <b.rizzo@student.uliege.be>
//          蓝俊玮 <ljw13@mail.ustc.edu.cn>

// Copyright (c) 2021, University Politehnica of Bucharest.
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
// 3. Neither the name of the copyright holder nor the names of its
//    contributors may be used to endorse or promote products derived from
//    this software without specific prior written permission.
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
use rklist::List;

pub type Sigset = u64;

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
    pub sa_mask: Sigset,
    pub sa_flags: i32,
    pub sa_restorer: fn()
}

pub struct Thread;

pub struct Signal<'a> {
    pub info: Siginfo,
    pub list_node: List<'a, u8>
}

pub struct ProcSig<'a> {
    pub pending: Sigset,
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
    pub awaited: Sigset,
    pub received_signal: Siginfo
}

pub struct ThreadSig<'a> {
    pub mask: Sigset,
    pub pending: Sigset,
    pub pending_signals: List<'a, u8>,
    pub wait: ThreadSigWait,
    pub list_node: List<'a, u8>
}
