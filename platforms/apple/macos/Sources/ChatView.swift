import SwiftUI

/// The open chat: its messages, or a hint for the new chat, above the
/// composer.
struct ChatView: View {
    @Environment(AppModel.self) private var model
    @FocusState private var isComposerFocused: Bool

    var body: some View {
        let chat = model.selectedChat
        Group {
            if let chat {
                MessageList(chat: chat)
                    // A fresh scroll view per chat opens at its last message.
                    .id(chat.id)
            } else {
                EmptyChatView()
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .safeAreaBar(edge: .bottom) {
            Composer(text: draft, isFocused: $isComposerFocused, onSend: send)
        }
        .navigationTitle(chat?.title ?? String(localized: "New Chat"))
        // Only scroll views get the soft scroll-edge effect; without this the
        // empty new chat would show a toolbar separator.
        .toolbarBackgroundVisibility(.hidden, for: .windowToolbar)
        .toolbar {
            // In the chat's toolbar, so it stays in reach when the sidebar
            // is hidden.
            ToolbarItem(placement: .primaryAction) {
                Button("New Chat", systemImage: "square.and.pencil") {
                    model.send(.openNewChat)
                }
                .help("New Chat")
            }
        }
        .frame(minWidth: 360, minHeight: 240)
        .defaultFocus($isComposerFocused, true)
        // Only a new chat takes focus: picking a chat keeps it in the
        // sidebar, which would otherwise lose its selection highlight.
        .onChange(of: model.selectedChatId) { _, id in
            if id == nil { isComposerFocused = true }
        }
    }

    /// The core's draft: every edit is sent to it.
    private var draft: Binding<String> {
        Binding {
            model.draft
        } set: { text in
            model.send(.editDraft(text: text))
        }
    }

    private func send() {
        withAnimation(.snappy) {
            model.send(.sendMessage(sentAt: .now))
        }
    }
}
