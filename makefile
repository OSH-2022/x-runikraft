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
export TEST_ROOT_DIR := $(PWD)/test
export SRC_ROOT_DIR := $(PWD)
export MAKE_BUILD_TYPE := debug
export CROSS_COMPILE := riscv64-linux-gnu-
TEST_LIST := @all
IGNORED_LIST := 

.PHONY: all
all: test

.PHONY: everything
everything: all report

.PHONY: report
report: $(MAKE_ROOT_DIR)/report/makefile
	cd "$(MAKE_ROOT_DIR)/report" && $(MAKE)

$(MAKE_ROOT_DIR)/report/makefile: makefiles/report.mk
	-mkdir --parents "$(MAKE_ROOT_DIR)/report"
	cp makefiles/report.mk "$(MAKE_ROOT_DIR)/report/makefile"

.PHONY: test build_test $(MAKE_ROOT_DIR)/test/makefile
test: $(MAKE_ROOT_DIR)/test/makefile opensbi
	cd "$(MAKE_ROOT_DIR)/test" && $(MAKE)

build_test: $(MAKE_ROOT_DIR)/test/makefile
	cd "$(MAKE_ROOT_DIR)/test" && $(MAKE) build

$(MAKE_ROOT_DIR)/test/makefile: makefiles/test.mk.sh makefiles/test.mk.0 makefiles/test.mk.1
	-mkdir --parents "$(MAKE_ROOT_DIR)/test"
	makefiles/test.mk.sh makefiles/test.mk "$(MAKE_ROOT_DIR)/test/makefile" "$(TEST_ROOT_DIR)" "$(TEST_LIST)" "$(IGNORED_LIST)" "$(MAKE_ROOT_DIR)/opensbi/platform/generic/firmware/fw_jump.bin"

.PHONY: opensbi
opensbi:
#FW_OPTIONS=1 indicates quiet boot
	-mkdir --parents "$(MAKE_ROOT_DIR)/opensbi"
	cd opensbi && $(MAKE) PLATFORM=generic FW_OPTIONS=1 FW_DYNAMIC=n FW_JUMP=y FW_PAYLOAD=n O="$(MAKE_ROOT_DIR)/opensbi"
