import MiracleCore
import SwiftUI

/// The chats with messages, grouped by date, newest first.
struct SidebarView: View {
    @Environment(AppModel.self) private var model
    @Environment(\.openWindow) private var openWindow
    @Binding var selection: UInt64?

    var body: some View {
        List(selection: $selection) {
            ForEach(model.sections) { section in
                Section(section.period.title) {
                    ForEach(section.chats) { chat in
                        Text(chat.title)
                            .lineLimit(1)
                            .tag(chat.id)
                            .contextMenu {
                                Button("Open in New Window", systemImage: "macwindow.badge.plus") {
                                    openWindow(value: model.openView(chatId: chat.id))
                                }
                            }
                    }
                }
            }
        }
        .navigationTitle("Chats")
    }
}
