#!/usr/bin/env bash
# Run against installed binaries inside a desktop session (X11 or native Wayland).
# Overrides: DESIGNCRAFT_BIN, DESIGNCRAFT_CLI, SMOKE_OUT, DESIGNCRAFT_SMOKE_PORT.
set -euo pipefail
APP="${DESIGNCRAFT_BIN:-designcraft}"
CLI="${DESIGNCRAFT_CLI:-designcraft-cli}"
OUT="${SMOKE_OUT:-$(mktemp -d -t designcraft-smoke.XXXXXX)}"
PORT="${DESIGNCRAFT_SMOKE_PORT:-17980}"
mkdir -p "$OUT"
OUT="$(cd "$OUT" && pwd)"
for file in input.designcraft output.png output.pdf original.json original.jsonl edited.designcraft reopened.json reopened.jsonl desktop.png desktop.log control.jsonl; do
  if [ -e "$OUT/$file" ] || [ -L "$OUT/$file" ]; then
    echo "error: smoke output already exists: $OUT/$file (use a fresh SMOKE_OUT directory)" >&2
    exit 1
  fi
done
command -v jq >/dev/null || { echo "error: jq is required" >&2; exit 1; }
"$APP" --version
"$CLI" --version
"$CLI" run --cmd 'frame.create={"rect":[36,36,300,200],"content":"text","text":"DesignCraft Ubuntu smoke"}' \
  --export "$OUT/input.designcraft" --export "$OUT/output.png" --export "$OUT/output.pdf" \
  --cmd document.inspect >"$OUT/original.jsonl"
jq -s 'last' "$OUT/original.jsonl" >"$OUT/original.json"
test -s "$OUT/output.png"
test -s "$OUT/output.pdf"

# Keep a developer's normal preferences and recovery documents out of this test session.
DESIGNCRAFT_NO_PREFS=1 XDG_CONFIG_HOME="$OUT/config" XDG_DATA_HOME="$OUT/data" \
  "$APP" --control "$PORT" "$OUT/input.designcraft" >"$OUT/desktop.log" 2>&1 &
APP_PID=$!
cleanup() {
  kill "$APP_PID" 2>/dev/null || true
  wait "$APP_PID" 2>/dev/null || true
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
CONNECTED=0
for ((attempt = 0; attempt < 60; attempt++)); do
  if ! kill -0 "$APP_PID" 2>/dev/null; then cat "$OUT/desktop.log" >&2; exit 1; fi
  # Do not drive another app that already owns the requested port.
  if grep -Fq "designcraft: control server listening on 127.0.0.1:$PORT" "$OUT/desktop.log" \
    && { exec 3<>"/dev/tcp/127.0.0.1/$PORT"; } 2>/dev/null; then CONNECTED=1; break; fi
  sleep 0.5
done
if [ "$CONNECTED" != 1 ]; then cat "$OUT/desktop.log" >&2; echo "desktop did not start" >&2; exit 1; fi

rpc() {
  local reply
  printf '%s\n' "$1" >&3
  if ! IFS= read -r -t 30 reply <&3; then
    cat "$OUT/desktop.log" >&2
    echo "desktop control request timed out" >&2
    return 1
  fi
  printf '%s\n' "$reply" | tee -a "$OUT/control.jsonl" | jq -e '.ok == true' >/dev/null
}
rpc '{"id":1,"method":"ui.inspect","params":{}}'
jq -e 'select(.id == 1) | .result.documents | length == 1' "$OUT/control.jsonl" >/dev/null
rpc '{"id":2,"method":"engine.execute","params":{"command":"frame.create","params":{"rect":[36,240,300,340],"content":"text","text":"Ubuntu saved edit"}}}'
rpc "$(jq -nc --arg path "$OUT/edited.designcraft" '{id:3,method:"engine.execute",params:{command:"file.saveAs",params:{path:$path}}}')"
test -s "$OUT/edited.designcraft"
rpc "$(jq -nc --arg path "$OUT/desktop.png" '{id:4,method:"ui.screenshot",params:{path:$path}}')"
test -s "$OUT/desktop.png"
"$CLI" run --in "$OUT/edited.designcraft" --cmd document.inspect >"$OUT/reopened.jsonl"
jq -s 'last' "$OUT/reopened.jsonl" >"$OUT/reopened.json"
jq -e --slurpfile original "$OUT/original.json" \
  '([.spreads[].items[]] | length) == (([$original[0].spreads[].items[]] | length) + 1) and
   any(.stories[]; .preview == "Ubuntu saved edit")' "$OUT/reopened.json" >/dev/null
echo "DesignCraft desktop, edit/save/reopen and CLI smoke passed: $OUT"
