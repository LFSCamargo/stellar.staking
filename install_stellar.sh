# checks if stellar is installed using which
if ! which stellar >/dev/null; then
  # installs stellar
  cargo install stellar --locked
fi
