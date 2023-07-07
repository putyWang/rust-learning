fn main() {
    // 使用 枚举名::枚举值 获取指定枚举值
    // v4 赋值与 127, 0, 0, 1 绑定
    // v6 赋值与 String::from("::1") 绑定
    let v4:IpAddrKind = IpAddrKind::V4(127, 0, 0, 1);
    let v6:IpAddrKind = IpAddrKind::V6(String::from("::1"));

}

// 定义具有 V4 与 V6 两个不同值的 IpAddrKind 枚举
enum IpAddrKind {
    // 成员 V4 绑定 4个 u8 类型的数据
    V4(u8, u8, u8, u8),
    // 成员 V6 绑定 String 类型的数据
    V6(String)
}
