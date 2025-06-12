use actix::prelude::*;

/// 定义 `Ping` 消息结构体
/// Define `Ping` message
struct Ping(usize);

// 为 Ping 实现 Message trait，表示它是一个可以发送给 Actor 的消息
impl Message for Ping {
    type Result = usize; // 消息处理后的返回类型
}

/// Actor 结构体
/// Actor
struct MyActor {
    count: usize, // Actor 内部状态：计数器
}

/// 声明 actor 及其上下文
/// Declare actor and its context
impl Actor for MyActor {
    type Context = Context<Self>; // 使用标准的 Actor 上下文
}

/// `Ping` 消息的处理器
/// Handler for `Ping` message
impl Handler<Ping> for MyActor {
    type Result = usize; // 处理结果类型

    // 处理 Ping 消息的方法
    fn handle(&mut self, msg: Ping, _: &mut Context<Self>) -> Self::Result {
        self.count += msg.0; // 将消息中的值加到计数器上
        self.count           // 返回当前计数器的值
    }
}

#[actix::main] // Actix 运行时宏
async fn main() {
    // 启动新的 actor，初始计数为 10
    // start new actor
    let addr = MyActor { count: 10 }.start();

    // 发送消息并获取结果的 future
    // send message and get future for result
    let res = addr.send(Ping(10)).await;

    // handle() 返回 tokio handle
    // 打印结果：10 + 10 = 20，所以应该输出 true
    // handle() returns tokio handle
    println!("RESULT: {}", res.unwrap() == 20);

    // 停止系统并退出
    // stop system and exit
    System::current().stop();
}