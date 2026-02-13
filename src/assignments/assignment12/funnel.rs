//! Funnel
//!
//! Spawn a thread that executes a funnel.
//!
//! Funnel will receive data from multiple receivers and send it to a single sender. Also, the
//! funnel will filter out data that does not pass the filter function.
//!
//! Refer to `funnel_grade.rs` for test cases.

use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

/// Spawn a thread that concurrently receive datas from `rxs`, send it to `tx` if it makes `f` true.
/// Returns its handle.
pub fn spawn_funnel<T, F>(rxs: Vec<Receiver<T>>, tx: Sender<T>, f: F) -> JoinHandle<()>
where
    T: Send + 'static,
    F: Send + Sync + Fn(&T) -> bool + 'static,
{
    // hard
    // 产生主管理线程并返回它的JoinHandle
    thread::spawn(move || {
        // 将filter包装到Arc中，以便在多个子线程中共享。由于F时Sync的，所以Arc<F>可以在多个线程中同时调用
        let filter = Arc::new(f);
        // 创建作用域，确保在此作用域产生的线程在作用域结束前全部Join
        thread::scope(|s| {
            for rx in rxs {
                // 为每个receiver 克隆一个Sender(都发送到同一个tx的对端)，和一个filter(再创建一个Arc强引用同一个f)
                let tx_clone = tx.clone();
                let f_shared = Arc::clone(&filter);

                // 为每个接收端rx开启一个并发的子线程
                let _unused = s.spawn(move || {
                    for val in rx {
                        // 该通道一有值来，做过滤再通过tx_clone发送
                        if f_shared(&val) {
                            if tx_clone.send(val).is_err() {
                                break; // 如果目标接收端关了，直接退出
                            }
                        }
                    }
                });
            }
        });
    }) // parameter tx drops here
}
