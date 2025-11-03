# Cyx Makefile - Cross-platform build and development automation

# Detect operating system
UNAME_S := $(shell uname -s 2>/dev/null || echo Windows)
UNAME_M := $(shell uname -m 2>/dev/null || echo unknown)

# OS-specific configuration
ifeq ($(UNAME_S),Linux)
    OS := linux
    BINARY_EXT :=
    SYMLINK_CMD := ln -sf
    INSTALL_DIR := /usr/local/bin
    SHELL_DETECT := $(shell echo $$SHELL)
endif

ifeq ($(UNAME_S),Darwin)
    OS := macos
    BINARY_EXT :=
    SYMLINK_CMD := ln -sf
    INSTALL_DIR := /usr/local/bin
    SHELL_DETECT := $(shell echo $$SHELL)
endif

ifeq ($(UNAME_S),Windows)
    OS := windows
    BINARY_EXT := .exe
    SYMLINK_CMD := copy
    INSTALL_DIR := C:\Program Files\cyx
    SHELL_DETECT := cmd
endif

ifneq (,$(findstring MINGW,$(UNAME_S)))
    OS := windows
    BINARY_EXT := .exe
    SYMLINK_CMD := cp -f
    INSTALL_DIR := /usr/local/bin
    SHELL_DETECT := bash
endif

ifneq (,$(findstring MSYS,$(UNAME_S)))
    OS := windows
    BINARY_EXT := .exe
    SYMLINK_CMD := cp -f
    INSTALL_DIR := /usr/local/bin
    SHELL_DETECT := bash
endif

ifneq (,$(findstring CYGWIN,$(UNAME_S)))
    OS := windows
    BINARY_EXT := .exe
    SYMLINK_CMD := cp -f
    INSTALL_DIR := /usr/local/bin
    SHELL_DETECT := bash
endif

# Project configuration
BINARY_NAME := cyx$(BINARY_EXT)
TARGET_DIR := target/release
BINARY_PATH := $(TARGET_DIR)/$(BINARY_NAME)
BINARY_ABS_PATH := $(shell pwd)/$(BINARY_PATH)
SYSTEM_SYMLINK := $(INSTALL_DIR)/$(BINARY_NAME)

# Cargo configuration
CARGO := cargo
CARGO_BUILD_FLAGS := --release
CARGO_FMT_FLAGS := --all
CARGO_CLIPPY_FLAGS := --all-targets --all-features -- -D warnings

