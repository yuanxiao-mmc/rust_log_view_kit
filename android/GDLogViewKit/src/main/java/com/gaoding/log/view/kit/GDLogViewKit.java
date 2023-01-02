package com.gaoding.log.view.kit;

/**
 * 日志可视化工具包
 */
public class GDLogViewKit {

    static {
        System.loadLibrary("gaoding_log_view_kit");
    }
  
    private static String kAddress = null;

    private static native void sendWindInfo(final String message, final String host);

    /**
     * 保存 IP 地址
     */
    public static void saveAddress(String address) {
        if (address == null || address.length() == 0) {
          address = null;
          return;
        }
        kAddress = "ws://" + address;
    }

    /**
     * 发送埋点信息
     */
    public static void sendWindMessage(String message) {
        if (kAddress == null) {
          return;
        }
        GDLogViewKit.sendWindInfo(message, kAddress);
    }
}