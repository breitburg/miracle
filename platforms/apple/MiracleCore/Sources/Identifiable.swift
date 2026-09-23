extension Session: Identifiable {}

extension SessionSummary: Identifiable {}

extension SessionSection: Identifiable {
    /// Each period appears once, as sessions are grouped newest first.
    public var id: Period { period }
}
