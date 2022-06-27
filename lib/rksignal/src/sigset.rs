// SPDX-License-Identifier: BSD-3-Clause
// sigset.rs
// Authors: 蓝俊玮 <ljw13@mail.ustc.edu.cn>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

// indirectly taken from newlib
use super::signal::Sigset;

pub fn sigemptyset(ptr: &mut Sigset) -> i32 {
    *ptr = 0 as u64;
    return 0;
}

pub fn sigfillset(ptr: &mut Sigset) -> i32 {
    *ptr = !0;
    return 0;
}

pub fn sigaddset(ptr: &mut Sigset, signo: i32) -> i32 {
    //if (signo >= NSIG || signo <= 0) {
	//	errno = EINVAL;
	//	return -1;
	//}

    *ptr |= 1 << (signo - 1);
    return 0;
}

pub fn sigdelset(ptr: &mut Sigset, signo: i32) -> i32 {
    //if (signo >= NSIG || signo <= 0) {
	//	errno = EINVAL;
	//	return -1;
	//}

    *ptr &= !(1 << (signo - 1));
    return 0;
}

pub fn sigcopyset(ptr1: &mut Sigset, ptr2: &mut Sigset) {
    *ptr1 = *ptr2;
}

pub fn sigandset(ptr1: &mut Sigset, ptr2: &mut Sigset) {
    *ptr1 &= *ptr2;
}

pub fn sigorset(ptr1: &mut Sigset, ptr2: &mut Sigset) {
    *ptr1 |= *ptr2;
}

pub fn sigreverseset(ptr: &mut Sigset) {
    *ptr = !(*ptr);
}

pub fn sigismember(ptr: &mut Sigset, signo: i32) -> i32 {
    //if (signo >= NSIG || signo <= 0) {
	//	errno = EINVAL;
	//	return -1;
	//}

    if *ptr & (1 << (signo - 1)) != 0 {
        return 1;
    }
    else {
        return 0;
    }
}

pub fn sigisempty(ptr: &mut Sigset) -> i32 {
    if *ptr == 0 {
        return 1;
    }
    else {
        return 0;
    }
}
