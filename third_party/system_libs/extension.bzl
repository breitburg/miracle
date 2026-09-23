"""Exposes host C libraries, found with pkg-config, as Bazel cc_library targets.

GTK and libadwaita come from the system (the host during development, the
GNOME SDK inside Flatpak), not from the BCR. This extension runs pkg-config
once at fetch time, so no build action ever calls it.

Usage in MODULE.bazel:

    system_libs = use_extension("//third_party/system_libs:extension.bzl", "system_libs")
    system_libs.library(name = "gtk4", pkg_config = "gtk4")
    use_repo(system_libs, "system_libs")

Then depend on `@system_libs//:gtk4`. Refresh after a system upgrade with
`bazel fetch --configure`.
"""

_INSTALL_HINT = """\
pkg-config could not find '{pkg}':
{err}
Install the development package, for example on Fedora:
  sudo dnf install gtk4-devel libadwaita-devel
or on Debian/Ubuntu:
  sudo apt install libgtk-4-dev libadwaita-1-dev
"""

def _pkg_config(rctx, pkg, flag):
    result = rctx.execute(["pkg-config", flag, pkg])
    if result.return_code != 0:
        fail(_INSTALL_HINT.format(pkg = pkg, err = result.stderr.strip()))
    return result.stdout.strip().split(" ") if result.stdout.strip() else []

def _system_libs_repo_impl(rctx):
    if not rctx.which("pkg-config"):
        fail("pkg-config is not on PATH. Install it (Fedora: pkgconf, Debian: pkg-config).")

    targets = []
    for name, pkg in rctx.attr.libraries.items():
        version = _pkg_config(rctx, pkg, "--modversion")
        targets.append("""\
# {pkg} {version}
cc_library(
    name = {name},
    linkopts = {linkopts},
    copts = {copts},
    target_compatible_with = ["@platforms//os:linux"],
    visibility = ["//visibility:public"],
)
""".format(
            pkg = pkg,
            version = " ".join(version),
            name = repr(name),
            linkopts = repr(_pkg_config(rctx, pkg, "--libs")),
            copts = repr(_pkg_config(rctx, pkg, "--cflags")),
        ))

    rctx.file("BUILD.bazel", 'load("@rules_cc//cc:cc_library.bzl", "cc_library")\n\n' + "\n".join(targets))

_system_libs_repo = repository_rule(
    implementation = _system_libs_repo_impl,
    attrs = {"libraries": attr.string_dict()},
    configure = True,
    environ = ["PATH", "PKG_CONFIG_PATH", "PKG_CONFIG_LIBDIR", "PKG_CONFIG_SYSROOT_DIR"],
    local = True,
)

def _empty_repo_impl(rctx):
    rctx.file("BUILD.bazel", "")

_empty_repo = repository_rule(implementation = _empty_repo_impl)

def _system_libs_impl(mctx):
    libraries = {}
    for mod in mctx.modules:
        for lib in mod.tags.library:
            libraries[lib.name] = lib.pkg_config

    # pkg-config and these libraries only exist on Linux hosts. Elsewhere,
    # expose an empty repo: the targets that need it are Linux-only anyway.
    if mctx.os.name.lower().startswith("linux"):
        _system_libs_repo(name = "system_libs", libraries = libraries)
    else:
        _empty_repo(name = "system_libs")

    return mctx.extension_metadata(reproducible = False)

system_libs = module_extension(
    implementation = _system_libs_impl,
    tag_classes = {
        "library": tag_class(attrs = {
            "name": attr.string(mandatory = True, doc = "Target name in @system_libs."),
            "pkg_config": attr.string(mandatory = True, doc = "pkg-config package name."),
        }),
    },
    os_dependent = True,
)
