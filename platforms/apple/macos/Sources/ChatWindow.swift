import AppKit
import SwiftUI

/// A window with one chat and nothing else, opened from the sidebar. It owns
/// its core view and closes it with the window.
struct ChatWindow: View {
    @Environment(AppModel.self) private var model
    let viewId: UInt64?

    var body: some View {
        if let viewId, model.view(viewId) != nil {
            ChatView(viewId: viewId)
                // Toolbar content, even an empty spacer, gives the window the
                // full-height toolbar of the main window; without it macOS
                // shows a compact title bar.
                .toolbar { ToolbarSpacer(.flexible) }
                .containerBackground(.regularMaterial, for: .window)
                // A companion window: full screen is for the main window.
                .windowFullScreenBehavior(.disabled)
                .background(ZoomButtonHider())
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
