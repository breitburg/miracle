import SwiftUI

@main
struct MiracleApp: App {
    var body: some Scene {
        WindowGroup("Miracle") {
            ContentView()
                .frame(minWidth: 480, minHeight: 320)
        }
        .defaultSize(width: 1000, height: 720)
    }
}
