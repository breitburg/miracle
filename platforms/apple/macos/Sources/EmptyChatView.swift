import SwiftUI

/// The new chat has no messages yet, so it shows a hint instead.
struct EmptyChatView: View {
    var body: some View {
        ContentUnavailableView {
            Label("Start a Conversation", systemImage: "bubble.left.and.text.bubble.right")
        } description: {
            Text("Send your first message to begin")
        }
    }
}
