import SwiftUI

struct ContentView: View {
    @State private var output = "Tap \"Batch Verify\" to run multi-part decode+parse for 8-10 sample groups"

    var body: some View {
        NavigationView {
            VStack(alignment: .leading, spacing: 12) {
                Button("Batch Verify") {
                    do {
                        output = try ParseCryptoMultiAccountsDemo.run()
                    } catch {
                        output = "Error: \(error)"
                    }
                }
                .buttonStyle(.borderedProminent)

                ScrollView {
                    Text(output)
                        .font(.system(.footnote, design: .monospaced))
                        .frame(maxWidth: .infinity, alignment: .leading)
                        .textSelection(.enabled)
                }
                .padding(12)
                .background(Color(.secondarySystemBackground))
                .clipShape(RoundedRectangle(cornerRadius: 12))
            }
            .padding()
            .navigationTitle("Keystone FFI Demo")
        }
    }
}
