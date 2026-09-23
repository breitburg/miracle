# Miracle

A shared Rust core with native UI shells: SwiftUI on macOS, GTK 4 +
libadwaita on GNOME. All business logic lives in Rust; each shell only renders
state and sends user actions. Bazel builds everything.

## Layout

```
core/                        miracle_core: pure Rust domain (State, Action, reduce, Store)
ffi/                         miracle_ffi: UniFFI facade over core, for non-Rust shells
platforms/
  apple/MiracleCore/         Swift bindings generated from //ffi (shared by Apple shells)
  apple/macos/               SwiftUI shell
  linux/gnome/               GTK 4 + libadwaita shell in Rust (uses //core directly)
build/app.bzl                APP_ID, APP_NAME, VERSION shared by every shell
third_party/system_libs/     host C libraries (GTK, libadwaita) found with pkg-config
tools/uniffi-bindgen-swift/  hermetic Swift bindings generator
```

Dependencies only point one way, enforced with Bazel `visibility`:

```
core  <-  ffi  <-  platforms/apple/*     (non-Rust shells)
core  <-  platforms/linux/gnome          (Rust shells)
```

No shell depends on another shell. Every platform target sets
`target_compatible_with`, so `bazel build //...` and `bazel test //...` work on
any host and skip the targets of other platforms.

## How it fits together

The core uses a unidirectional data flow:

```
shell --Action--> Store::dispatch --> reduce(State, Action) --> State --> shell renders
```

- **macOS.** `//ffi:miracle_ffi` is a static library. `//platforms/apple/MiracleCore`
  runs `uniffi-bindgen-swift` in library mode on it and compiles the generated
  Swift API. `AppModel` (an `@Observable`) holds the `Store` and republishes
  `State` to SwiftUI.
- **GNOME.** `//platforms/linux/gnome:app` (the `miracle` binary) links `//core` directly. `AppModel` is
  a GObject that republishes `State` as properties; the Blueprint UI binds to
  them. The UI and CSS are compiled into a GResource embedded in the binary.

## Setup

Use [Bazelisk](https://github.com/bazelbuild/bazelisk). It reads
`.bazelversion` and runs the pinned Bazel version. The commands below call it
`bazel`; if only `bazelisk` is installed, link it:

```sh
sudo ln -s "$(command -v bazelisk)" /usr/local/bin/bazel
```

For the GNOME shell, install the development packages for GTK and libadwaita:

```sh
sudo dnf install gtk4-devel libadwaita-devel blueprint-compiler glib2-devel pkgconf   # Fedora
sudo apt install libgtk-4-dev libadwaita-1-dev blueprint-compiler libglib2.0-dev-bin pkg-config  # Debian/Ubuntu
```

The macOS shell needs Xcode.

## Commands

```sh
bazel run //platforms/linux/gnome:app      # build and launch the GNOME app (Linux)
bazel run //platforms/apple/macos:app      # build and launch the macOS app (macOS)
bazel test //...                           # Rust unit tests
bazel run @rules_rust//tools/rust_analyzer:gen_rust_project  # rust-project.json for rust-analyzer
```

## Rust dependencies

Crates are declared with `crate.spec` in `MODULE.bazel`. There is no Cargo
workspace; `ffi/Cargo.toml` is a stub that UniFFI needs for the crate name.
After changing a spec, re-pin the lockfiles in `third_party/`:

```sh
CARGO_BAZEL_REPIN=1 bazel build //...
```

## System libraries (GNOME)

GTK and libadwaita come from the system: the host during development, the
GNOME SDK inside Flatpak. The `system_libs` module extension runs `pkg-config`
once at fetch time and exposes each library as a `cc_library`.

The build scripts of the gtk-rs `-sys` crates only run `pkg-config`, so
`MODULE.bazel` turns them off (`gen_build_script = "off"`) and links the
`//third_party/system_libs` targets instead. If a new `-sys` crate appears
after a re-pin, add it to that list.

After a system upgrade, refresh the libraries:

```sh
bazel fetch --configure
```

## Adding a platform

Shells are grouped by OS family: `platforms/<family>/<shell>/`, for example
`platforms/apple/macos` or `platforms/linux/gnome`. Code that several shells of
one family share (such as `platforms/apple/MiracleCore`) lives next to them.

- **Rust shell** (for example a TUI or a KDE shell with cxx-qt): create
  `platforms/<family>/<name>/`, depend on `//core:miracle_core`, and add the package
  to the `visibility` of `//core:miracle_core`.
- **Other language** (for example Kotlin on Android, C# on Windows): create a
  bindings package next to the shell that runs the matching `uniffi-bindgen`
  on `//ffi:miracle_ffi`, the same way as `platforms/apple/MiracleCore`.
- In both cases, set `target_compatible_with` on every target, read the
  app identity from `//build:app.bzl`, and name the runnable target `:app`
  (an `alias` if the binary needs a different name).

When the core needs a platform capability (storage, network, clock), declare
a trait in the core. Rust shells implement it directly; FFI shells implement
it through a `#[uniffi::export(with_foreign)]` trait in `ffi`.
