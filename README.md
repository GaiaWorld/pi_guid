# pi_guid

pi_guid 生成 96 位的全局唯一标识。一个 Guid 由三个字段组成：

- time_ns: u64：Unix 时间戳纳秒值，用于保证同一个生成器生成的时间不重复。
- node_id: u16：节点编号，用于区分不同节点。
- ctrl_id: u16：控制编号，可用于区分业务控制域、管理哈希或进行一致性哈希。

节点启动时间参数 node_starttime_sec 的单位是 Unix 秒。传入 0 时，生成器会自动读取当前节点的启动时间。节点重启后，建议递增 ctrl_id，避免重启造成的时间序列与旧实例冲突。

## 安装

在项目的 Cargo.toml 中添加依赖：

    [dependencies]
    pi_guid = "0.2"

## 快速开始

    use pi_guid::GuidGen;

    let generator = GuidGen::new(0, 1, 0);
    let guid = generator.gen();

    println!("time_ns = {}", guid.time_ns());
    println!("node_id = {}", guid.node_id());
    println!("ctrl_id = {}", guid.ctrl_id());

## 指定控制编号

使用 gen_by_ctrl_id 可以在不创建新生成器的情况下，为单个 Guid 指定控制编号：

    use pi_guid::GuidGen;

    let generator = GuidGen::new(0, 1, 10);
    let guid = generator.gen_by_ctrl_id(20);
    assert_eq!(guid.node_id(), 1);
    assert_eq!(guid.ctrl_id(), 20);

## 多线程使用

GuidGen 内部使用原子计数器，可以通过 Arc 在多个线程之间共享同一个生成器：

    use std::sync::Arc;
    use std::thread;
    use pi_guid::GuidGen;

    let generator = Arc::new(GuidGen::new(0, 1, 0));
    let handles: Vec<_> = (0..4).map(|_| {
        let generator = Arc::clone(&generator);
        thread::spawn(move || generator.gen())
    }).collect();

    for handle in handles {
        let _guid = handle.join().expect("worker thread panicked");
    }

## API 概览

- Guid::time_ns()：读取 Unix 时间戳纳秒值。
- Guid::node_id()：读取节点编号。
- Guid::ctrl_id()：读取控制编号。
- GuidGen::new(node_starttime_sec, node_id, ctrl_id)：创建生成器。
- GuidGen::node_starttime_ns()：读取节点启动时间的纳秒表示。
- GuidGen::node_id()：读取生成器的节点编号。
- GuidGen::ctrl_id()：读取生成器的默认控制编号。
- GuidGen::unique_runtime_ns()：分配一个不重复的运行时间纳秒值。
- GuidGen::gen()：使用默认控制编号生成 Guid。
- GuidGen::gen_by_ctrl_id(ctrl_id)：使用指定控制编号生成 Guid。
