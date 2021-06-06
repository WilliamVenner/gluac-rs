use crate::{compiler::BytecodeCompiler, lua::LuaInt};

pub type Bytecode = Vec<u8>;

#[derive(Debug, Clone)]
pub enum LuaError {
	/// Out of memory
	///
	/// `LUA_ERRMEM`
	MemoryAllocationError,

	/// A syntax error occurred in the passed Lua source code.
	///
	/// `LUA_ERRSYNTAX`
	SyntaxError(Option<String>),

	/// Lua failed to load the given file
	///
	/// `LUA_ERRFILE`
	FileError(Option<String>),

	/// A runtime error occurred while compiling bytecode.
	///
	/// `LUA_ERRRUN`
	RuntimeError(Option<String>),

	/// An error occurred while running the error handler function.
	///
	/// `LUA_ERRERR`
	ErrorHandlerError,

	/// Unknown Lua error code
	Unknown(LuaInt),

	#[cfg(not(feature = "parking_lot"))]
	/// The Mutex guarding the Lua state is poisoned by a panic in another thread.
	PoisonError,
}

/// Creates a new bytecode compiler instance.
///
/// When dropped, it will close the Lua state and free any used dynamic memory.
///
/// ## Thread safety
/// The bytecode compiler instance's Lua state is locked behind a Mutex to ensure concurrency safety.
pub fn compiler() -> Result<BytecodeCompiler, LuaError> {
	unsafe { BytecodeCompiler::new() }
}

/// Converts a string literal to a Lua-compatible NUL terminated `CString`.
///
/// Also can convert a `String` or `&str` to a Lua-compatible NUL terminated `CString`.
///
/// **You must not add any NUL bytes into this string yourself.**
#[macro_export]
macro_rules! lua_string {
	( $str:literal ) => {
		#[allow(unused_unsafe)]
		unsafe {
			std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($str, "\0").as_bytes()).as_ptr()
		}
	};

	( $str:expr ) => {
		std::ffi::CString::new($str)
			.expect("Tried to create a Lua string from a string that contained a NUL byte (\\0)!")
			.as_ptr()
	};
}
