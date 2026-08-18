
// #![feature(integer_atomics)]

use std::sync::atomic::{AtomicU64, Ordering};

use pi_time::{run_nanos, start_secs};
use pi_null::Null;

/// 96 位全局唯一标识，由时间、节点和控制编号组成。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Guid(u64, u16, u16);

impl Guid {
	/// 返回从 Unix 纪元开始计算的纳秒时间戳。
	#[inline]
	pub fn time_ns(&self) -> u64 {
		self.0
	}
	#[inline]
	/// 返回生成此标识的节点编号。
	pub fn node_id(&self) -> u16 {
		self.1
	}
	#[inline]
	/// 返回此标识使用的控制编号。
	pub fn ctrl_id(&self) -> u16 {
		self.2
	}
}
impl Null for Guid {
    #[inline(always)]
    fn null() -> Self {
        Guid(0, 0, 0)
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.0 == 0
    }
}
/// 生成 Guid 的线程安全生成器。
#[derive(Debug)]
pub struct GuidGen {
	runtime_ns: AtomicU64,
	node_starttime_ns: u64,
	node_id: u16,
	ctrl_id: u16,
}

impl Default for GuidGen {
	fn default() -> Self {
		Self::new(0, 0, 0)
	}
}

impl GuidGen {
	/// 创建一个 Guid 生成器。
	///
	/// node_starttime_sec 使用 Unix 秒表示；传入 0 会自动读取节点启动时间。
	///
	/// # Examples
	///
	///     use pi_guid::GuidGen;
	///     let generator = GuidGen::new(0, 1, 2);
	///     assert_eq!(generator.node_id(), 1);
	///     assert_eq!(generator.ctrl_id(), 2);
	pub fn new(node_starttime_sec: u64, node_id: u16, ctrl_id: u16) -> Self {
		let sec = if node_starttime_sec == 0 {
			start_secs()
		} else {
			node_starttime_sec
		};
		GuidGen {
			runtime_ns: AtomicU64::new(run_nanos()),
			node_starttime_ns: sec * 1000_000_000,
			node_id,
			ctrl_id,
		}
	}
	/// 返回节点启动时间的 Unix 纳秒时间戳。
	pub fn node_starttime_ns(&self) -> u64 {
		self.node_starttime_ns
	}
	/// 返回生成器的节点编号。
	pub fn node_id(&self) -> u16 {
		self.node_id
	}
	/// 返回生成器的默认控制编号。
	pub fn ctrl_id(&self) -> u16 {
		self.ctrl_id
	}
	/// 分配一个不重复的运行时间纳秒值。
	#[inline]
	pub fn unique_runtime_ns(&self) -> u64 {
		let now = run_nanos();
		loop {
			let t = self.runtime_ns.load(Ordering::Relaxed);
			if t < now {
				match self.runtime_ns.compare_exchange(t, now, Ordering::SeqCst, Ordering::Relaxed) {
					Ok(_) => return now,
					Err(_) => ()
				}
			}else {
				return self.runtime_ns.fetch_add(1, Ordering::SeqCst) + 1
			}
		}
	}
	/// 使用生成器的默认控制编号生成一个 Guid。
	///
	/// # Examples
	///
	///     use pi_guid::GuidGen;
	///     let generator = GuidGen::new(0, 7, 9);
	///     let guid = generator.gen();
	///     assert_eq!(guid.node_id(), 7);
	///     assert_eq!(guid.ctrl_id(), 9);
	#[inline]
	pub fn gen(&self) -> Guid {
		Guid(self.unique_runtime_ns() + self.node_starttime_ns, self.node_id, self.ctrl_id)
	}
	/// 使用指定的控制编号生成一个 Guid，不改变生成器的默认控制编号。
	///
	/// # Examples
	///
	///     use pi_guid::GuidGen;
	///     let generator = GuidGen::new(0, 7, 9);
	///     let guid = generator.gen_by_ctrl_id(12);
	///     assert_eq!(guid.node_id(), 7);
	///     assert_eq!(guid.ctrl_id(), 12);
	#[inline]
	pub fn gen_by_ctrl_id(&self, ctrl_id: u16) -> Guid {
		Guid(self.unique_runtime_ns() + self.node_starttime_ns, self.node_id, ctrl_id)
	}
}



#[test]
	fn test_guid() {
		use std::collections::HashMap;
		let guid = GuidGen::new(0, 0, 0);
		
		let mut map = HashMap::new();
		let mut i = 1000000;
		while i > 0 {
			let uuid = guid.gen().0;
			map.insert(uuid, "");
			i = i - 1;
		}
		assert_eq!(map.len(), 1000000);

	}

#[test]
fn test_guid_gen_default() {
	let generator = GuidGen::default();

	assert_eq!(generator.node_id(), 0);
	assert_eq!(generator.ctrl_id(), 0);
	let guid = generator.gen();
	assert_eq!(guid.node_id(), 0);
	assert_eq!(guid.ctrl_id(), 0);
}
