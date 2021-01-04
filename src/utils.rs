// this macro is shorthand for adding algorithms to to benchmark
// input: function pointer, string
// output: tuple (fn pointer, fn name, string)
#[macro_export] macro_rules! sfn {
	($f:expr, $c:literal) => {
		($f, (&Regex::new("::<.+>$").unwrap().replace(stringify!($f), "")).to_string(), $c)
	};
}

#[cfg(target_family = "unix")]
pub fn set_priority() {
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
pub fn set_priority() {
	winproc::Process::current().set_priority(winproc::PriorityClass::Realtime).unwrap();
	//winproc::Thread::current().set_priority(winproc::PriorityLevel::TimeCritical).unwrap();
}

// returns number with comma separators (i.e. 1000000 -> "1,000,000")
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

pub fn verify_sorted<T: Ord + std::fmt::Debug>(array: &[T]) {
	assert!(array.windows(2).all(|slice| slice[0] <= slice[1]), "{:?}", array);
}
