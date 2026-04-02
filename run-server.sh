#!/bin/sh
set -e
export IP=0.0.0.0
cd target/dx/snap-tray-auth/release/web
exec ./snap-tray-auth
