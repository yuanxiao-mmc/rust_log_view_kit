#
# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint FGUIComponentAPI.podspec` to validate before publishing.
#
Pod::Spec.new do |s|
  s.name = "GDLogViewKit"
  s.version = "0.0.1"
  s.summary          = '日志可视化工具.'
  s.description      = <<-DESC
  稿定日志可视化工具，用于埋点信息实时查看，Log 信息实时查看.
                       DESC
  s.homepage         = 'https://git.gaoding.com/gdmobile/GDLogViewKit'
  s.license          = { :type => 'MIT', :file => 'LICENSE' }
  s.author           = { 'yuanxiao' => 'yuanxiao@gaoding.com' }
  s.source           = { :git => 'https://git.gaoding.com/gdmobile/GDLogViewKit', :tag => s.version.to_s }

  s.ios.deployment_target = '9.0'
  
  s.source_files = 'GDLogViewKit/Classes/**/*.{m,h}'

  s.vendored_libraries = 'GDLogViewKit/Library/**/*.a'

  s.public_header_files = 'GDLogViewKit/Classes/*.h'

end
