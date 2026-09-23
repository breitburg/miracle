"""Helpers for the GNOME shell build."""

def host_tool(tool, package):
    """Shell prefix that runs a host tool, or fails with an install hint.

    Host tools come from the same system that provides GTK (the host during
    development, the GNOME SDK inside Flatpak).
    """
    return "command -v {t} >/dev/null || {{ echo 'error: {t} not found; install {p}' >&2; exit 1; }}; {t}".format(
        t = tool,
        p = package,
    )
