import Foundation
import MiracleCore
import Observation

/// Holds the one core `Store` that every window shares, and republishes its
/// state to SwiftUI, like the GNOME `MiracleAppModel`. Each window shows one
/// core view by its id; views on the same session share its chat.
@MainActor
@Observable
final class AppModel {
    @ObservationIgnored private let store = Store()

    /// Newest first.
    private(set) var sessions: [Session] = []
    /// `sessions` grouped for the sidebar.
    private(set) var sections: [SessionSection] = []
    /// One per open window. Kept apart from `sessions`, so typing a draft
    /// does not redraw the sessions.
    private(set) var views: [SessionViewState] = []
    /// The main window's view, open for the app's lifetime, on the newest
    /// session.
    @ObservationIgnored let mainViewId: UInt64

    init() {
        mainViewId = store.openView(sessionId: store.state().sessions.first?.id)
        apply(store.state())
    }

    func session(_ id: UInt64) -> Session? {
        sessions.first { $0.id == id }
    }

    func view(_ id: UInt64) -> SessionViewState? {
        views.first { $0.id == id }
    }

    func send(_ action: Action) {
        apply(store.dispatch(action: action))
    }

    /// Opens a view on the session and returns its id, for a new window.
    func openView(sessionId: UInt64?) -> UInt64 {
        let id = store.openView(sessionId: sessionId)
        apply(store.state())
        return id
    }

    private func apply(_ state: MiracleCore.State) {
        if state.sessions != sessions {
            sessions = state.sessions
            sections = store.sessionSections(now: .now)
        }
        if state.views != views {
            views = state.views
        }
    }
}
