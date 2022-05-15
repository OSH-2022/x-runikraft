# SPDX-License-Identifier: BSD-3-Clause
# makefile for Runikraft

# Authors: 张子辰 <zichen350@gmail.com>

# Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

# Redistribution and use in source and binary forms, with or without
# modification, are permitted provided that the following conditions
# are met:
# 1. Redistributions of source code must retain the above copyright
#    notice, this list of conditions and the following disclaimer.
# 2. Redistributions in binary form must reproduce the above copyright
#    notice, this list of conditions and the following disclaimer in the
#    documentation and/or other materials provided with the distribution.
# 3. Neither the name of the copyright holder nor the names of its
#    contributors may be used to endorse or promote products derived from
#    this software without specific prior written permission.
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
# AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
# IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
# ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
# LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
# CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
# SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
# INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
# CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
# ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
# POSSIBILITY OF SUCH DAMAGE.
 
#Depends:
# doc
# 	XeLaTeX compiler, recommend TeXLive 2021+
# 	Noto Serif CJK and Noto Sans CJK (Ubuntu package: fonts-noto-cjk fonts-noto-cjk-extra)
# 	SimKai, SimFang

PWD := $(shell pwd)
export MAKE_ROOT_DIR := $(PWD)/build
export REPORT_ROOT_DIR := $(PWD)/report
MAKE_BUILD_TYPE ?= release
OBJCOPY_PREFIX ?= rust-
RUST_OUTPUT_DIR := $(MAKE_ROOT_DIR)/dev-test/$(MAKE_BUILD_TYPE)
RUST_BUILD_DIR := $(MAKE_ROOT_DIR)/riscv64gc-unknown-none-elf/$(MAKE_BUILD_TYPE)

.PHONY: all
all: dev-test

.PHONY: everything
everything: all report

.PHONY: report
report: $(MAKE_ROOT_DIR)/report/makefile
	cd $(MAKE_ROOT_DIR)/report && $(MAKE)

.PHONY: dev-test
dev-test: $(RUST_OUTPUT_DIR)/dev-test.bin

$(RUST_OUTPUT_DIR)/dev-test.bin: $(RUST_OUTPUT_DIR)/dev-test
	$(OBJCOPY_PREFIX)objcopy --strip-all $(RUST_OUTPUT_DIR)/dev-test -O binary $(RUST_OUTPUT_DIR)/dev-test.bin

$(RUST_OUTPUT_DIR)/dev-test: $(RUST_BUILD_DIR)/dev-test
	-mkdir --parents $(RUST_OUTPUT_DIR)
	cp $(RUST_BUILD_DIR)/dev-test $(RUST_OUTPUT_DIR)/dev-test

.PHONY: $(RUST_BUILD_DIR)/dev-test
$(RUST_BUILD_DIR)/dev-test: $(MAKE_ROOT_DIR)/liballoc_error_handler.rlib $(RUST_BUILD_DIR)/deps/liballoc_error_handler.rlib
ifeq ($(MAKE_BUILD_TYPE), release)
	cd dev-test && cargo build --release --features rkalloc/__alloc_error_handler
else
ifeq ($(MAKE_BUILD_TYPE), debug)
	cd dev-test && cargo build --features rkalloc/__alloc_error_handler
else
	@echo "Unknown build type, expect release/debug."
	false
endif
endif
	
$(RUST_BUILD_DIR)/deps/liballoc_error_handler.rlib: $(MAKE_ROOT_DIR)/liballoc_error_handler.rlib
	-mkdir --parents $(RUST_BUILD_DIR)/deps
	cp $(MAKE_ROOT_DIR)/liballoc_error_handler.rlib $(RUST_BUILD_DIR)/deps/liballoc_error_handler.rlib

$(MAKE_ROOT_DIR)/liballoc_error_handler.rlib: lib/rkalloc/alloc_error_handler.rs
	env RUSTC_BOOTSTRAP=1 rustc --edition=2021 lib/rkalloc/alloc_error_handler.rs --crate-type lib --target riscv64gc-unknown-none-elf -o $(MAKE_ROOT_DIR)/liballoc_error_handler.rlib

$(MAKE_ROOT_DIR)/report/makefile: makefiles/report.mk
	-mkdir --parents $(MAKE_ROOT_DIR)/report
	cp makefiles/report.mk $(MAKE_ROOT_DIR)/report/makefile
