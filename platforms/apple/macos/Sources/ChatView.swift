import MiracleCore
import SwiftUI

/// One core view: its chat's messages, or a hint for a new chat, above the
/// composer with the view's draft. Views on the same chat share its
/// messages, since they all read the shared store.
struct ChatView: View {
    let viewId: UInt64

    @Environment(AppModel.self) private var model
    @FocusState private var isComposerFocused: Bool

    var body: some View {
        let chatId = model.view(viewId)?.chatId
        let chat = chatId.flatMap(model.chat)
        Group {
            if let chat {
                MessageList(messages: chat.messages)
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
        .frame(minWidth: 360, minHeight: 240)
        .defaultFocus($isComposerFocused, true)
        // Only a new chat takes focus: picking a chat keeps it in the
        // sidebar, which would otherwise lose its selection highlight.
        .onChange(of: chatId) {
            if chatId == nil { isComposerFocused = true }
        }
    }

    /// The view's draft in the core: every edit is sent to it.
    private var draft: Binding<String> {
        Binding {
            model.view(viewId)?.draft ?? ""
        } set: { text in
            model.send(.editDraft(viewId: viewId, text: text))
        }
    }

    private func send() {
        withAnimation(.snappy) {
            model.send(.sendMessage(viewId: viewId, sentAt: .now))
        }
    }
}
