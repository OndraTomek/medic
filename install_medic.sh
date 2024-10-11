#!/bin/bash

# Medic CLI installation script
REPO_URL="https://github.com/OndraTomek/medic.git"
INSTALL_DIR="/usr/local/bin"
APP_NAME="medic"

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Step 1: Check if Git is installed
if ! command_exists git; then
    echo "Git is not installed. Installing Git..."
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        sudo apt update
        sudo apt install git -y
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # For macOS using Homebrew
        if ! command_exists brew; then
            echo "Homebrew not found. Installing Homebrew first..."
            /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        fi
        brew install git
    else
        echo "Unsupported OS. Please install Git manually."
        exit 1
    fi
fi

# Step 2: Check if Cargo (Rust) is installed
if ! command_exists cargo; then
    echo "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Step 3: Clone the Medic CLI repository
echo "Cloning Medic CLI repository..."
if [ -d "medic" ]; then
    echo "Removing existing medic directory."
    rm -rf medic
fi

git clone "$REPO_URL" || { echo "Failed to clone repository."; exit 1; }
cd medic || { echo "Failed to enter directory."; exit 1; }

# Step 4: Build the project using Cargo
echo "Building the project using Cargo..."
cargo build --release || { echo "Build failed. Please check for errors."; exit 1; }

# Step 5: Install the binary to /usr/local/bin or a suitable location
echo "Installing Medic to $INSTALL_DIR..."
sudo cp "target/release/$APP_NAME" "$INSTALL_DIR/" || { echo "Failed to copy binary to $INSTALL_DIR."; exit 1; }

echo "Cleaning up installation files..."
cd ..
rm -rf medic

# Step 6: Verify the installation
if command_exists "$APP_NAME"; then
    echo "Installation successful! You can now use the Medic with the following command:"
    echo "$APP_NAME --help"
else
    echo "Error: The Medic CLI was not installed properly. Please check your system's PATH."
    exit 1
fi