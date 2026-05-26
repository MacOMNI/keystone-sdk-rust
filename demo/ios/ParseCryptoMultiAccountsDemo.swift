import Foundation
import URRegistryFFI

public enum ParseCryptoMultiAccountsDemo {
    private static let targetGroupCount = 10
    private static let minSuccessGroupCount = 8
    private static let sourcePartStart = 2

    private struct CandidateGroup {
        let label: String
        let parts: [String]
    }

    public enum DemoError: Error, CustomStringConvertible {
        case ffiError(code: Int32, message: String)
        case emptyResult
        case decodeError(String)
        case decodePayloadInvalid(String)

        public var description: String {
            switch self {
            case let .ffiError(code, message):
                return "FFI error(code=\(code)): \(message)"
            case .emptyResult:
                return "FFI returned nil result"
            case let .decodeError(message):
                return "Decode failed: \(message)"
            case let .decodePayloadInvalid(payload):
                return "Decode payload is invalid: \(payload)"
            }
        }
    }

    public static func run() throws -> String {
        let groups = candidateGroups(parts: CryptoMultiAccountsLogSamples.parts)
        var details: [String] = []
        var success = 0
        var tested = 0

        for (index, group) in groups.enumerated() {
            if tested >= targetGroupCount {
                break
            }
            tested += 1
            do {
                let decodeInputData = try JSONSerialization.data(withJSONObject: group.parts)
                let decodeInput = String(data: decodeInputData, encoding: .utf8) ?? ""
                let decodeResult = try decodeURToCborHex(decodeInput)
                let payload = try decodePayload(from: decodeResult)
                let parseResult = try parseCryptoMultiAccounts(urType: payload.urType, cborHex: payload.cborHex)

                guard
                    let parseData = parseResult.data(using: .utf8),
                    let parseObject = try JSONSerialization.jsonObject(with: parseData) as? [String: Any]
                else {
                    details.append("Group \(index + 1) (\(group.label)) [\(group.parts.count) parts]: parse payload invalid")
                    continue
                }

                if let error = parseObject["error"] as? String {
                    details.append("Group \(index + 1) (\(group.label)) [\(group.parts.count) parts]: parse error=\(error)")
                    continue
                }

                let keyCount = (parseObject["keys"] as? [Any])?.count ?? 0
                let fingerprint = (parseObject["master_fingerprint"] as? String) ?? ""
                details.append("Group \(index + 1) (\(group.label)) [\(group.parts.count) parts]: OK type=\(payload.urType) cbor_len=\(payload.cborHex.count) keys=\(keyCount) xfp=\(fingerprint)")
                success += 1
            } catch {
                details.append("Group \(index + 1) (\(group.label)) [\(group.parts.count) parts]: error=\(error)")
            }
        }

        let status = success >= minSuccessGroupCount ? "PASS" : "FAIL"
        return """
        Batch multipart decode+parse: \(status)
        success=\(success) tested=\(tested) target=\(targetGroupCount) min_success=\(minSuccessGroupCount)

        \(details.joined(separator: "\n"))
        """
    }

    private static func decodeURToCborHex(_ ur: String) throws -> String {
        var err = ExternError(code: 0, message: nil)
        let raw = decode_ur_to_cbor_hex(&err, ur)
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

    private static func parseCryptoMultiAccounts(urType: String, cborHex: String) throws -> String {
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

    private static func decodePayload(from decodeResult: String) throws -> (urType: String, cborHex: String) {
        guard
            let data = decodeResult.data(using: .utf8),
            let object = try JSONSerialization.jsonObject(with: data) as? [String: Any]
        else {
            throw DemoError.decodePayloadInvalid(decodeResult)
        }

        if let error = object["error"] as? String {
            throw DemoError.decodeError(error)
        }

        guard
            let urType = object["type"] as? String,
            let cborHex = object["cbor"] as? String,
            !urType.isEmpty,
            !cborHex.isEmpty
        else {
            throw DemoError.decodePayloadInvalid(decodeResult)
        }

        return (urType: urType, cborHex: cborHex)
    }

    private static func candidateGroups(parts: [String]) -> [CandidateGroup] {
        var groups: [CandidateGroup] = []
        for window in 12...16 {
            if window > parts.count {
                continue
            }
            for start in 0...(parts.count - window) {
                let partStart = sourcePartStart + start
                let partEnd = partStart + window - 1
                groups.append(
                    CandidateGroup(
                        label: "parts \(partStart)-\(partEnd)",
                        parts: Array(parts[start..<(start + window)])
                    )
                )
            }
        }
        groups.append(
            CandidateGroup(
                label: "even-index-pick",
                parts: parts.enumerated().compactMap { index, value in
                    index.isMultiple(of: 2) ? value : nil
                }
            )
        )
        groups.append(
            CandidateGroup(
                label: "odd-index-pick",
                parts: parts.enumerated().compactMap { index, value in
                    index.isMultiple(of: 2) ? nil : value
                }
            )
        )
        return groups
    }
}
