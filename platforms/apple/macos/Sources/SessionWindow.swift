import AppKit
import SwiftUI

/// A window with one session and nothing else, opened from the sidebar. It
/// owns its core view and closes it with the window.
struct SessionWindow: View {
    @Environment(AppModel.self) private var model
    let viewId: UInt64?
    /// Keeps the window above the windows of other apps.
    @State private var isAlwaysOnTop = false

    var body: some View {
        if let viewId, model.view(viewId) != nil {
            ChatView(viewId: viewId)
                // Toolbar content also gives the window the full-height
                // toolbar of the main window; without it macOS shows a
                // compact title bar.
                .toolbar {
                    ToolbarItem(placement: .primaryAction) {
                        Toggle("Always on Top", systemImage: "pin", isOn: $isAlwaysOnTop)
                            .toggleStyle(.button)
                            .help("Always on Top")
                    }
                }
                .containerBackground(.regularMaterial, for: .window)
                // A companion window: full screen is for the main window.
                .windowFullScreenBehavior(.disabled)
                .background(ZoomButtonHider())
                .background(WindowLevelSetter(isFloating: isAlwaysOnTop))
                .onDisappear { model.send(.closeView(viewId: viewId)) }
        }
    }
}

/// Hides the window's zoom (green) button. SwiftUI can turn off full screen
/// but has no way to remove the button, so this reaches the `NSWindow`.
private struct ZoomButtonHider: NSViewRepresentable {
    func makeNSView(context: Context) -> NSView {
        HiderView()
    }

    func updateNSView(_ nsView: NSView, context: Context) {}

    private final class HiderView: NSView {
        override func viewDidMoveToWindow() {
            super.viewDidMoveToWindow()
            window?.standardWindowButton(.zoomButton)?.isHidden = true
        }
    }
}

/// Puts the window on the floating level while `isFloating` is true.
/// SwiftUI sets a window level only for a whole scene, so this reaches the
/// `NSWindow`.
private struct WindowLevelSetter: NSViewRepresentable {
    let isFloating: Bool

    func makeNSView(context: Context) -> LevelView {
        LevelView()
    }

    func updateNSView(_ nsView: LevelView, context: Context) {
        nsView.isFloating = isFloating
    }

    final class LevelView: NSView {
        var isFloating = false {
            didSet { applyLevel() }
        }

        override func viewDidMoveToWindow() {
            super.viewDidMoveToWindow()
            applyLevel()
        }

        private func applyLevel() {
            window?.level = isFloating ? .floating : .normal
        }
    }
}
