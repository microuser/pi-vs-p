.PHONY: setup build build-html debug

UNAME_S := $(shell uname -s)

setup:
	@echo "Checking for Rust (cargo) and VSCode (code) locally before using package managers..."
	@if command -v cargo >/dev/null 2>&1; then \
		echo 'cargo found locally.'; \
	else \
		if [ "$(UNAME_S)" = "Darwin" ]; then \
			echo 'Rust not found. Installing with rustup curl script...'; \
			command -v rustup || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
		else \
			echo 'Rust not found. Installing with apt-get and rustup curl script...'; \
			sudo apt-get update; \
			sudo apt-get install -y curl build-essential; \
			command -v rustup || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
		fi; \
	fi
	@if command -v code >/dev/null 2>&1; then \
		echo 'VSCode (code) found locally.'; \
	else \
		if [ "$(UNAME_S)" = "Darwin" ]; then \
			echo 'VSCode not found. Installing with Homebrew...'; \
			brew list --cask visual-studio-code || brew install --cask visual-studio-code; \
		else \
			echo 'VSCode not found. Please install manually from https://code.visualstudio.com/'; \
		fi; \
	fi
	@echo "Setup complete. Please restart your terminal if Rust was just installed."

build:
	@echo "Building Rust project in debug mode (using cargo if available)..."
	@if command -v cargo >/dev/null 2>&1; then \
		cargo build; \
	else \
		echo 'cargo not found. Please run make setup.'; \
	fi

build-html:
	@echo "Opening index.html in your default browser..."
ifeq ($(UNAME_S),Darwin)
	open index.html
else
	xdg-open index.html
endif

debug:
	@echo "Launching VSCode for debugging..."
	code .
	@echo "Set breakpoints in VSCode and start debugging from the Run panel."
