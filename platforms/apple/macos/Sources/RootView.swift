import SwiftUI

/// The window: chats in the sidebar, the open chat in the detail column.
struct RootView: View {
    var body: some View {
        NavigationSplitView {
            SidebarView()
                .navigationSplitViewColumnWidth(min: 220, ideal: 280, max: 360)
        } detail: {
            ChatView()
        }
    }
}
