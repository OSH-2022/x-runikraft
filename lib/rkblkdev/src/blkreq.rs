#![no_std]

type Sector = usize;
type Atomic = u32;

///用于向设备发送请求
pub struct RkBlkreq {
    //输入成员
    ///操作类型
    operation: RkBlkreqOp,
    ///操作开始的起始扇区
    start_sector: Sector,
    ///扇区数量的大小
    nb_sectors: Sector,
    ///指向数据的指针
    aio_buf: *mut u8,
    ///回复的请求的参数
    cb_cookie: *mut u8,
    //输出成员
    ///请求的状态：完成/未完成
    state: Atomic,
    ///操作状态的结果（错误返回负值）
    result: isize,
}

///操作状态
pub enum RkBlkreqState {
    RkBlkreqFinished(bool),
    RkBlkreqUnfinished(bool),
}

///支持的操作
pub enum RkBlkreqOp {
    ///读操作
    RkBlkreqRead(u8),
    ///写操作
    RkBlkreqWrite(u8),
    ///冲洗易变的写缓存
    RkBlkreqFflush(u8),
}

///用于执行一个响应后的请求
/// @参数 req
///
/// RkBlkreq结构体
///
/// @参数 cookie_callback
///
///	由用户在递交请求时设定的可选参数
///
fn rk_blkreq_ent_t(req: &RkBlkreq, cb_cookie: *mut u8) {
    todo!()
}

///初始化一个请求结构体
///
///@参数 req
///
///	请求结构
///
///@参数 op
///
///	操作
///
///@参数 start
///
///	起始扇区
///
///@参数  nb_sectors
///
///	扇区数量
///
///@参数 aio_buf
///
///	数据缓冲区
///
///@参数 cb_cookie
///
///	请求回复的参数
///
#[inline]
fn rk_blkreq_init(req: &RkBlkreq, op: RkBlkreqOp, start: Sector, nb_sectors: Sector, aio_buf: *mut u8, cb_cookie: *mut u8) {
    todo!()
}

/// 检查请求是否结束
/// @参数 req
///
/// RkBlkreq结构体
fn rk_blkreg_is_done(req: &RkBlkreq) -> bool {
    todo!()
}
