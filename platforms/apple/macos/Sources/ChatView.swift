import MiracleCore
import SwiftUI

/// One core view: its session's chat, or a hint for a new session, above
/// the composer with the view's draft. Views on the same session share its
/// chat, since they all read the shared store.
struct ChatView: View {
    let viewId: UInt64

    @Environment(AppModel.self) private var model
    @FocusState private var isComposerFocused: Bool

    var body: some View {
        let sessionId = model.view(viewId)?.sessionId
        let session = sessionId.flatMap(model.session)
        Group {
            if let session {
                MessageList(messages: session.chat.messages)
                    // A fresh scroll view per session opens at its last
                    // message.
                    .id(session.id)
            } else {
                EmptyChatView()
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .safeAreaBar(edge: .bottom) {
            Composer(text: draft, isFocused: $isComposerFocused, onSend: send)
        }
        .navigationTitle(session?.title ?? String(localized: "New Chat"))
        // Only scroll views get the soft scroll-edge effect; without this the
        // empty new session would show a toolbar separator.
        .toolbarBackgroundVisibility(.hidden, for: .windowToolbar)
        .frame(minWidth: 360, minHeight: 240)
        .defaultFocus($isComposerFocused, true)
        // Only a new session takes focus: picking a session keeps it in the
        // sidebar, which would otherwise lose its selection highlight.
        .onChange(of: sessionId) {
            if sessionId == nil { isComposerFocused = true }
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
