if [[ "$PATH" == *"$HOME/.cargo/bin"* ]]; then
  echo "Installing in $HOME/.cargo/bin..."
  curl -LSfs https://rossmacarthur.github.io/install/crate.sh | bash -s -- --repo "techstackbrew/tsb" --to ~/.cargo/bin
elif [[ "$PATH" == *"/opt/homebrew/bin"* ]]; then
  echo "Installing in /opt/homebrew/bin..."
  curl -LSfs https://rossmacarthur.github.io/install/crate.sh | bash -s -- --repo "techstackbrew/tsb" --to /opt/homebrew/bin
else
  echo "Installing in /usr/local/bin (if this fails you may need to re-run with sudo)..."
  curl -LSfs https://rossmacarthur.github.io/install/crate.sh | bash -s -- --repo "techstackbrew/tsb" --to /usr/local/bin
fi
