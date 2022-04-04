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
TEX := latexmk
TEX_FLAGS := -xelatex -silent -latexoption=-interaction=nonstopmode

all: report

report: research-report.pdf feasibility-report.pdf

research-report.pdf: $(REPORT_ROOT_DIR)/11_research/research-report.tex
	env TEXINPUTS=$(REPORT_ROOT_DIR)/11_research:$$TEXINPUTS $(TEX) $(TEX_FLAGS) $(REPORT_ROOT_DIR)/11_research/research-report.tex

feasibility-report.pdf: $(REPORT_ROOT_DIR)/20_feasibility/feasibility-report.tex
	env TEXINPUTS=$(REPORT_ROOT_DIR)/20_feasibility:$$TEXINPUTS $(TEX) $(TEX_FLAGS) $(REPORT_ROOT_DIR)/20_feasibility/feasibility-report.tex
