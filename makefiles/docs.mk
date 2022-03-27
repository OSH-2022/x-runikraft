# docs.mk: 实验报告的makefile 
# Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿
# All rights reserved.

#Depends:
# XeLaTeX compiler, recommend TeXLive 2021+
# Noto Serif CJK and Noto Sans CJK (Ubuntu package: fonts-noto-cjk fonts-noto-cjk-extra)
# SimKai, SimFang

# 在运行时，该文件将被复制到build/docs
TEX := xelatex
TEX_FLAGS := -interaction=nonstopmode

all: report

report: research-report.pdf

research-report.pdf: $(DOCS_ROOT_DIR)/11_research/research-report.tex
	$(TEX) $(TEX_FLAGS) $(DOCS_ROOT_DIR)/11_research/research-report.tex
	$(TEX) $(TEX_FLAGS) $(DOCS_ROOT_DIR)/11_research/research-report.tex

