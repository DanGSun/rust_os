
/// Achitecture-specific code - AMD64 (aka x86-64)
#[macro_use]
#[cfg(arch="amd64")] #[path="amd64/mod.rs"]
#[doc(hidden)]
pub mod imp;	// Needs to be pub for exports to be avaliable

#[macro_use]
#[cfg(arch="armv7")] #[path="armv7/mod.rs"]
#[doc(hidden)]
pub mod imp;

// If on x86/amd64, import ACPI
#[cfg(arch="amd64")]
pub use self::imp::acpi;


pub mod memory {
	pub type PAddr = ::arch::imp::memory::PAddr;
	pub type VAddr = ::arch::imp::memory::VAddr;
	pub const PAGE_SIZE: usize = ::arch::imp::memory::PAGE_SIZE;

	pub mod addresses {
		use arch::imp::memory::addresses as imp;

		pub fn is_global(addr: usize) -> bool {
			imp::is_global(addr)
		}

		pub const STACK_SIZE: usize = imp::STACK_SIZE;

		pub const USER_END: usize = imp::USER_END;

		pub const STACKS_BASE: usize = imp::STACKS_BASE;
		pub const STACKS_END : usize = imp::STACKS_END ;

		pub const TEMP_BASE: usize = imp::TEMP_BASE;
		pub const TEMP_END : usize = imp::TEMP_END ;
		
		pub const HARDWARE_BASE: usize = imp::HARDWARE_BASE;
		pub const HARDWARE_END : usize = imp::HARDWARE_END ;

		pub const HEAP_START: usize = imp::HEAP_START;
		//pub const HEAP_END : usize = imp::HEAP_END ;
	}
	pub mod virt {
		use arch::imp::memory::virt as imp;
		
		pub type AddressSpace = imp::AddressSpace;

		pub fn get_phys<T>(p: *const T) -> ::memory::PAddr {
			imp::get_phys(p)
		}
		pub fn is_reserved<T>(p: *const T) -> bool {
			imp::is_reserved(p)
		}
		pub fn get_info<T>(p: *const T) -> Option<(::memory::PAddr,::memory::virt::ProtectionMode)> {
			imp::get_info(p)
		}

		pub fn is_fixed_alloc(addr: *const (), size: usize) -> bool {
			imp::is_fixed_alloc(addr, size)
		}
		pub unsafe fn fixed_alloc(p: ::memory::PAddr, count: usize) -> Option<*mut ()> {
			imp::fixed_alloc(p, count)
		}

		pub unsafe fn map(a: *mut (), p: ::memory::PAddr, mode: ::memory::virt::ProtectionMode) {
			imp::map(a, p, mode)
		}
		pub unsafe fn reprotect(a: *mut (), mode: ::memory::virt::ProtectionMode) {
			imp::reprotect(a, mode)
		}
		pub unsafe fn unmap(a: *mut ()) -> Option<::memory::PAddr> {
			imp::unmap(a)
		}
	}
	pub mod phys {
		use arch::imp::memory::phys as imp;

		pub fn ref_frame(frame_idx: u64) {
			imp::ref_frame(frame_idx)
		}
		pub fn deref_frame(frame_idx: u64) -> u32 {
			imp::deref_frame(frame_idx)
		}
		pub fn get_multiref_count(frame_idx: u64) -> u32 {
			imp::get_multiref_count(frame_idx)
		}

		pub fn mark_free(frame_idx: u64) -> bool {
			imp::mark_free(frame_idx)
		}
		pub fn mark_used(frame_idx: u64) {
			imp::mark_used(frame_idx)
		}
	}
}

pub mod sync {
	use super::imp::sync as imp;
	pub type Spinlock<T> = imp::Spinlock<T>;
	pub type HeldSpinlock<'a, T: 'a> = imp::HeldSpinlock<'a, T>;
	pub type HeldInterrupts = imp::HeldInterrupts;

	pub fn hold_interrupts() -> HeldInterrupts {
		imp::hold_interrupts()
	}
}
pub mod interrupts {
	use super::imp::interrupts as imp;

	pub type BindError = imp::BindError;
	pub type IRQHandle = imp::IRQHandle;

	
	pub fn bind_gsi(gsi: usize, handler: fn(*const()), info: *const ()) -> Result<IRQHandle, BindError> {
		imp::bind_gsi(gsi, handler, info)
	}
}
pub mod boot {
	use super::imp::boot as imp;

	pub fn get_boot_string() -> &'static str {
		imp::get_boot_string()
	}
	
	pub fn get_video_mode() -> Option<::metadevs::video::bootvideo::VideoMode> {
		imp::get_video_mode()
	}
	pub fn get_memory_map() -> &'static [::memory::MemoryMapEnt] {
		imp::get_memory_map()
	}
}
pub mod pci {
	use super::imp::pci as imp;

	pub fn read(a: u32) -> u32 {
		imp::read(a)
	}
	pub fn write(a: u32, v: u32) {
		imp::write(a, v)
	}
}
pub mod threads {
	use lib::mem::Box;
	use super::imp::threads as imp;

	pub type State = imp::State;
	pub fn init_tid0_state() -> State {
		imp::init_tid0_state()
	}

	pub fn set_thread_ptr(t: Box<::threads::Thread>) {
		imp::set_thread_ptr(t)
	}
	pub fn get_thread_ptr() -> Option<Box<::threads::Thread>> {
		imp::get_thread_ptr()
	}
	pub fn borrow_thread() -> *const ::threads::Thread {
		imp::borrow_thread()
	}

	pub fn idle() {
		imp::idle()
	}
	pub fn switch_to(t: Box<::threads::Thread>) {
		imp::switch_to(t)
	}

	pub fn start_thread<F: FnOnce()+Send>(thread: &mut ::threads::Thread, code: F) {
		imp::start_thread(thread, code)
	}
}

pub mod x86_io {
	use super::imp::x86_io as imp;

	pub unsafe fn inb(p: u16) -> u8 { imp::inb(p) }
	pub unsafe fn inw(p: u16) -> u16 { imp::inw(p) }
	pub unsafe fn inl(p: u16) -> u32 { imp::inl(p) }
	pub unsafe fn outb(p: u16, v: u8) { imp::outb(p, v) }
	pub unsafe fn outw(p: u16, v: u16) { imp::outw(p, v) }
	pub unsafe fn outl(p: u16, v: u32) { imp::outl(p, v) }
}


#[inline]
pub fn puts(s: &str) {
	imp::puts(s);
}
#[inline]
pub fn puth(v: u64) {
	imp::puth(v)
}

pub fn cur_timestamp() -> u64 {
	imp::cur_timestamp()
}
pub fn print_backtrace() {
	imp::print_backtrace()
}

pub unsafe fn drop_to_user(entry: usize, stack: usize, args_len: usize) -> ! {
	imp::drop_to_user(entry, stack, args_len)
}

