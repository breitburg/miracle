import MiracleCore
import SwiftUI

/// A chat's messages, oldest first, in a centered column.
struct MessageList: View {
    let messages: [Message]
    @State private var position = ScrollPosition()

    var body: some View {
        ScrollView {
            LazyVStack(spacing: 0) {
                // Messages are only ever appended, so an index is a stable id.
                ForEach(messages.indices, id: \.self) { index in
                    MessageRow(message: messages[index])
                        .transition(
                            .asymmetric(
                                insertion: .opacity.combined(with: .offset(y: 16)),
                                removal: .identity
                            )
                        )
                }
            }
            .frame(maxWidth: Layout.contentMaxWidth)
            .frame(maxWidth: .infinity)
            .padding(.vertical, Layout.contentVerticalPadding)
            .scrollTargetLayout()
        }
        .scrollPosition($position)
        // Short chats start at the top; long ones open at the last message
        // and stay there as messages arrive.
        .defaultScrollAnchor(.top, for: .alignment)
        .defaultScrollAnchor(.bottom, for: .initialOffset)
        .defaultScrollAnchor(.bottom, for: .sizeChanges)
        // Bring a sent message into view, even when scrolled back.
        .onChange(of: messages.count) { _, count in
            withAnimation(.snappy) { position.scrollTo(id: count - 1, anchor: .bottom) }
        }
    }
}
