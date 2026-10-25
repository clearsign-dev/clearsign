package dev.clearsign.core

internal object ClearSignNativeBridge {
  init {
    System.loadLibrary("clearsign_native")
  }

  external fun apiVersion(): Int
  external fun reviewSafeJson(jsonText: String, chainId: String, version: Int): String
}
