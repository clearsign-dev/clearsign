# Fetch the pinned Linux tarball and verify it two ways before anything is built:
#   1. SHA-256 against config/pins.env
#   2. the kernel.org signature, against the key stored in config/keys
# Sourced by the image builds. Sets nothing global except the files in $1.
verify_kernel_tarball() {
  local src="$1" root="$2" kv="$GUEST_KERNEL_VERSION"
  mkdir -p "$src"
  [[ -f "$src/linux-$kv.tar.xz" ]] || curl -fsSL -o "$src/linux-$kv.tar.xz" "$GUEST_KERNEL_URL/linux-$kv.tar.xz"
  curl -fsSL -o "$src/linux-$kv.tar.sign" "$GUEST_KERNEL_URL/linux-$kv.tar.sign"
  [[ "$(shasum -a 256 "$src/linux-$kv.tar.xz" | cut -d' ' -f1)" == "$GUEST_KERNEL_TARBALL_SHA256" ]] \
    || { echo "kernel tarball SHA-256 mismatch" >&2; return 1; }
  local gnupghome; gnupghome="$(mktemp -d)"
  GNUPGHOME="$gnupghome" gpg -q --import "$root/config/keys/kernel-gregkh-$GUEST_KERNEL_SIGNING_KEY_FPR.asc"
  if ! xz -dc "$src/linux-$kv.tar.xz" \
      | GNUPGHOME="$gnupghome" gpg --status-fd 1 --verify "$src/linux-$kv.tar.sign" - 2>/dev/null \
      | grep -q "VALIDSIG $GUEST_KERNEL_SIGNING_KEY_FPR"; then
    rm -rf "$gnupghome"
    echo "kernel signature not from the pinned key" >&2
    return 1
  fi
  rm -rf "$gnupghome"
  echo "kernel $kv: tarball hash and kernel.org signature verified"
}
