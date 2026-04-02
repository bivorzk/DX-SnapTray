#!/bin/sh
set -e
PUBLIC_SRC="target/dx/snap-tray-auth/release/web/public"
PUBLIC_DST="target/release/public"
if [ ! -d "$PUBLIC_DST" ]; then
    cp -r "$PUBLIC_SRC" "$PUBLIC_DST"
fi
exec ./target/release/snap-tray-auth
