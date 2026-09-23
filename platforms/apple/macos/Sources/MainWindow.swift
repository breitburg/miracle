import SwiftUI

/// The main window: chats in the sidebar, next to its view.
struct MainWindow: View {
    @Environment(AppModel.self) private var model

    var body: some View {
        NavigationSplitView {
            SidebarView(selection: selection)
                .navigationSplitViewColumnWidth(min: 220, ideal: 280, max: 360)
        } detail: {
            ChatView(viewId: model.mainViewId)
                .toolbar {
                    // In the chat's toolbar, so it stays in reach when the
                    // sidebar is hidden.
                    ToolbarItem(placement: .primaryAction) {
                        Button("New Chat", systemImage: "square.and.pencil", action: newChat)
                            .help("New Chat")
                    }
                }
        }
        .focusedSceneValue(\.newChat, newChat)
    }

    private func newChat() {
        model.send(.showChat(viewId: model.mainViewId, chatId: nil))
    }

    /// The chat the main view shows; nothing is selected for a new chat.
    private var selection: Binding<UInt64?> {
        Binding {
            model.view(model.mainViewId)?.chatId
        } set: { id in
            if let id { model.send(.showChat(viewId: model.mainViewId, chatId: id)) }
        }
    }
}
