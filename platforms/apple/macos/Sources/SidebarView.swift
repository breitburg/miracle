import MiracleCore
import SwiftUI

/// The chats, grouped by date, newest first. No row is selected while the
/// new chat is open.
struct SidebarView: View {
    @Environment(AppModel.self) private var model

    var body: some View {
        List(selection: selection) {
            ForEach(model.sections) { section in
                Section(section.period.title) {
                    ForEach(section.chats) { chat in
                        Text(chat.title)
                            .lineLimit(1)
                            .tag(chat.id)
                    }
                }
            }
        }
        .navigationTitle("Chats")
    }

    /// The core's selection: choosing a row selects that chat.
    private var selection: Binding<UInt64?> {
        Binding {
            model.selectedChatId
        } set: { id in
            if let id { model.send(.selectChat(id: id)) }
        }
    }
}
