use std::time::Duration;

// this macro is shorthand for adding algorithms to to benchmark
// input: function pointer, string
// output: tuple (fn pointer, fn name, string)
#[macro_export] macro_rules! sfn {
	($f:expr, $c:literal) => {
		($f, (&Regex::new("::<.+>$").unwrap().replace(stringify!($f), "")).to_string(), $c)
	};
}

// this macro assists with extending the lifetime of an object
// it is short for extend_lifetime(LifetimeContainer(obj)).0
pub struct LifetimeContainer<'a, T>(pub &'a T);
#[cfg(not(tarpaulin_include))]
pub unsafe fn extend_lifetime<'b, T>(lc: LifetimeContainer<'b, T>) -> LifetimeContainer<'static, T> {
	std::mem::transmute::<LifetimeContainer<'b, T>, LifetimeContainer<'static, T>>(lc)
}
#[macro_export] macro_rules! u_extend_lifetime {
	($f:expr) => {
		utils::extend_lifetime(utils::LifetimeContainer($f)).0
	};
}

#[cfg(target_family = "unix")]
#[cfg(not(tarpaulin_include))]
#[allow(dead_code)]
pub fn set_thread_priority_max() {
	thread_priority::unix::set_current_thread_priority(thread_priority::ThreadPriority::Max).unwrap();
	// TODO: investigate permission requirements of reltime static priorities
	// https://man7.org/linux/man-pages/man3/pthread_setschedparam.3.html #NOTES
	// https://man7.org/linux/man-pages/man7/sched.7.html #Privileges and resource limits
	//thread_priority::unix::set_thread_priority_and_policy(
	//	thread_priority::Thread::current().unwrap().id,
	//	thread_priority::ThreadPriority::Max,
	//	thread_priority::unix::ThreadSchedulePolicy::Realtime(
	//		thread_priority::unix::RealtimeThreadSchedulePolicy::Fifo
	//	)
	//).unwrap();
}

#[cfg(target_family = "windows")]
#[cfg(not(tarpaulin_include))]
#[allow(dead_code)]
pub fn set_thread_priority_max() {
	winproc::Thread::current().set_priority(winproc::PriorityLevel::TimeCritical).unwrap();
}

// returns number with comma separators (i.e. 1000000 -> "1,000,000")
// note: this method fails for num == 0 - it prints "x". I'm leaving it for now.
pub fn commafy(mut num: usize) -> String {
	let log = (num as f64).log10() as usize;
	let len = log + log / 3 + 1;
	let mut s = vec![b'x'; len];
	let mut i = 0;
	let mut count = 0;
	while num > 0 {
		if count > 0 && count % 3 == 0 {
			s[len - i - 1] = b',';
			i += 1;
		}
		s[len - i - 1] = b'0' + (num % 10) as u8;
		i += 1;
		num /= 10;
		count += 1;
	}
	return String::from_utf8(s).unwrap();
}

pub fn duration_to_human(d: Duration) -> String {
	let ns = d.as_nanos() as f64;
	if ns < 1e3 {
		format!("{:_>6.2}ns", ns)
	} else if ns < 1e6 {
		format!("{:_>6.2}μs", ns / 1e3)
	} else if ns < 1e9 {
		format!("{:_>6.2}ms", ns / 1e6)
	} else {
		let seconds = ns / 1e9;
		if seconds < 60.0 {
			format!("{:_>5.2}s", seconds)
		} else if seconds < 60.0 * 60.0 {
			format!("{:_>2}m {:_>5.2}s", (seconds / 60.0).floor(), seconds % 60.0)
		} else {
			format!("{}h {:_>2}m {:_>5.2}s",
				(seconds / (60.0 * 60.0)).floor(),
				((seconds % (60.0 * 60.0)) / 60.0).floor(),
				(seconds % (60.0 * 60.0)) % 60.0)
		}
	}
}

pub fn verify_sorted<T: Ord + std::fmt::Debug>(array: &[T]) {
	if array.len() <= 1_000 {
		assert!(array.windows(2).all(|slice| slice[0] <= slice[1]), "{:?}", array);
	} else {
		assert!(array.windows(2).all(|slice| slice[0] <= slice[1]),
					"large array failing (size = {})", array.len());
	}
}

pub fn fmin<T: PartialOrd>(f0: T, f1: T) -> T {
	// assuming neither is NaN
	if f0 < f1 {
		f0
	} else {
		f1
	}
}

pub fn fmax<T: PartialOrd>(f0: T, f1: T) -> T {
	// assuming neither is NaN
	if f0 < f1 {
		f1
	} else {
		f0
	}
}
