.PHONY: setup build build-html debug

setup:
	@echo "Checking for Rust (cargo), VSCode (code), and Bevy dependency..."
	@if command -v cargo >/dev/null 2>&1; then \
		echo 'cargo found locally.'; \
	else \
		echo 'Rust not found. Installing with rustup curl script...'; \
		command -v rustup || curl --proto '=https' --tlsv1.2 -sSf https://sh.rust-lang.org | sh -s -- -y; \
	fi
	@if command -v code >/dev/null 2>&1; then \
		echo 'VSCode (code) found locally.'; \
		@echo "Installing VSCode Rust debug and TOML extensions..."; \

	elif [ -d "/Applications/Visual Studio Code.app" ]; then \
		echo 'VSCode app found in /Applications, but the code command is not available in your shell.'; \
		echo 'Recommended: Open VSCode, press Cmd+Shift+P, type: Shell Command: Install '\''code'\'' command in PATH, and select it.'; \
		echo 'Alternatively, you can run (with sudo if needed):'; \
		echo '  sudo ln -s "/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code" /usr/local/bin/code'; \
	else \
		echo 'VSCode not found. Installing with Homebrew...'; \
		brew list --cask visual-studio-code || brew install --cask visual-studio-code; \
	fi

	echo 'install these extensions: '\
	echo code --install-extension vadimcn.vscode-lldb \
	echo code --install-extension tamasfe.even-better-toml \


	@echo "Ensuring Bevy is in Cargo.toml..."
	@grep -q 'bevy' Cargo.toml || echo 'bevy = "0.13"' >> Cargo.toml
	@echo "Setup complete. Ready for Bevy development!"

build:
	@echo "Building Rust project in debug mode (using cargo if available)..."
	@if command -v cargo >/dev/null 2>&1; then \
		cargo build; \
	else \
		echo 'cargo not found. Please run make setup.'; \
	fi

# Build the main application (default)
build:
	@echo "Building main Rust app..."
	cargo build

# Run the TinyText 3D Scoreboard demo binary
run-scoreboard:
	@echo "Running TinyText 3D Scoreboard Demo..."
	cargo run --bin scoreboard

build-html:
	@echo "Opening index.html in your default browser..."
	open index.html

run:
	cargo run

debug:
	@echo "Building in debug mode for Bevy development..."
	cargo build
	@echo "Launching VSCode for debugging..."
	code .
	@echo "Set breakpoints in src/main.rs and start debugging from the Run panel."

# Open the project folder in Finder (macOS)
edit:
	open .
# If you use VS Code, you can use:
# edit:
	code .
