import SwiftUI

struct ContentView: View {
    @State private var output = "Tap \"Parse\" to call parse_crypto_multi_accounts"

    var body: some View {
        NavigationView {
            VStack(alignment: .leading, spacing: 12) {
                Button("Parse") {
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
