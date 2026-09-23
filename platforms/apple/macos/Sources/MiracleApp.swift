import SwiftUI

@main
struct MiracleApp: App {
    /// One store for every window.
    @State private var model = AppModel()

    var body: some Scene {
        Window("Miracle", id: "main") {
            MainWindow()
                .environment(model)
        }
        .defaultSize(width: 1000, height: 720)
        // Open at every launch, even if the window was closed at quit.
        .defaultLaunchBehavior(.presented)
        .windowToolbarStyle(.unified)
        .commands {
            MiracleCommands()
        }

        // "Open in New Window": one window per core view id.
        WindowGroup("Chat", for: UInt64.self) { $viewId in
            SessionWindow(viewId: viewId)
                .environment(model)
        }
        // Views live in memory, so there is nothing to restore after a
        // relaunch.
        .restorationBehavior(.disabled)
        .defaultSize(width: 640, height: 720)
        .windowToolbarStyle(.unified)
    }
}
