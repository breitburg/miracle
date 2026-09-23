import Foundation
import MiracleCore

extension Period {
    /// The sidebar section title: "Today", "Yesterday", "Previous 7 Days",
    /// "Previous 30 Days", a month name or a year.
    var title: String {
        switch self {
        case .today: String(localized: "Today")
        case .yesterday: String(localized: "Yesterday")
        case .previousSevenDays: String(localized: "Previous 7 Days")
        case .previousThirtyDays: String(localized: "Previous 30 Days")
        case .month(let month): Calendar.current.standaloneMonthSymbols[Int(month) - 1]
        case .year(let year): String(year)
        }
    }
}
