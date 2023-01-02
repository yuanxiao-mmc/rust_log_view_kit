use std::{ffi::CStr, os::raw::c_char};
use ws::{connect, CloseCode};

#[no_mangle]
pub extern "C" fn send_wind_info(message: *const c_char, host: *const c_char) {
    let c_host = unsafe { CStr::from_ptr(host) }.to_str().unwrap();
    let c_message = unsafe { CStr::from_ptr(message) }.to_str().unwrap();

    // 连接到url并调用闭包
    if let Err(error) = connect(c_host, |out| {
        // 将WebSocket打开时要发送的消息排队
        if out.send(c_message).is_err() {
            println!("[gaoding-log-view-kit]: 无法初始消息排队")
        }

        // 处理程序需要获取out的所有权，因此我们使用move
        move |msg| {
            // 处理在此连接上接收的消息
            println!("[gaoding-log-view-kit]: <receive> '{}'. ", msg);

            // 关闭连接
            out.close(CloseCode::Normal)
        }
    }) {
        // 通知用户故障
        println!(
            "[gaoding-log-view-kit]: Failed to create WebSocket due to: {:?}",
            error
        );
    }
}

/// Expose the JNI interface for android below
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::objects::{JClass, JString};
    use self::jni::JNIEnv;
    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_gaoding_log_view_kit_GDLogViewKit_sendWindInfo(
        env: JNIEnv,
        _: JClass,
        message: JString,
        host: JString,
    ) {
        send_wind_info(
            env.get_string(message)
                .expect("invalid pattern string")
                .as_ptr(),
            env.get_string(host)
                .expect("invalid pattern string")
                .as_ptr(),
        );
    }
}
