import CoreGraphics

/// Shared metrics, matching the GNOME shell's.
enum Layout {
    /// Messages and the composer are centered and at most this wide.
    static let contentMaxWidth: CGFloat = 640
    static let contentPadding: CGFloat = 16
    static let contentVerticalPadding: CGFloat = 12
    /// Inside a user message's bubble.
    static let bubblePadding: CGFloat = 16
    static let bubbleVerticalPadding: CGFloat = 12
    /// Of message bubbles and the composer.
    static let cornerRadius: CGFloat = 22
}
