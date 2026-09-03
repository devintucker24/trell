#!/bin/sh
# Turn out.ll into ./trell-program for this host.
#
# macOS must not use Homebrew clang to link: after env.sh, `clang` on PATH is
# llvm@18, which does not see the Apple SDK. Keep the original two-step:
# Homebrew llc → object, then xcrun clang (Xcode) to link.
set -e

if [ ! -f out.ll ]; then
  echo "out.ll not found; compile a .trell file first." >&2
  exit 1
fi

case "$(uname -s)" in
  Darwin)
    if ! command -v llc >/dev/null 2>&1; then
      echo "llc not found. source ./env.zsh so Homebrew llvm@18 is on PATH." >&2
      exit 1
    fi
    if ! command -v xcrun >/dev/null 2>&1; then
      echo "xcrun not found. Install the Xcode command-line tools." >&2
      exit 1
    fi

    # Same llc invocation as the original Mac workflow on Apple Silicon.
    if [ "$(uname -m)" = arm64 ]; then
      llc -mtriple=arm64-apple-macosx15.2 -filetype=obj out.ll -o trell-program.o
    else
      llc -filetype=obj out.ll -o trell-program.o
    fi
    xcrun clang trell-program.o -o trell-program
    ;;

  *)
    if command -v clang >/dev/null 2>&1; then
      clang out.ll -o trell-program
    elif command -v llc >/dev/null 2>&1; then
      llc -filetype=obj out.ll -o trell-program.o
      cc trell-program.o -o trell-program
    else
      echo "Need clang or llc to produce a native binary from out.ll" >&2
      exit 1
    fi
    ;;
esac
