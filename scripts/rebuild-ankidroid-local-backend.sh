#!/usr/bin/env bash
set -euo pipefail

ROOT="${ROOT:-"$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"}"
ANDROID_HOME="${ANDROID_HOME:-${ANDROID_SDK_ROOT:-"$HOME/Library/Android/sdk"}}"

if [[ -z "${JAVA_HOME:-}" ]]; then
  if command -v /usr/libexec/java_home >/dev/null 2>&1; then
    JAVA_HOME="$(/usr/libexec/java_home -v 21 2>/dev/null || true)"
  fi
fi
JAVA_HOME="${JAVA_HOME:-}"

export ANDROID_HOME
export JAVA_HOME
export PATH="$JAVA_HOME/bin:$ANDROID_HOME/platform-tools:$PATH"

if [[ ! -x "$JAVA_HOME/bin/java" ]]; then
  echo "Java 21 not found. Set JAVA_HOME to a Java 21 installation."
  echo "Install with: brew install openjdk@21"
  exit 1
fi

if [[ ! -d "$ANDROID_HOME" ]]; then
  echo "Android SDK not found at: $ANDROID_HOME"
  exit 1
fi

if [[ ! -f "$ROOT/anki/rslib/Cargo.toml" ]]; then
  echo "Missing Anki checkout at: $ROOT/anki"
  exit 1
fi

if [[ ! -d "$ROOT/Anki-Android-Backend" ]]; then
  echo "Missing backend checkout at: $ROOT/Anki-Android-Backend"
  exit 1
fi

if [[ ! -d "$ROOT/Anki-Android" ]]; then
  echo "Missing Android checkout at: $ROOT/Anki-Android"
  exit 1
fi

if [[ ! -e "$ROOT/Anki-Android-Backend/anki" ]]; then
  echo "Missing backend Anki dependency at: $ROOT/Anki-Android-Backend/anki"
  echo "After migration, point it at the sibling checkout with:"
  echo "  cd \"$ROOT/Anki-Android-Backend\" && ln -s ../anki anki"
  exit 1
fi

echo "Using Java:"
java -version

echo
echo "Building local rsdroid backend..."
cd "$ROOT/Anki-Android-Backend"
./build.sh

echo
echo "Installing AnkiDroid with local backend..."
cd "$ROOT/Anki-Android"
./gradlew :AnkiDroid:installPlayDebug

echo
echo "Done."

