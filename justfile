set shell := ["bash", "-cu"]

default:
    just --list

desktop:
    cd anki && just run

desktop-check:
    cd anki && just check

backend:
    cd Anki-Android-Backend && ./build.sh

android-install:
    cd Anki-Android && ./gradlew :AnkiDroid:installPlayDebug

android-check:
    cd Anki-Android && ./gradlew test

rebuild-local-backend:
    ./scripts/rebuild-ankidroid-local-backend.sh

