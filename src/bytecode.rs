#[derive(Debug)]
pub enum ByteCode {  // 中间代码
    GetGlobal(u8, u8), // 获取全局变量, 参数1: 栈位置, 参数2: 常量表索引
    LoadConstant(u8, u8), // 加载常量，参数1: 栈位置, 参数2: 常量表索引
    LoadNil(u8), // 加载nil，参数1: 栈位置
    LoadBool(u8, bool), // 加载bool，参数1: 栈位置, 参数2: 布尔值
    LoadInt(u8, i16), // 加载整数，参数1: 栈位置, 参数2: 整数值

    SetGlobal(u8, u8),  // global = local
    SetGlobalConst(u8, u8),  // global = const
    SetGlobalGlobal(u8, u8),  // global = global

    Move(u8, u8),

    Call(u8, u8), // 调用函数
}