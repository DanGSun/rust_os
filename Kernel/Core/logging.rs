// Module: ::logging
//
//

use core::fmt::FormatWriter;

pub enum Level
{
	LevelPanic,	// Everything broke
	LevelError,	// Something broke
	LevelWarning,	// Recoverable
	LevelNotice,	// Odd
	LevelInfo,	// Interesting
	LevelLog,	// General
	LevelDebug,	// What
	LevelTrace,	// Where
}

struct LoggingFormatter
{
	lock_handle: ::arch::sync::HeldSpinlock<'static,()>,
}

// NOTE: Has to be a spinlock, stops interrupts while held
static s_logging_lock: ::arch::sync::Spinlock<()> = spinlock_init!( () );

impl ::core::fmt::Char for Level
{
	fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
		write!(f, "{:c}",
			match *self
			{
			LevelPanic   => 'k',
			LevelError   => 'e',
			LevelWarning => 'w',
			LevelNotice  => 'n',
			LevelInfo    => 'i',
			LevelLog     => 'l',
			LevelDebug   => 'd',
			LevelTrace   => 't',
			}
			)
	}
}

impl LoggingFormatter
{
	pub fn new() -> LoggingFormatter
	{
		LoggingFormatter {
			lock_handle: s_logging_lock.lock()
		}
	}
}

impl ::core::fmt::FormatWriter for LoggingFormatter
{
	fn write(&mut self, bytes: &[u8]) -> ::core::fmt::Result
	{
		match ::core::str::from_utf8(bytes)
		{
		::core::option::Some(s) => ::arch::puts(s),
		::core::option::None => {
			let rs = unsafe { ::core::mem::transmute::<_,::core::raw::Slice<u8>>(bytes) };
			panic!("LoggingFormatter.write bytes={}+{}", rs.data, rs.len);
			}
		}
		::core::result::Ok( () )
	}
}
impl ::core::ops::Drop for LoggingFormatter
{
	fn drop(&mut self)
	{
		::arch::puts("\n");
	}
}

pub fn enabled(_level: Level, _modname: &str) -> bool
{
	true
}

pub fn getstream(level: Level, modname: &str) -> LoggingFormatter
{
	assert!( enabled(level, modname) );
	let mut rv = LoggingFormatter::new();
	let _ = write!(&mut rv, "{:8u}{:c} [{:6s}] - ", ::time::ticks(), level, modname);
	rv
}


// vim: ft=rust
