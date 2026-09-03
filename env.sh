# Shared LLVM 18 setup for building Trell with inkwell / llvm-sys 181.
# Source from bash or zsh:  source ./env.sh

if [ -z "${LLVM_SYS_181_PREFIX:-}" ]; then
  if command -v brew >/dev/null 2>&1; then
    # brew --prefix succeeds even when the formula is not installed;
    # require llvm-config so a missing keg does not win over later probes.
    brew_llvm="$(brew --prefix llvm@18 2>/dev/null || true)"
    if [ -n "$brew_llvm" ] && [ -x "$brew_llvm/bin/llvm-config" ]; then
      LLVM_SYS_181_PREFIX="$brew_llvm"
    fi
  fi

  if [ -z "${LLVM_SYS_181_PREFIX:-}" ] && [ -x /usr/lib/llvm-18/bin/llvm-config ]; then
    LLVM_SYS_181_PREFIX=/usr/lib/llvm-18
  elif [ -z "${LLVM_SYS_181_PREFIX:-}" ] && command -v llvm-config-18 >/dev/null 2>&1; then
    LLVM_SYS_181_PREFIX="$(llvm-config-18 --prefix)"
  elif [ -z "${LLVM_SYS_181_PREFIX:-}" ] && command -v llvm-config >/dev/null 2>&1; then
    LLVM_SYS_181_PREFIX="$(llvm-config --prefix)"
  fi
fi

if [ -z "${LLVM_SYS_181_PREFIX:-}" ]; then
  echo "Trell needs LLVM 18. Install llvm@18 (Homebrew) or llvm-18-dev (apt)." >&2
  return 1 2>/dev/null || exit 1
fi

export LLVM_SYS_181_PREFIX
export PATH="$LLVM_SYS_181_PREFIX/bin:$PATH"

# Linux only: llvm-sys links with `cc` and requests -lstdc++. When `cc` is
# clang, it does not search GCC's private libdir for the unversioned
# libstdc++.so. Do not touch LIBRARY_PATH on macOS — Apple clang uses libc++
# and a Homebrew g++ path would break the Xcode link.
if [ "$(uname -s)" = Linux ]; then
  if [ -z "${LIBRARY_PATH:-}" ] || ! echo "${LIBRARY_PATH:-}" | grep -q libstdc++; then
    if command -v g++ >/dev/null 2>&1; then
      gcc_stdlib="$(dirname "$(g++ -print-file-name=libstdc++.so 2>/dev/null)")"
      if [ -n "$gcc_stdlib" ] && [ -e "$gcc_stdlib/libstdc++.so" ]; then
        export LIBRARY_PATH="${gcc_stdlib}${LIBRARY_PATH:+:$LIBRARY_PATH}"
      fi
    fi
  fi
fi
