use serde_json::Value;
use std::net::UdpSocket;

use ws::listen;

fn main() {
    let address = format!("{}:{}", get_ip().unwrap(), get_available_port());
    println!("当前地址为：{}", address);
    // 监听地址并为每个连接调用闭包
    if let Err(error) = listen(address, |out| {
        // 处理程序需要获取out的所有权，因此我们使用move
        move |msg: ws::Message| {
            let text = msg.as_text();
            match text {
                Ok(_t) => {
                    // println!("{}", msg);
                    let json: Value = serde_json::from_str(_t).unwrap();
                    println!(
                        "触发埋点\n event_id: {}\n event_name: {}\n attributes: {}\n\n",
                        json["event_id"],
                        json["event"],
                        serde_json::to_string(&json["attributes"]).unwrap()
                    );

                    // 使用输出通道发送消息
                    out.send(msg)
                }
                Err(_) => todo!(),
            }
        }
    }) {
        // 通知用户故障
        println!("创建Websocket失败，原因： {:?}", error);
    }
}

/// 获取本机 IP 地址
pub fn get_ip() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => return Some(addr.ip().to_string()),
        Err(_) => return None,
    };
}

/// 获取可用的端口号
fn get_available_port() -> u16 {
    std::net::TcpListener::bind("0.0.0.0:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}
