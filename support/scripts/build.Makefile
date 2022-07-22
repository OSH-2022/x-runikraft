# SPDX-License-Identifier: GPL-2.0
#
# Houses the targets which top level Makfiles can also define.
PHONY += clean
clean: $(clean-subdirs)
	$(MAKE) -C $(KCONFIG_DIR)/ clean

version-check: $(SRC_ROOT_DIR)/config/project.release
	@echo Version: $(PROJECTVERSION)
	@echo Release: $(PROJECTRELEASE)

PHONY += help
help:
	@$(MAKE) -s -C $(KCONFIG_DIR) help
	@echo "Debugging"
	@echo "version-check      - demos version release functionality"
	@echo "clean              - cleans all output files"

.PHONY: $(PHONY)
