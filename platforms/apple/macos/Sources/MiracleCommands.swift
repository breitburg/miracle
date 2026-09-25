import SwiftUI

extension FocusedValues {
    /// Opens a new session in the focused window, if it can.
    @Entry var newSession: (() -> Void)?
}

/// Menu bar commands: New Chat replaces New Window. Only the main window
/// offers it; session windows each keep their one session.
struct MiracleCommands: Commands {
    @FocusedValue(\.newSession) private var newSession

    var body: some Commands {
        CommandGroup(replacing: .newItem) {
            Button("New Chat") {
                newSession?()
            }
            .keyboardShortcut("n")
            .disabled(newSession == nil)
        }
        SidebarCommands()
        InspectorCommands()
    }
}
