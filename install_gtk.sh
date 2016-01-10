#!/bin/sh

set -x
set -e

WD="$PWD"
cd "$HOME"
curl -LO "https://github.com/gkoz/gtk-bootstrap/releases/download/gtk-3.18.1-2/deps.txz"
tar xf deps.txz
cd "$WD"
