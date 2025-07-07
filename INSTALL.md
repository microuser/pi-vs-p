# Installation & Usage Guide

This project uses a Makefile to simplify setup, building, and development workflows.

## Prerequisites

- Rust toolchain (`rustup`)
- Make utility (`make`)
- For Mac: Homebrew (`brew`)
- For Linux: `apt-get`
- VSCode (recommended for debugging)

## Setup Instructions

1. **Clone the repository:**
   ```sh
   git clone <your-repo-url>
   cd <your-repo>
   ```

2. **Install prerequisites and set up the dev environment:**
   ```sh
   make setup
   ```
   This will install Rust and VSCode (if missing), using Homebrew on Mac or apt-get on Linux. Manual steps will be echoed if required.

3. **Build the Rust project:**
   ```sh
   make build
   ```
   This compiles the Rust code in debug mode.

4. **Open the HTML frontend:**
   ```sh
   make build-html
   ```
   This opens `index.html` in your default browser.

5. **Debug with VSCode:**
   ```sh
   make debug
   ```
   This opens VSCode in the project folder. Set breakpoints and use the Run panel for debugging.

---

## Additional Notes

- If you install Rust for the first time, restart your terminal session before running `make build`.
- For 3D inspection or advanced debugging, ensure you have the necessary VSCode extensions (e.g., Rust Analyzer).
- If you encounter permission issues, try running `make` commands with `sudo` (Linux only, not recommended unless necessary).
