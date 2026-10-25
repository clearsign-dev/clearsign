import ExpoModulesCore
import ClearSignNative

public class ClearSignCoreModule: Module {
  public func definition() -> ModuleDefinition {
    Name("ClearSignCore")

    Constant("apiVersion") {
      Int(clearsign_api_version())
    }

    AsyncFunction("reviewSafeJson") { (jsonText: String, chainId: String, version: Int) -> String in
      guard let parsedChainId = UInt64(chainId) else {
        throw Exception(name: "InvalidChainId", description: "Chain ID must be a whole, non-negative number.")
      }
      guard let parsedVersion = UInt32(exactly: version) else {
        throw Exception(name: "InvalidDomainVersion", description: "Safe domain version is invalid.")
      }

      let maximum = Int(clearsign_max_record_bytes())
      guard jsonText.utf8.count <= maximum else {
        throw Exception(name: "RecordTooLarge", description: "The transaction record exceeds the \(maximum)-byte review limit.")
      }
      let input = Array(jsonText.utf8)
      let required = input.withUnsafeBytes { inputBytes in
        clearsign_review_safe_json(
          inputBytes.bindMemory(to: UInt8.self).baseAddress,
          input.count,
          parsedChainId,
          parsedVersion,
          nil,
          0
        )
      }
      guard required >= 0 else {
        throw Exception(name: "ReviewFailed", description: "The review engine refused the input boundary (error \(required)).")
      }

      var output = [UInt8](repeating: 0, count: Int(required))
      let outputCapacity = output.count
      let written = input.withUnsafeBytes { inputBytes in
        output.withUnsafeMutableBytes { outputBytes in
          clearsign_review_safe_json(
            inputBytes.bindMemory(to: UInt8.self).baseAddress,
            input.count,
            parsedChainId,
            parsedVersion,
            outputBytes.bindMemory(to: UInt8.self).baseAddress,
            outputCapacity
          )
        }
      }
      guard written == required, let response = String(bytes: output, encoding: .utf8) else {
        throw Exception(name: "ReviewFailed", description: "The review engine returned an invalid response.")
      }
      return response
    }
  }
}
