// SPDX-License-Identifier: BSD-3-Clause
// compat_list/mod.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.


mod slist;
mod list;
mod stailq;
mod tailq;
pub use slist::*;
pub use list::*;
pub use stailq::*;
pub use tailq::*;
