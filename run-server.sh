#!/bin/sh
set -e
export IP=0.0.0.0
cd target/dx/snap-tray-auth/release/web
ls -la
chmod +x snap-tray-auth
exec ./snap-tray-auth
