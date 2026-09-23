import Foundation
import MiracleCore
import Observation

/// Holds the core `Store` and republishes its state to SwiftUI, like the
/// GNOME `MiracleAppModel`. Each part is its own property and changes only
/// when the core's does, so typing does not redraw the chats.
@MainActor
@Observable
final class AppModel {
    @ObservationIgnored private let store = Store()

    /// Newest first.
    private(set) var chats: [Chat] = []
    /// `chats` grouped for the sidebar.
    private(set) var sections: [ChatSection] = []
    /// `nil` while the new chat is open.
    private(set) var selectedChatId: UInt64?
    /// The unsent text in the composer.
    private(set) var draft = ""

    init() {
        apply(store.state())
    }

    /// The open chat, or `nil` while the new chat is open.
    var selectedChat: Chat? {
        guard let selectedChatId else { return nil }
        return chats.first { $0.id == selectedChatId }
    }

    func send(_ action: Action) {
        apply(store.dispatch(action: action))
    }

    private func apply(_ state: MiracleCore.State) {
        if state.chats != chats {
            chats = state.chats
            sections = store.chatSections(now: .now)
        }
        if state.selectedChatId != selectedChatId {
            selectedChatId = state.selectedChatId
        }
        if state.draft != draft {
            draft = state.draft
        }
    }
}
