//
//  GDLogViewKit.h
//  GDLogViewKit
//
//  AUTO GENERATE
//

#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

/// 埋点日志可视化工具包
@interface GDLogViewKit : NSObject

/// 保存连接地址
/// - Parameters:
///   - address: ip 以及端口
+ (void)saveConnectAddress:(NSString *)address;

/// 停止发送
+ (void)stop;

/// 发送埋点信息
/// - Parameters:
///   - address: ip 以及端口
+ (void)sendWindMessage:(NSString *)message;

@end

NS_ASSUME_NONNULL_END
