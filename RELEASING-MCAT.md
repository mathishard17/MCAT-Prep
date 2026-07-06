# Releasing MCAT Prep builds (Android APK + macOS DMG)

How the `mvp-preview` GitHub release artifacts are produced and uploaded
(<https://github.com/mathishard17/MCAT-Prep/releases/tag/mvp-preview>). This is
the method that works **without Xcode / Briefcase's dmg packaging** — the part
that keeps failing interactively.

## Prerequisites

- **JDK 21.** The Android backend builds only with JVM **17 / 21 / 25** — JDK 20
  fails ("Incompatible major version detected: '20'"). JDK 21 is installed via
  `brew install openjdk@21`, but it's keg-only, so it is **not** the default
  `java`. Point `JAVA_HOME` at it explicitly:

  ```sh
  export JAVA_HOME="$(brew --prefix openjdk@21)/libexec/openjdk.jdk/Contents/Home"
  export PATH="$JAVA_HOME/bin:$PATH"
  java -version   # must print 21.x
  ```

- Android SDK at `~/Library/Android/sdk`.
- `gh` uses the GitHub token stored in the macOS keychain:

  ```sh
  export GH_TOKEN=$(printf 'protocol=https\nhost=github.com\n\n' | git credential fill | sed -n 's/^password=//p')
  ```

## Android APK (arm64 debug)

```sh
export JAVA_HOME="$(brew --prefix openjdk@21)/libexec/openjdk.jdk/Contents/Home"
export PATH="$JAVA_HOME/bin:$PATH"

cd Anki-Android-Backend && ./build.sh          # builds the local Rust backend (rsdroid JNI)
cd ../Anki-Android && ./gradlew :AnkiDroid:assemblePlayDebug
```

APK output:
`Anki-Android/AnkiDroid/build/outputs/apk/play/debug/AnkiDroid-play-arm64-v8a-debug.apk`
→ copy to `dist/AnkiDroid-MCAT-arm64-debug.apk`.

(`just rebuild-local-backend` runs `build.sh` + `installPlayDebug`, which also
needs a connected device/emulator. `assemblePlayDebug` just builds the APK.)

## macOS DMG (Apple silicon) — the reliable way

`just desktop-installer` (= `cd anki && RELEASE=2 ./ninja installer`) compiles and
**ad-hoc-signs** `Anki.app`, but its final Briefcase **`package` (dmg) step fails
non-interactively** — it tries to pop up an install dialog (the "Xcode weird
stuff"). That failure happens *after* the app bundle is built and signed, so the
app is complete. Wrap it into a dmg with `hdiutil` yourself:

```sh
just desktop-installer   # fine if it ends on the Briefcase "package" error — the .app is already built

APP="anki/out/installer/build/anki/macos/app/Anki.app"   # codesign -dv shows "Signature=adhoc"
STAGE="$(mktemp -d)"
ditto "$APP" "$STAGE/Anki.app"            # ditto preserves the ad-hoc code signature
ln -s /Applications "$STAGE/Applications" # drag-to-install shortcut
rm -f dist/Anki-MCAT-mac-arm64.dmg
hdiutil create -volname "Anki MCAT" -srcfolder "$STAGE" -ov -format UDZO dist/Anki-MCAT-mac-arm64.dmg
hdiutil verify dist/Anki-MCAT-mac-arm64.dmg
rm -rf "$STAGE"
```

Only `hdiutil` / `ditto` / `codesign` (base macOS tools) are used — no Xcode, no
Briefcase packaging.

## Upload to the release (replace old assets)

```sh
gh release upload mvp-preview dist/Anki-MCAT-mac-arm64.dmg        -R mathishard17/MCAT-Prep --clobber
gh release upload mvp-preview dist/AnkiDroid-MCAT-arm64-debug.apk -R mathishard17/MCAT-Prep --clobber
```

## Notes

- The app is **ad-hoc signed, not notarized**, so downloaders must clear the
  quarantine flag once: `xattr -dr com.apple.quarantine /Applications/Anki.app`
  (or System Settings → Privacy & Security → Open Anyway).
- Both builds embed the lesson `.md` files via `include_str!`, so lessons +
  inline diagrams ship inside the binaries (no external files needed).
