#!/bin/bash
set -e

echo "[nv-osmium] Booting container..."
cd /runner

if [ ! -d ".runner" ]; then
  echo "[nv-osmium] Configuring runner..."
  ./config.sh \
    --url https://github.com/GhostKellz/nvcontrol \
    --token AORJNAY7IXHL3TXTMYGUGNDJFYGEA  \
    --name ck-arch \
    --labels self-hosted \
    --unattended
else
  echo "[nv-osmium] Already configured."
fi

echo "[nv-osmium] Starting runner..."
exec ./run.sh

