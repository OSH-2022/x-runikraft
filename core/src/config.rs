// SPDX-License-Identifier: FSFAP
// config.rs
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.
// Copying and distribution of this file, with or without modification, are
// permitted in any medium without royalty provided the copyright notice and
// this notice are preserved. This file is offered as-is, without any warranty.

#[cfg(__runikraft_custom_config)]
include!{env!("RUNIKRAFT_CONFIG_FILE")}
#[cfg(not(feature="custom_config"))]
include!{"../../default_config.rs"}
