#[allow(dead_code)]
pub fn execute() {
    let _home = IpAddrKind::V4(127, 0, 0, 1);
    let _loopback = IpAddrKind::V6(String::from("::1"));
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

/// An enum whose variants each store different amounts and types of values
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // includes an anonymous struct inside it
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[allow(dead_code)]
impl Message {
    fn call(&self) {
        // the method body would be defined here
    }
}

#[allow(dead_code)]
struct QuitMessage; // unit struct

#[allow(dead_code)]
struct MoveMessage {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
struct WriteMessage(String); // tuple struct

#[allow(dead_code)]
struct ChangeColorMessage(i32, i32, i32); // tuple struct
