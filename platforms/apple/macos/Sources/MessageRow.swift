import AppKit
import MiracleCore
import SwiftUI

/// One message: the user's in a bubble at the trailing edge, the assistant's
/// as plain text across the column. Copy and Edit sit underneath, on the
/// same side, and appear on hover or keyboard focus, as in GNOME.
struct MessageRow: View {
    let message: Message

    @State private var isHovered = false
    @FocusState private var focusedAction: MessageAction?

    var body: some View {
        VStack(alignment: isUser ? .trailing : .leading, spacing: 12) {
            content
            MessageActions(text: message.content, focusedAction: $focusedAction)
                // In line with the text inside bubbles.
                .padding(.horizontal, Layout.bubblePadding)
                .opacity(areActionsVisible ? 1 : 0)
                .animation(.easeOut(duration: 0.15), value: areActionsVisible)
        }
        .frame(maxWidth: .infinity, alignment: isUser ? .trailing : .leading)
        .padding(.horizontal, Layout.contentPadding)
        .padding(.vertical, Layout.contentVerticalPadding)
        // Padding, not spacing, so the whole row counts for hover.
        .contentShape(.rect)
        .onHover { isHovered = $0 }
    }

    @ViewBuilder
    private var content: some View {
        let text = Text(message.content)
            .lineSpacing(6)
        if isUser {
            text
                .padding(.horizontal, Layout.bubblePadding)
                .padding(.vertical, Layout.bubbleVerticalPadding)
                .background(.fill.tertiary, in: .rect(cornerRadius: Layout.cornerRadius))
                .padding(.leading, 48)
        } else {
            // Lines up with the text inside user bubbles. Only assistant
            // text is selectable, as on GNOME.
            text
                .textSelection(.enabled)
                .padding(.horizontal, Layout.bubblePadding)
                .frame(maxWidth: .infinity, alignment: .leading)
        }
    }

    private var isUser: Bool { message.role == .user }

    private var areActionsVisible: Bool {
        isHovered || focusedAction != nil
    }
}

private enum MessageAction: Hashable {
    case copy
    case edit
}

/// Copy and Edit, as small borderless icon buttons.
private struct MessageActions: View {
    let text: String
    var focusedAction: FocusState<MessageAction?>.Binding

    /// Every icon is centered in a square this size, so swapping in the
    /// checkmark does not move the layout.
    private static let iconSize: CGFloat = 16

    @State private var copies = 0
    @State private var showsCopied = false

    var body: some View {
        HStack(spacing: 12) {
            Button {
                NSPasteboard.general.clearContents()
                NSPasteboard.general.setString(text, forType: .string)
                copies += 1
            } label: {
                Label(
                    showsCopied ? "Copied" : "Copy",
                    systemImage: showsCopied ? "checkmark" : "doc.on.doc"
                )
                .contentTransition(.symbolEffect(.replace))
                .frame(width: Self.iconSize, height: Self.iconSize)
            }
            .help("Copy")
            .focused(focusedAction, equals: .copy)

            // Editing is not in the core yet.
            Button {} label: {
                Label("Edit", systemImage: "pencil")
                    .frame(width: Self.iconSize, height: Self.iconSize)
            }
            .help("Edit")
            .disabled(true)
            .focused(focusedAction, equals: .edit)
        }
        .labelStyle(.iconOnly)
        .buttonStyle(.borderless)
        .imageScale(.small)
        .fontWeight(.semibold)
        .foregroundStyle(.secondary)
        // Confirm in place instead of a toast: the icon turns into a check.
        .task(id: copies) {
            guard copies > 0 else { return }
            showsCopied = true
            try? await Task.sleep(for: .seconds(1.5))
            showsCopied = false
        }
    }
}
