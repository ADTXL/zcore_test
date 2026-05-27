// Cache and timer utilities - architecture specific
use core::arch::asm;

const L1_CACHE_BYTES: u64 = 64;
const CACHE_LINE_SIZE: u64 = 64;

pub fn flush_cache(addr: u64, size: u64) {
    flush_dcache_range(addr, addr + size);
}

pub fn invalidate_dcache(addr: u64, size: u64) {
    invalidate_dcache_range(addr, addr + size);
}

cfg_if::cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        // clean+invalidate data cache by virtual address
        pub fn flush_dcache_range(start: u64, end: u64) {
            let end = (end + (CACHE_LINE_SIZE - 1)) & !(CACHE_LINE_SIZE - 1);
            let mut i: u64 = start & !(L1_CACHE_BYTES - 1);
            while i < end {
                unsafe {
                    asm!("dc civac, {}", in(reg) i);
                }
                i += L1_CACHE_BYTES;
            }
            unsafe {
                asm!("dsb sy");
            }
        }

        // invalidate data cache by virtual address
        pub fn invalidate_dcache_range(start: u64, end: u64) {
            let end = (end + (CACHE_LINE_SIZE - 1)) & !(CACHE_LINE_SIZE - 1);
            let mut i: u64 = start & !(L1_CACHE_BYTES - 1);
            while i < end {
                unsafe {
                    asm!("dc ivac, {}", in(reg) i);
                }
                i += L1_CACHE_BYTES;
            }
            unsafe {
                asm!("dsb sy");
            }
        }

        pub fn fence_w() {
            unsafe {
                asm!("dmb osh");
            }
        }

        pub fn get_cycle() -> u64 {
            let val: u64;
            unsafe {
                asm!("mrs {}, cntvct_el0", out(reg) val);
            }
            val
        }

        // 微秒(us)
        pub fn usdelay(us: u64) {
            let freq: u64;
            unsafe {
                asm!("mrs {}, cntfrq_el0", out(reg) freq);
            }
            let t1 = get_cycle();
            let ticks = us * freq / 1_000_000;
            let t2 = t1 + ticks;
            while get_cycle() < t2 {}
        }
    } else {
        // Fallback implementations for other architectures (libos mode)
        pub fn flush_dcache_range(_start: u64, _end: u64) {
            // No-op in libos mode
        }

        pub fn invalidate_dcache_range(_start: u64, _end: u64) {
            // No-op in libos mode
        }

        pub fn fence_w() {
            // No-op in libos mode
        }

        pub fn get_cycle() -> u64 {
            // Use std time in libos mode
            #[cfg(feature = "libos")]
            {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos() as u64
            }
            #[cfg(not(feature = "libos"))]
            {
                0
            }
        }

        pub fn usdelay(us: u64) {
            #[cfg(feature = "libos")]
            {
                std::thread::sleep(std::time::Duration::from_micros(us));
            }
            #[cfg(not(feature = "libos"))]
            {
                let _ = us;
            }
        }
    }
}

// 毫秒(ms)
#[allow(unused)]
pub fn msdelay(ms: u64) {
    usdelay(ms * 1000);
}
