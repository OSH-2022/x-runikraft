// SPDX-License-Identifier: BSD-3-Clause
// constants.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

/// RISC-V支持2^(MXLEN-1)（2^31/2^63/2^127）种中断/异常，
/// 但目前大于等于64的exception code是被保留的。
/// 
/// 只处理中断，不处理异常。
pub const MAX_IRQ: usize = 64;