# Colors for output (using tput if available)
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
BLUE := \033[0;34m
MAGENTA := \033[0;35m
CYAN := \033[0;36m
RESET := \033[0m

# Default target
.DEFAULT_GOAL := help

.PHONY: help
help: ## Show this help message
	@echo "$(CYAN)╔════════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(CYAN)║$(RESET)  $(MAGENTA)Cyx - Cybersecurity Companion Makefile$(RESET)              $(CYAN)║$(RESET)"
	@echo "$(CYAN)╚════════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@echo "$(BLUE)Detected OS:$(RESET) $(GREEN)$(OS)$(RESET) ($(UNAME_S))"
	@echo "$(BLUE)Architecture:$(RESET) $(GREEN)$(UNAME_M)$(RESET)"
	@echo "$(BLUE)Binary:$(RESET) $(GREEN)$(BINARY_NAME)$(RESET)"
	@echo ""
	@echo "$(YELLOW)Available targets:$(RESET)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-15s$(RESET) %s\n", $$1, $$2}'
	@echo ""

.PHONY: info
info: ## Display system and project information
	@echo "$(CYAN)═══════════════════════════════════════════════════════════$(RESET)"
	@echo "$(MAGENTA)System Information$(RESET)"
	@echo "$(CYAN)═══════════════════════════════════════════════════════════$(RESET)"
	@echo "$(BLUE)Operating System:$(RESET)     $(GREEN)$(OS)$(RESET)"
	@echo "$(BLUE)Platform:$(RESET)             $(GREEN)$(UNAME_S)$(RESET)"
	@echo "$(BLUE)Architecture:$(RESET)         $(GREEN)$(UNAME_M)$(RESET)"
	@echo "$(BLUE)Shell:$(RESET)                $(GREEN)$(SHELL_DETECT)$(RESET)"
	@echo ""
	@echo "$(CYAN)═══════════════════════════════════════════════════════════$(RESET)"
	@echo "$(MAGENTA)Project Configuration$(RESET)"
	@echo "$(CYAN)═══════════════════════════════════════════════════════════$(RESET)"
	@echo "$(BLUE)Binary Name:$(RESET)          $(GREEN)$(BINARY_NAME)$(RESET)"
	@echo "$(BLUE)Binary Path:$(RESET)          $(GREEN)$(BINARY_PATH)$(RESET)"
	@echo "$(BLUE)System Symlink:$(RESET)       $(GREEN)$(SYSTEM_SYMLINK)$(RESET) $(BLUE)→$(RESET) $(GREEN)$(BINARY_ABS_PATH)$(RESET)"
	@echo "$(BLUE)Install Directory:$(RESET)    $(GREEN)$(INSTALL_DIR)$(RESET)"
	@echo ""
	@echo "$(CYAN)═══════════════════════════════════════════════════════════$(RESET)"
	@echo "$(MAGENTA)Rust Toolchain$(RESET)"
	@echo "$(CYAN)═══════════════════════════════════════════════════════════$(RESET)"
	@which rustc > /dev/null && rustc --version || echo "$(RED)Rust not found!$(RESET)"
	@which cargo > /dev/null && cargo --version || echo "$(RED)Cargo not found!$(RESET)"
	@echo ""

.PHONY: check-rust
check-rust: ## Check if Rust is installed
	@echo "$(BLUE)Checking Rust installation...$(RESET)"
	@which rustc > /dev/null || (echo "$(RED)Error: Rust is not installed. Install from https://rustup.rs/$(RESET)" && exit 1)
	@which cargo > /dev/null || (echo "$(RED)Error: Cargo is not installed.$(RESET)" && exit 1)
	@echo "$(GREEN)✓ Rust is installed$(RESET)"
	@rustc --version
	@cargo --version

.PHONY: deps
deps: check-rust ## Check and install dependencies
	@echo "$(BLUE)Checking dependencies...$(RESET)"
	@$(CARGO) --version > /dev/null 2>&1 || (echo "$(RED)Error: Cargo not found$(RESET)" && exit 1)
	@echo "$(GREEN)✓ All dependencies satisfied$(RESET)"

.PHONY: build
build: deps ## Build the release binary and update symlink
	@echo "$(CYAN)╔════════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(CYAN)║$(RESET)  $(MAGENTA)Building Cyx (Release Mode)$(RESET)                          $(CYAN)║$(RESET)"
	@echo "$(CYAN)╚════════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@$(CARGO) build $(CARGO_BUILD_FLAGS)
	@echo ""
	@echo "$(GREEN)✓ Build completed successfully!$(RESET)"
	@echo ""
	@$(MAKE) --no-print-directory symlink

.PHONY: build-dev
build-dev: deps ## Build the debug binary (faster compilation)
	@echo "$(CYAN)╔════════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(CYAN)║$(RESET)  $(MAGENTA)Building Cyx (Debug Mode)$(RESET)                            $(CYAN)║$(RESET)"
	@echo "$(CYAN)╚════════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@$(CARGO) build
	@echo ""
	@echo "$(GREEN)✓ Debug build completed!$(RESET)"

.PHONY: symlink
symlink: ## Create/update system symlink to development binary
	@echo "$(BLUE)Creating system symlink: $(RESET)$(SYSTEM_SYMLINK) $(BLUE)→$(RESET) $(BINARY_ABS_PATH)"
	@if [ ! -f "$(BINARY_PATH)" ]; then \
		echo "$(RED)Error: Binary not found at $(BINARY_PATH)$(RESET)"; \
		echo "$(YELLOW)Run 'make build' first$(RESET)"; \
		exit 1; \
	fi
	@if [ ! -d "$(INSTALL_DIR)" ]; then \
		echo "$(YELLOW)Creating directory: $(INSTALL_DIR)$(RESET)"; \
		sudo mkdir -p $(INSTALL_DIR); \
	fi
	@sudo rm -f $(SYSTEM_SYMLINK)
	@sudo ln -sf $(BINARY_ABS_PATH) $(SYSTEM_SYMLINK)
	@echo "$(GREEN)✓ System symlink created: $(SYSTEM_SYMLINK)$(RESET)"
	@echo ""
	@echo "$(YELLOW)You can now run: $(RESET)$(GREEN)cyx$(RESET) $(YELLOW)from anywhere!$(RESET)"
	@echo "$(YELLOW)Changes to $(BINARY_PATH) will be used immediately$(RESET)"
	@echo ""

.PHONY: path-info
path-info: ## Show symlink status and installation info
	@echo "$(CYAN)╔════════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(CYAN)║$(RESET)  $(MAGENTA)Cyx Installation Status$(RESET)                             $(CYAN)║$(RESET)"
	@echo "$(CYAN)╚════════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@echo "$(BLUE)System Symlink:$(RESET)"
	@if [ -L "$(SYSTEM_SYMLINK)" ]; then \
		echo "  $(GREEN)✓ Installed$(RESET) at $(SYSTEM_SYMLINK)"; \
		echo "  $(BLUE)→$(RESET) Points to: $$(readlink $(SYSTEM_SYMLINK))"; \
	else \
		echo "  $(YELLOW)✗ Not installed$(RESET)"; \
		echo "  Run $(GREEN)make build$(RESET) to create development symlink"; \
	fi
	@echo ""
	@echo "$(BLUE)Cargo Install:$(RESET)"
	@if command -v cyx >/dev/null 2>&1 && [ "$$(command -v cyx)" != "$(SYSTEM_SYMLINK)" ]; then \
		echo "  $(GREEN)✓ Installed$(RESET) at $$(command -v cyx)"; \
	else \
		echo "  $(YELLOW)✗ Not installed$(RESET)"; \
		echo "  Run $(GREEN)make install$(RESET) for production installation"; \
	fi
	@echo ""

.PHONY: install
install: build ## Install production binary to system (copies binary, not a symlink)
	@echo "$(CYAN)╔════════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(CYAN)║$(RESET)  $(MAGENTA)Installing Cyx to System (Production)$(RESET)               $(CYAN)║$(RESET)"
	@echo "$(CYAN)╚════════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@echo "$(YELLOW)NOTE: This creates a COPY of the binary, not a symlink.$(RESET)"
	@echo "$(YELLOW)For development, use 'make build' (creates symlink instead).$(RESET)"
	@echo ""
	@echo "$(BLUE)Installing binary to system PATH...$(RESET)"
	@$(CARGO) install --path . --force
	@echo ""
	@echo "$(BLUE)Installing ONNX Runtime library...$(RESET)"
	@INSTALL_DIR=$$(dirname $$(which cyx 2>/dev/null || echo "$$HOME/.cargo/bin/cyx")) && \
	if [ -f "$(TARGET_DIR)/libonnxruntime.1.16.0.dylib" ]; then \
		cp "$(TARGET_DIR)/libonnxruntime.1.16.0.dylib" "$$INSTALL_DIR/" && \
		cp "$(TARGET_DIR)/libonnxruntime.dylib" "$$INSTALL_DIR/" 2>/dev/null || true && \
		echo "$(GREEN)✓ Copied ONNX Runtime library to $$INSTALL_DIR$(RESET)"; \
	elif [ -f "$(TARGET_DIR)/libonnxruntime.so.1.16.0" ]; then \
		cp "$(TARGET_DIR)/libonnxruntime.so.1.16.0" "$$INSTALL_DIR/" && \
		cp "$(TARGET_DIR)/libonnxruntime.so" "$$INSTALL_DIR/" 2>/dev/null || true && \
		echo "$(GREEN)✓ Copied ONNX Runtime library to $$INSTALL_DIR$(RESET)"; \
	else \
		echo "$(YELLOW)⚠ ONNX Runtime library not found in $(TARGET_DIR)$(RESET)"; \
		echo "$(YELLOW)  You may need to manually copy the library.$(RESET)"; \
	fi
	@echo ""
	@echo "$(GREEN)✓ Cyx installed successfully!$(RESET)"
	@echo ""
	@echo "$(YELLOW)You can now run: $(RESET)$(GREEN)cyx$(RESET) $(YELLOW)from anywhere$(RESET)"
	@echo ""

.PHONY: uninstall
uninstall: ## Remove symlink and uninstall from system
	@echo "$(YELLOW)Removing system symlink and installed binary...$(RESET)"
	@sudo rm -f $(SYSTEM_SYMLINK) && echo "$(GREEN)✓ Symlink removed$(RESET)" || echo "$(YELLOW)No symlink found$(RESET)"
	@$(CARGO) uninstall cyx 2>/dev/null && echo "$(GREEN)✓ Binary uninstalled$(RESET)" || echo "$(YELLOW)No installed binary found$(RESET)"
	@echo "$(GREEN)✓ Uninstall complete$(RESET)"

.PHONY: check
check: ## Run cargo fmt and clippy with detailed reports
	@echo "$(CYAN)╔════════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(CYAN)║$(RESET)  $(MAGENTA)Code Quality Check$(RESET)                                   $(CYAN)║$(RESET)"
	@echo "$(CYAN)╚════════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@echo "$(YELLOW)┌─ Running cargo fmt check...$(RESET)"
	@echo "$(YELLOW)└────────────────────────────────────────────────────────────$(RESET)"
	@$(CARGO) fmt $(CARGO_FMT_FLAGS) -- --check && \
		echo "$(GREEN)✓ Code formatting is correct$(RESET)" || \
		(echo "$(RED)✗ Code formatting issues found!$(RESET)" && \
		 echo "$(YELLOW)  Run 'make fmt' to fix formatting$(RESET)" && \
		 echo "" && exit 1)
	@echo ""
	@echo "$(YELLOW)┌─ Running cargo clippy...$(RESET)"
	@echo "$(YELLOW)└────────────────────────────────────────────────────────────$(RESET)"
	@$(CARGO) clippy $(CARGO_CLIPPY_FLAGS) && \
		echo "$(GREEN)✓ No clippy warnings$(RESET)" || \
		(echo "$(RED)✗ Clippy found issues!$(RESET)" && \
		 echo "$(YELLOW)  Review the warnings above and fix them$(RESET)" && \
		 echo "" && exit 1)
	@echo ""
	@echo "$(CYAN)═══════════════════════════════════════════════════════════$(RESET)"
	@echo "$(GREEN)✓ All checks passed!$(RESET)"
	@echo "$(CYAN)═══════════════════════════════════════════════════════════$(RESET)"
	@echo ""

.PHONY: check-verbose
check-verbose: ## Run detailed code quality checks with full output
	@echo "$(CYAN)╔════════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(CYAN)║$(RESET)  $(MAGENTA)Detailed Code Quality Report$(RESET)                        $(CYAN)║$(RESET)"
	@echo "$(CYAN)╚════════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@echo "$(YELLOW)┌─────────────────────────────────────────────────────────┐$(RESET)"
	@echo "$(YELLOW)│$(RESET) 1/3 Checking code formatting...                      $(YELLOW)│$(RESET)"
	@echo "$(YELLOW)└─────────────────────────────────────────────────────────┘$(RESET)"
	@$(CARGO) fmt $(CARGO_FMT_FLAGS) -- --check --verbose && \
		echo "$(GREEN)✓ All files are properly formatted$(RESET)" || \
		(echo "$(RED)✗ Formatting issues detected:$(RESET)" && \
		 echo "" && \
		 echo "$(YELLOW)Files that need formatting:$(RESET)" && \
		 $(CARGO) fmt $(CARGO_FMT_FLAGS) -- --check --files-with-diff 2>&1 || true && \
		 echo "" && \
		 echo "$(YELLOW)Run 'make fmt' to auto-fix these issues$(RESET)" && \
		 echo "" && exit 1)
	@echo ""
	@echo "$(YELLOW)┌─────────────────────────────────────────────────────────┐$(RESET)"
	@echo "$(YELLOW)│$(RESET) 2/3 Running clippy (all targets)...                  $(YELLOW)│$(RESET)"
	@echo "$(YELLOW)└─────────────────────────────────────────────────────────┘$(RESET)"
	@$(CARGO) clippy $(CARGO_CLIPPY_FLAGS) -v
	@echo ""
	@echo "$(YELLOW)┌─────────────────────────────────────────────────────────┐$(RESET)"
	@echo "$(YELLOW)│$(RESET) 3/3 Checking for outdated dependencies...            $(YELLOW)│$(RESET)"
	@echo "$(YELLOW)└─────────────────────────────────────────────────────────┘$(RESET)"
	@$(CARGO) outdated || echo "$(YELLOW)Note: Install cargo-outdated for this check: cargo install cargo-outdated$(RESET)"
	@echo ""
	@echo "$(CYAN)═══════════════════════════════════════════════════════════$(RESET)"
	@echo "$(GREEN)✓ Detailed check complete!$(RESET)"
	@echo "$(CYAN)═══════════════════════════════════════════════════════════$(RESET)"
	@echo ""

.PHONY: fmt
fmt: ## Auto-fix code formatting issues
	@echo "$(BLUE)Running cargo fmt to fix formatting...$(RESET)"
	@$(CARGO) fmt $(CARGO_FMT_FLAGS)
	@echo "$(GREEN)✓ Code formatted successfully!$(RESET)"

.PHONY: fix
fix: ## Auto-fix clippy warnings where possible
	@echo "$(BLUE)Running clippy with auto-fix...$(RESET)"
	@$(CARGO) clippy --fix --allow-dirty --allow-staged $(CARGO_CLIPPY_FLAGS) || true
	@echo "$(GREEN)✓ Auto-fix complete!$(RESET)"
	@echo "$(YELLOW)Note: Some issues may require manual fixes$(RESET)"

.PHONY: test
test: ## Run all tests
	@echo "$(CYAN)╔════════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(CYAN)║$(RESET)  $(MAGENTA)Running Tests$(RESET)                                        $(CYAN)║$(RESET)"
	@echo "$(CYAN)╚════════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@$(CARGO) test --all
	@echo ""
	@echo "$(GREEN)✓ All tests passed!$(RESET)"

.PHONY: test-verbose
test-verbose: ## Run tests with verbose output
	@echo "$(CYAN)Running tests (verbose mode)...$(RESET)"
	@$(CARGO) test --all --verbose -- --nocapture

.PHONY: clean
clean: ## Clean build artifacts
	@echo "$(YELLOW)Cleaning build artifacts...$(RESET)"
	@$(CARGO) clean
	@rm -f cyx 2>/dev/null || true
	@echo "$(GREEN)✓ Clean complete!$(RESET)"
	@echo "$(YELLOW)Note: System symlink at $(SYSTEM_SYMLINK) not removed$(RESET)"
	@echo "$(YELLOW)Run 'make uninstall' to remove it$(RESET)"

.PHONY: clean-all
clean-all: clean ## Clean everything including config
	@echo "$(YELLOW)Removing config file...$(RESET)"
	@rm -f ~/.config/cyx/config.toml
	@rmdir ~/.config/cyx 2>/dev/null || true
	@echo "$(GREEN)✓ All artifacts and config removed!$(RESET)"

.PHONY: setup
setup: build ## Build and run initial setup wizard
	@echo "$(CYAN)╔════════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(CYAN)║$(RESET)  $(MAGENTA)Running Cyx Setup Wizard$(RESET)                            $(CYAN)║$(RESET)"
	@echo "$(CYAN)╚════════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@cyx setup

.PHONY: run
run: build ## Build and run cyx with interactive mode
	@cyx

.PHONY: dev
dev: build-dev ## Quick dev build and run
	@./target/debug/$(BINARY_NAME)

.PHONY: watch
watch: ## Watch for changes and rebuild (requires cargo-watch)
	@which cargo-watch > /dev/null || (echo "$(RED)Error: cargo-watch not installed$(RESET)" && echo "$(YELLOW)Install with: cargo install cargo-watch$(RESET)" && exit 1)
	@echo "$(BLUE)Watching for changes...$(RESET)"
	@cargo watch -x 'build --release' -s 'make symlink'

.PHONY: bench
bench: ## Run benchmarks
	@echo "$(BLUE)Running benchmarks...$(RESET)"
	@$(CARGO) bench

.PHONY: doc
doc: ## Generate and open documentation
	@echo "$(BLUE)Generating documentation...$(RESET)"
	@$(CARGO) doc --open --no-deps

.PHONY: audit
audit: ## Check for security vulnerabilities in dependencies
	@echo "$(BLUE)Auditing dependencies for security vulnerabilities...$(RESET)"
	@which cargo-audit > /dev/null || (echo "$(YELLOW)Installing cargo-audit...$(RESET)" && cargo install cargo-audit)
	@cargo audit

.PHONY: bloat
bloat: ## Analyze binary size (requires cargo-bloat)
	@which cargo-bloat > /dev/null || (echo "$(YELLOW)Installing cargo-bloat...$(RESET)" && cargo install cargo-bloat)
	@echo "$(BLUE)Analyzing binary size...$(RESET)"
	@cargo bloat --release -n 20

.PHONY: size
size: build ## Show binary size
	@echo "$(BLUE)Binary size:$(RESET)"
	@ls -lh $(BINARY_PATH) | awk '{print "  " $$5 " - " $$9}'
	@echo ""
	@file $(BINARY_PATH)

.PHONY: pre-commit
pre-commit: fmt check test ## Run all pre-commit checks (fmt, check, test)
	@echo ""
	@echo "$(CYAN)═══════════════════════════════════════════════════════════$(RESET)"
	@echo "$(GREEN)✓ All pre-commit checks passed!$(RESET)"
	@echo "$(CYAN)═══════════════════════════════════════════════════════════$(RESET)"
	@echo "$(GREEN)Ready to commit!$(RESET)"
	@echo ""

.PHONY: build-release-only
build-release-only: deps ## Build the release binary without symlink
	@echo "$(CYAN)╔════════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(CYAN)║$(RESET)  $(MAGENTA)Building Cyx (Release Mode)$(RESET)                          $(CYAN)║$(RESET)"
	@echo "$(CYAN)╚════════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@$(CARGO) build $(CARGO_BUILD_FLAGS)
	@echo ""
	@echo "$(GREEN)✓ Build completed successfully!$(RESET)"
	@echo ""

.PHONY: package
package: build-release-only ## Create distributable release package with bundled libraries
	@echo "$(CYAN)╔════════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(CYAN)║$(RESET)  $(MAGENTA)Creating Release Package$(RESET)                           $(CYAN)║$(RESET)"
	@echo "$(CYAN)╚════════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@bash scripts/package-release.sh

.PHONY: release
release: pre-commit build package ## Full release build with all checks and packaging
	@echo ""
	@echo "$(CYAN)╔════════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(CYAN)║$(RESET)  $(MAGENTA)Release Build Complete!$(RESET)                             $(CYAN)║$(RESET)"
	@echo "$(CYAN)╚════════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@$(MAKE) --no-print-directory size
	@echo ""
	@echo "$(GREEN)Release package created in dist/ directory$(RESET)"
	@echo "$(GREEN)Ready for distribution!$(RESET)"
	@echo ""

.PHONY: all
all: clean build test check ## Clean, build, test, and check everything
	@echo ""
	@echo "$(CYAN)═══════════════════════════════════════════════════════════$(RESET)"
	@echo "$(GREEN)✓ Full build pipeline complete!$(RESET)"
	@echo "$(CYAN)═══════════════════════════════════════════════════════════$(RESET)"
	@echo ""
