package dev.clearsign.core

import expo.modules.kotlin.modules.Module
import expo.modules.kotlin.modules.ModuleDefinition

class ClearSignCoreModule : Module() {
  override fun definition() = ModuleDefinition {
    Name("ClearSignCore")

    Constants {
      mapOf("apiVersion" to ClearSignNativeBridge.apiVersion())
    }

    AsyncFunction("reviewSafeJson") { jsonText: String, chainId: String, version: Int ->
      require(jsonText.length <= MAX_RECORD_BYTES) { "The transaction record exceeds the review limit." }
      require(jsonText.toByteArray(Charsets.UTF_8).size <= MAX_RECORD_BYTES) {
        "The transaction record exceeds the review limit."
      }
      ClearSignNativeBridge.reviewSafeJson(jsonText, chainId, version)
    }
  }

  private companion object {
    const val MAX_RECORD_BYTES = 1024 * 1024
  }
}
