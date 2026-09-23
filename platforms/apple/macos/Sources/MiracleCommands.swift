import SwiftUI

/// Menu bar commands: New Chat replaces New Window, since the app has one
/// window over one store.
struct MiracleCommands: Commands {
    let model: AppModel

    var body: some Commands {
        CommandGroup(replacing: .newItem) {
            Button("New Chat") {
                model.send(.openNewChat)
            }
            .keyboardShortcut("n")
        }
        SidebarCommands()
    }
}
