Pod::Spec.new do |s|
  s.name           = 'ClearSignCore'
  s.version        = '1.0.0'
  s.summary        = 'The native ClearSign transaction review engine.'
  s.description    = 'Runs the shared Rust reviewer locally and returns the versioned ClearSign review response.'
  s.author         = 'ClearSign contributors'
  s.homepage       = 'https://github.com/clearsign-dev/clearsign'
  s.platforms      = {
    :ios => '16.4',
    :tvos => '16.4'
  }
  s.source         = { git: '' }
  s.static_framework = true

  s.dependency 'ExpoModulesCore'
  s.vendored_frameworks = 'native/ClearSignNative.xcframework'
  s.preserve_paths = 'native/ClearSignNative.xcframework'

  # Swift/Objective-C compatibility
  s.pod_target_xcconfig = {
    'DEFINES_MODULE' => 'YES',
  }

  s.source_files = "*.swift"
end
