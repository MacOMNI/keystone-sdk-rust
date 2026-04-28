import Foundation
import URRegistryFFI

public enum ParseCryptoMultiAccountsDemo {
    public enum DemoError: Error, CustomStringConvertible {
        case ffiError(code: Int32, message: String)
        case emptyResult

        public var description: String {
            switch self {
            case let .ffiError(code, message):
                return "FFI error(code=\(code)): \(message)"
            case .emptyResult:
                return "FFI returned nil result"
            }
        }
    }

    private static let urType = "crypto-multi-accounts"
    private static let cborHex =
        "a5011ae9181cf30281d9012fa203582102eae4b876a8696134b868f88cc2f51f715f2dbedb7446b8e6edf3d4541c4eb67b06d90130a10188182cf51901f5f500f500f503686b657973746f6e65047828323834373563386438306636633036626166626534366137643137353066336663663235363566370565312e302e30"

    public static func run() throws -> String {
        var err = ExternError(code: 0, message: nil)
        let raw = parse_crypto_multi_accounts(&err, urType, cborHex)

        defer {
            if let raw {
                keystone_sdk_destroy_string(raw)
            }
            if let message = err.message {
                keystone_sdk_destroy_string(UnsafePointer(message))
            }
        }

        if err.code != 0 {
            let message = err.message.map { String(cString: $0) } ?? "unknown ffi error"
            throw DemoError.ffiError(code: err.code, message: message)
        }

        guard let raw else {
            throw DemoError.emptyResult
        }

        return String(cString: raw)
    }
}

