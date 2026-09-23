import AppKit
import SwiftUI

/// The message field with its send button, on glass above the messages.
/// It grows with its text instead of GNOME's draggable pane.
struct Composer: View {
    @Binding var text: String
    var isFocused: FocusState<Bool>.Binding
    let onSend: () -> Void

    var body: some View {
        HStack(alignment: .bottom, spacing: 8) {
            TextField("Message", text: $text, axis: .vertical)
                .textFieldStyle(.plain)
                .lineLimit(1...8)
                .focused(isFocused)
                // Return sends; Shift- or Option-Return starts a new line.
                .onKeyPress(.return, phases: [.down, .repeat]) { press in
                    if press.modifiers.isEmpty {
                        onSend()
                    } else {
                        // What Option-Return does in a field editor: a line
                        // break at the cursor, inserted by AppKit.
                        NSApp.sendAction(
                            #selector(NSResponder.insertNewlineIgnoringFieldEditor(_:)),
                            to: nil,
                            from: nil
                        )
                    }
                    return .handled
                }
                .padding(.leading, Layout.contentPadding)
                .padding(.vertical, 11)

            Button(action: onSend) {
                Label("Send", systemImage: "arrow.up")
                    .labelStyle(.iconOnly)
                    .fontWeight(.semibold)
                    .frame(width: 20, height: 20)
            }
            .buttonStyle(.borderedProminent)
            .buttonBorderShape(.circle)
            .help("Send")
            .disabled(!canSend)
            .padding(6)
        }
        .glassEffect(.regular.interactive(), in: .rect(cornerRadius: Layout.cornerRadius))
        .frame(maxWidth: Layout.contentMaxWidth)
        .padding(.horizontal, Layout.contentPadding)
        .padding(.bottom, Layout.contentVerticalPadding)
    }

    /// Only disables the button: the core ignores blank drafts itself.
    private var canSend: Bool {
        !text.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty
    }
}
