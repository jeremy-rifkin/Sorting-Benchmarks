// NOTE: this is only correct for host compiling, not cross-compiling
// cpuid only works on x86
// procfs only works on linux
#[cfg(target_os = "linux")]
use procfs;
// but don't actually need any cpu-specific stuff on windows now
//#[cfg(target_os = "windows")]
//use cpuid;

fn main() {
	println!("cargo:rustc-link-search=all=object/");
	//println!("cargo:rustc-flags=-l dylib=stdc++");
	//println!("cargo:rustc-flags=-l libgcc_s_seh-1");
	#[cfg(target_os = "linux")]
	{
		// this will have to be updated for every raspberry pi tested on
		if procfs::CpuInfo::new().unwrap().fields["model name"] == "ARMv6-compatible processor rev 7 (v6l)" {
			println!("cargo:rustc-cfg=arm11");
		}
	}
	//#[cfg(target_os = "windows")]
	//{}
}