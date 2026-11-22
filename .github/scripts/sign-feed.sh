#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
# Signs a channel feed with the update key (UPDATE_SIGNING_KEY, an Ed25519
# PEM), writing <feed>.sig, and checks the signature against the public key
# the apps carry (.github/release-key.pub.pem, desktop/app/src/release_key.rs).
#
#   sign-feed.sh <feed.json> <release-key.pub.pem>
set -euo pipefail

feed="${1:?usage: sign-feed.sh <feed.json> <release-key.pub.pem>}"
pub="${2:?usage: sign-feed.sh <feed.json> <release-key.pub.pem>}"
if [[ -z "${UPDATE_SIGNING_KEY:-}" ]]; then
    echo "::error::the UPDATE_SIGNING_KEY secret isn't set" >&2
    exit 1
fi
key="$(mktemp)"
trap 'rm -f -- "${key}"' EXIT
chmod 600 "${key}"
printf '%s\n' "${UPDATE_SIGNING_KEY}" > "${key}"
openssl pkeyutl -sign -inkey "${key}" -rawin -in "${feed}" -out "${feed}.sig"
openssl pkeyutl -verify -pubin -inkey "${pub}" -rawin -in "${feed}" -sigfile "${feed}.sig"
