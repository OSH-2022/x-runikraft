# report.mk: 实验报告的makefile 
# Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿

# This file is part of Runikraft. Runikraft is free software,
# which is released under the BSD 3-Clause License; see LICENSE for detail.
# Runikraft is provided ``as is'', without any express or implied warrenties.

#Depends:
# XeLaTeX compiler, recommend TeXLive 2021+
# Noto Serif CJK and Noto Sans CJK (Ubuntu package: fonts-noto-cjk fonts-noto-cjk-extra)
# SimKai, SimFang

# 在运行时，该文件将被复制到build/report
TEX := xelatex
TEX_FLAGS := -interaction=nonstopmode

all: report

report: research-report.pdf

research-report.pdf: $(REPORT_ROOT_DIR)/11_research/research-report.tex
	rm -f research-report.aux research-report.out research-report.toc
	env TEXINPUTS=$(REPORT_ROOT_DIR)/11_research:$$TEXINPUTS $(TEX) $(TEX_FLAGS) $(REPORT_ROOT_DIR)/11_research/research-report.tex >research-report-run1.stdout
	env TEXINPUTS=$(REPORT_ROOT_DIR)/11_research:$$TEXINPUTS $(TEX) $(TEX_FLAGS) $(REPORT_ROOT_DIR)/11_research/research-report.tex >research-report-run2.stdout
	env TEXINPUTS=$(REPORT_ROOT_DIR)/11_research:$$TEXINPUTS $(TEX) $(TEX_FLAGS) $(REPORT_ROOT_DIR)/11_research/research-report.tex >research-report-run3.stdout
