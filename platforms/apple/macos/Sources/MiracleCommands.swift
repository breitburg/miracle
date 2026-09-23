import SwiftUI

extension FocusedValues {
    /// Opens a new chat in the focused window, if it can.
    @Entry var newChat: (() -> Void)?
}

/// Menu bar commands: New Chat replaces New Window. Only the main window
/// offers it; chat windows each keep their one chat.
struct MiracleCommands: Commands {
    @FocusedValue(\.newChat) private var newChat

    var body: some Commands {
        CommandGroup(replacing: .newItem) {
            Button("New Chat") {
                newChat?()
            }
            .keyboardShortcut("n")
            .disabled(newChat == nil)
        }
        SidebarCommands()
    }
}
