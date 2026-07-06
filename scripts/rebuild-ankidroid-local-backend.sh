#!/usr/bin/env bash
set -euo pipefail

ROOT="${ROOT:-"$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"}"
ANDROID_HOME="${ANDROID_HOME:-${ANDROID_SDK_ROOT:-"$HOME/Library/Android/sdk"}}"

# Pick a JDK the Android backend accepts (17 / 21 / 25). A pre-set JAVA_HOME on an
# incompatible version (notably JDK 20) otherwise makes Gradle abort with
# "Incompatible major version detected". Homebrew's openjdk@N is keg-only and is
# NOT registered with /usr/libexec/java_home, so probe brew as well.
_java_major() { { "$1/bin/java" -version 2>&1 || true; } | sed -n 's/.*version "\([0-9][0-9]*\).*/\1/p' | head -1; }
_java_ok() { [[ -x "$1/bin/java" ]] && case "$(_java_major "$1")" in 17|21|25) true ;; *) false ;; esac; }

if [[ -z "${JAVA_HOME:-}" ]] || ! _java_ok "$JAVA_HOME"; then
  for _cand in \
    "$(brew --prefix openjdk@21 2>/dev/null || true)/libexec/openjdk.jdk/Contents/Home" \
    "$(brew --prefix openjdk@17 2>/dev/null || true)/libexec/openjdk.jdk/Contents/Home" \
    "$(/usr/libexec/java_home -v 21 2>/dev/null || true)" \
    "$(/usr/libexec/java_home -v 17 2>/dev/null || true)" \
    "$(/usr/libexec/java_home -v 25 2>/dev/null || true)"; do
    if _java_ok "$_cand"; then JAVA_HOME="$_cand"; break; fi
  done
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

