#!/usr/bin/env bash
# Packages from grapheneos.org/build ("Set up Debian GNU/Linux 12 (bookworm)") plus the
# verification dependencies it lists. Run as root on a fresh Debian 12 machine.
set -euo pipefail
apt-get update
apt-get install -y repo yarnpkg zip rsync git gnupg openssh-client python3 curl
grep -q '/usr/sbin' /etc/profile.d/grapheneos-path.sh 2>/dev/null || \
  echo 'export PATH=$PATH:/sbin:/usr/sbin:/usr/local/sbin' > /etc/profile.d/grapheneos-path.sh
echo "provisioned"
