import MiracleCore
import SwiftUI

/// The inspector of the main window: settings of the session that the main
/// view shows.
struct InspectorView: View {
    @Environment(AppModel.self) private var model
    /// Every model, and the model of a new session, from the core. They
    /// never change, so fetch them once.
    private static let models = MiracleCore.models()
    private static let defaultModel = MiracleCore.defaultModel()

    var body: some View {
        let session = model.mainSessionId.flatMap(model.session)
        Form {
            Picker("Model", selection: selection(of: session)) {
                ForEach(Self.models, id: \.self) { Text(modelId(model: $0)).tag($0) }
            }
            // A new session has no model yet: it shows the default.
            .disabled(session == nil)
        }
        .formStyle(.grouped)
    }

    private func selection(of session: Session?) -> Binding<Model> {
        let (sessionId, current) = (session?.id, session?.model ?? Self.defaultModel)
        return Binding {
            current
        } set: { newModel in
            if let sessionId { model.send(.setModel(sessionId: sessionId, model: newModel)) }
        }
    }
}
