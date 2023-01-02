//
//  GDLogViewKit.m
//  GDLogViewKit
//
//  AUTO GENERATE
//

#import "GDLogViewKit.h"
#import "api.h"

NS_ASSUME_NONNULL_BEGIN

static NSString *connectAddress;

@implementation GDLogViewKit

+ (void)saveConnectAddress:(NSString *)address {
  connectAddress = [NSString stringWithFormat:@"ws://%@", address];
}

+ (void)stop {
  connectAddress = nil;
}

+ (void)sendWindMessage:(NSString *)message {
  if (!connectAddress || !message) {
    return;
  }
  send_wind_info([message cStringUsingEncoding:NSUTF8StringEncoding], [connectAddress cStringUsingEncoding:NSUTF8StringEncoding]);
}

@end

NS_ASSUME_NONNULL_END
