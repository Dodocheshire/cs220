//! Small exercises
//!
//! Refer `small_exercises_grade.rs` for test cases

use std::sync::mpsc::{Receiver, RecvError, Sender};
use std::thread;

use etrace::*;

/// The "pong" function
///
/// Data will be sent and received through `rx` and `tx`.
/// Read the `test_ping_pong` function in `small_exercises_grade.rs` to figure out what it should
/// do.
pub fn pong(rx1: &mut Receiver<u32>, tx2: &mut Sender<u32>) -> bool {
    match rx1.recv() {
        Ok(val) => tx2.send(val + 1).is_ok(), // rx2 关闭接收端
        Err(_) => false,                      // tx1 已经销毁
    }
}

/// Executes the given functions (f1, f2) in concurrent and returns the results.
///
/// Read the `test_scoped_thread` function in `small_exercises_grade.rs` to figure out what it
/// should do.
// 作用域线程std::thread::scope 允许你在子线程中直接借用主线程中的局部变量，而不需要使用Arc或move所有权
// why？传统的spawn中编译器要求闭包捕获的所有数据必须是'static的，因为spawn出去的线程可能比当前函数活得更久，所以编译器不准你借用函数里的局部变量，怕发生“悬垂指针”，所以一般用Arc或move
// scope在代码里划定了一个“圈”。Rust 保证：在这个圈结束之前，所有在这个圈里创建的线程必须全部结束（Join）
// thread::scope函数接受一个闭包参数f，执行以下步骤
// 1. 创建一个Scope对象
// 2. 执行闭包f(s: &Scope)
// 3. 闭包中通过s.spawn创建线程，闭包执行完后，scope函数会查看s追踪的所有线程，并自动对它们调用.join()
// impl<'scope, 'env> Scope<'scope, 'env> {
//     pub fn spawn<F, T>(&'scope self, f: F) -> ScopedJoinHandle<'scope, T>
//     where
//         F: FnOnce() -> T + Send + 'scope,
//         T: Send + 'scope;
// }
// 4. 所有线程都结束后，scope函数才返回
// 通过Scope句柄可以追踪不同线程的借用行为，例如如果你在一个s.spawn中可变借用了x，那么在同一个scope的另一个s.spawn里就不能再借用x
// thread::scope的签名有两个生命周期'env and 'scope
// 1.'env: scope外部(之前)定义额度那些变量能活多久
// 2. 'scope: scope函数块运行时间——从开始执行到所有线程 Join 结束
// where F: FnOnce() -> T + 'scope中的F: 'scope表示内部捕获的所有引用，生命周期至少要能覆盖整个'scope
// 又因为scope函数保证在自己结束前，所有子线程必须先死，所以子线程的寿命 <= 'scope <= 'env
pub fn use_scoped_thread<'scope, T1, T2, F1, F2>(
    s: &'scope thread::Scope<'scope, '_>,
    f1: F1,
    f2: F2,
) -> (T1, T2)
where
    T1: Send + 'scope,
    T2: Send + 'scope,
    F1: Send + FnOnce() -> T1 + 'scope,
    F2: Send + FnOnce() -> T2 + 'scope,
{
    // spawn()特征约束F: FnOnce() -> T + Send + 'scope
    // Send:闭包F实际上是一个编译器自动生成的结构体，里面存着它捕获的所有变量。如果这个闭包要在另一个线程运行，它本身必须是Send的
    // F : 'scope 的形式是一种type bound： "Trait: 'a" 表示这个类型中包含的所有引用，其生命周期都必须大于或等于'a.
    // 编译器看到'scope,它会检查：“这个闭包里借用了(环境中的)变量X吗？如果有，变量X在整个'scope期间都活着吗？
    let handle1 = s.spawn(f1);
    let handle2 = s.spawn(f2);

    let res1 = handle1.join().unwrap();
    let res2 = handle2.join().unwrap();
    (res1, res2)
}

//
//e.g.
//fn leak_data() {
//     let x = 10;
//     // ❌ 编译器报错！
//     // 因为 thread::spawn 产生的线程可能会在 leak_data 函数结束后继续运行。
//     // 如果它继续运行，它引用的 x 就已经在大括号结束时被销毁了。
//     // 这就是所谓的线程“偷跑”出了数据的生命周期范围。
//     std::thread::spawn(|| {
//         println!("{}", x);
//     });
// }

// This does not work too!

// let mut container = Vec::new();

// thread::scope(|s| {
//     let handle = s.spawn(|| { ... });
//     // 尝试把 handle 存到 scope 外面的容器里
//     container.push(handle);
// });
