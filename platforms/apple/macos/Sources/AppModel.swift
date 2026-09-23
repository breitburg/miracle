import Foundation
import MiracleCore
import Observation

/// Holds the one core `Store` that every window shares, and republishes its
/// state to SwiftUI, like the GNOME `MiracleAppModel`. Each window shows one
/// core view by its id; views on the same chat share its messages.
@MainActor
@Observable
final class AppModel {
    @ObservationIgnored private let store = Store()

    /// Newest first.
    private(set) var chats: [Chat] = []
    /// `chats` grouped for the sidebar.
    private(set) var sections: [ChatSection] = []
    /// One per open window. Kept apart from `chats`, so typing a draft does
    /// not redraw the chats.
    private(set) var views: [ChatViewState] = []
    /// The main window's view, open for the app's lifetime, on the newest
    /// chat.
    @ObservationIgnored let mainViewId: UInt64

    init() {
        mainViewId = store.openView(chatId: store.state().chats.first?.id)
        apply(store.state())
    }

    func chat(_ id: UInt64) -> Chat? {
        chats.first { $0.id == id }
    }

    func view(_ id: UInt64) -> ChatViewState? {
        views.first { $0.id == id }
    }

    func send(_ action: Action) {
        apply(store.dispatch(action: action))
    }

    /// Opens a view on the chat and returns its id, for a new window.
    func openView(chatId: UInt64?) -> UInt64 {
        let id = store.openView(chatId: chatId)
        apply(store.state())
        return id
    }

    private func apply(_ state: MiracleCore.State) {
        if state.chats != chats {
            chats = state.chats
            sections = store.chatSections(now: .now)
        }
        if state.views != views {
            views = state.views
        }
    }
}
