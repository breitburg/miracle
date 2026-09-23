extension Chat: Identifiable {}

extension ChatSummary: Identifiable {}

extension ChatSection: Identifiable {
    /// Each period appears once, as chats are grouped newest first.
    public var id: Period { period }
}
