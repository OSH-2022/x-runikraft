# SPDX-License-Identifier: BSD-3-Clause
# report.mk: makefile for project reports

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
# XeLaTeX compiler, recommend TeXLive 2021+
# Noto Serif CJK and Noto Sans CJK (Ubuntu package: fonts-noto-cjk fonts-noto-cjk-extra)
# SimKai, SimFang

# 在运行时，该文件将被复制到build/report
TEX := latexmk
TEX_FLAGS := -xelatex -silent -latexoption=-interaction=nonstopmode

.PHONY: all
all: research-report.pdf feasibility-report.pdf

../runikraft-report.cls: $(REPORT_ROOT_DIR)/runikraft-report.cls
	cp $(REPORT_ROOT_DIR)/runikraft-report.cls ../runikraft-report.cls

research-report.pdf: $(REPORT_ROOT_DIR)/2_research/research-report.tex ../runikraft-report.cls
	env TEXINPUTS=$(REPORT_ROOT_DIR)/2_research:$$TEXINPUTS $(TEX) $(TEX_FLAGS) $(REPORT_ROOT_DIR)/2_research/research-report.tex

feasibility-report.bib: $(REPORT_ROOT_DIR)/3_feasibility/feasibility-report.bib
	cp $(REPORT_ROOT_DIR)/3_feasibility/feasibility-report.bib feasibility-report.bib

feasibility-report.pdf: $(REPORT_ROOT_DIR)/3_feasibility/feasibility-report.tex feasibility-report.bib ../runikraft-report.cls
	env TEXINPUTS=$(REPORT_ROOT_DIR)/3_feasibility:$$TEXINPUTS $(TEX) $(TEX_FLAGS) $(REPORT_ROOT_DIR)/3_feasibility/feasibility-report.tex

final-report.pdf: $(REPORT_ROOT_DIR)/5_final/final-report.tex ../runikraft-report.cls
	env TEXINPUTS=$(REPORT_ROOT_DIR)/5_final:$$TEXINPUTS $(TEX) $(TEX_FLAGS) $(REPORT_ROOT_DIR)/5_final/final-report.tex
