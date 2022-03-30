# runikraft的makefile
#Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿

# This file is part of Runikraft. Runikraft is free software,
# which is released under the BSD 3-Clause License; see LICENSE for detail.
# Runikraft is provided ``as is'', without any express or implied warrenties.
 
#Depends:
# doc
# 	XeLaTeX compiler, recommend TeXLive 2021+
# 	Noto Serif CJK and Noto Sans CJK (Ubuntu package: fonts-noto-cjk fonts-noto-cjk-extra)
# 	SimKai, SimFang

export MAKE_ROOT_DIR := $(shell pwd)/build
export REPORT_ROOT_DIR := $(shell pwd)/report

#currently nothing for all
.PHONY: all
all:

report: $(MAKE_ROOT_DIR)/report/makefile
	cd $(MAKE_ROOT_DIR)/report && $(MAKE)

$(MAKE_ROOT_DIR)/report/makefile: makefiles/report.mk
	-mkdir --parents $(MAKE_ROOT_DIR)/report
	cp makefiles/report.mk $(MAKE_ROOT_DIR)/report/makefile
