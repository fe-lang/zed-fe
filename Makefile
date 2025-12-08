TREE_SITTER_FE := tree-sitter-fe

QUERY_FILES := brackets.scm embedding.scm highlights.scm indents.scm \
               injections.scm outline.scm overrides.scm runnables.scm \
               textobjects.scm

.PHONY: sync sync-queries sync-grammar sync-submodule help

help:
	@echo "Targets:"
	@echo "  sync          - pull latest submodule, sync queries and grammar info"
	@echo "  sync-queries  - copy .scm files only"
	@echo "  sync-grammar  - update extension.toml repository and commit"

sync: sync-submodule sync-queries sync-grammar

sync-submodule:
	git submodule update --init --remote $(TREE_SITTER_FE)

sync-queries: sync-submodule
	@echo "Copying queries from $(TREE_SITTER_FE)/queries/"
	@for f in $(QUERY_FILES); do \
		cp "$(TREE_SITTER_FE)/queries/$$f" languages/fe/ && echo "  $$f"; \
	done

sync-grammar: sync-submodule
	@REPO=$$(cd $(TREE_SITTER_FE) && git remote get-url origin | sed 's|git@github.com:|https://github.com/|'); \
	COMMIT=$$(cd $(TREE_SITTER_FE) && git rev-parse HEAD); \
	sed -i "/^\[grammars\.fe\]$$/,/^$$\|^\[/{s|^repository = .*|repository = \"$$REPO\"|;s|^commit = .*|commit = \"$$COMMIT\"|}" extension.toml; \
	echo "Updated extension.toml [grammars.fe]:"; \
	echo "  repository: $$REPO"; \
	echo "  commit: $$COMMIT"
