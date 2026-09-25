import SwiftUI

/// The main window: sessions in the sidebar, next to its view, with an
/// inspector on the right.
struct MainWindow: View {
    @Environment(AppModel.self) private var model
    @State private var isInspectorPresented = false

    var body: some View {
        NavigationSplitView {
            SidebarView(selection: selection)
                .navigationSplitViewColumnWidth(min: 220, ideal: 280, max: 360)
        } detail: {
            ChatView(viewId: model.mainViewId)
                .toolbar {
                    // In the session's toolbar, so it stays in reach when the
                    // sidebar is hidden.
                    ToolbarItem(placement: .primaryAction) {
                        Button("New Chat", systemImage: "square.and.pencil", action: newSession)
                            .help("New Chat")
                    }
                    ToolbarItem(placement: .primaryAction) {
                        Toggle("Inspector", systemImage: "sidebar.right", isOn: $isInspectorPresented)
                            .toggleStyle(.button)
                            .help("Hide or Show the Inspector")
                    }
                }
                .inspector(isPresented: $isInspectorPresented) {
                    InspectorView()
                        .inspectorColumnWidth(min: 220, ideal: 280, max: 360)
                }
        }
        .focusedSceneValue(\.newSession, newSession)
    }

    private func newSession() {
        model.send(.showSession(viewId: model.mainViewId, sessionId: nil))
    }

    /// The session the main view shows; nothing is selected for a new
    /// session.
    private var selection: Binding<UInt64?> {
        Binding {
            model.mainSessionId
        } set: { id in
            if let id { model.send(.showSession(viewId: model.mainViewId, sessionId: id)) }
        }
    }
}
