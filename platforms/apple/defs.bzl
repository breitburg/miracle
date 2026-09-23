"""Helpers shared by the Apple shells."""

load("@platforms//host:constraints.bzl", "HOST_CONSTRAINTS")

# Apple targets need Xcode, so they build only on a macOS host. This checks
# the host, not the target platform: rules_apple transitions to the Apple
# platform before target_compatible_with is evaluated, so a target-OS check
# would never skip macos_application on Linux.
REQUIRES_MACOS_HOST = [] if (
    "@platforms//os:osx" in HOST_CONSTRAINTS or "@platforms//os:macos" in HOST_CONSTRAINTS
) else ["@platforms//:incompatible"]
