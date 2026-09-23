import SwiftUI

@main
struct MiracleApp: App {
    @State private var model = AppModel()

    var body: some Scene {
        // One window: every window would show the same store and selection.
        Window("Miracle", id: "main") {
            RootView()
                .environment(model)
        }
        .defaultSize(width: 1000, height: 720)
        // Open at every launch, even if the window was closed at quit.
        .defaultLaunchBehavior(.presented)
        .windowToolbarStyle(.unified)
        .commands {
            MiracleCommands(model: model)
        }
    }
}
