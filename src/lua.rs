use libloading::{Library, Symbol};

use crate::LuaError;

pub type LuaInt = std::os::raw::c_int;
pub type LuaSize = usize;
pub type LuaString = *const std::os::raw::c_char;

pub const LUA_GLOBALSINDEX: LuaInt = -10002;

pub const LUA_ERRRUN: LuaInt = 2;
pub const LUA_ERRSYNTAX: LuaInt = 3;
pub const LUA_ERRMEM: LuaInt = 4;
pub const LUA_ERRERR: LuaInt = 5;
pub const LUA_ERRFILE: LuaInt = 6;

impl LuaError {
	fn get_error_message(lua_state: LuaState) -> Option<String> {
		unsafe { lua_state.get_string(-1).map(|str| str.into_owned()) }
	}

    pub(crate) fn from_lua_state(lua_state: LuaState, lua_int_error_code: LuaInt) -> Self {
		use LuaError::*;
        match lua_int_error_code {
			LUA_ERRMEM => MemoryAllocationError,
			LUA_ERRERR => ErrorHandlerError,
			LUA_ERRSYNTAX | LUA_ERRRUN | LUA_ERRFILE => {
				let msg = LuaError::get_error_message(lua_state);
				match lua_int_error_code {
					LUA_ERRSYNTAX => SyntaxError(msg),
					LUA_ERRRUN => RuntimeError(msg),
					LUA_ERRFILE => FileError(msg),
					_ => unreachable!()
				}
			},
			_ => Unknown(lua_int_error_code)
		}
    }
}

lazy_static::lazy_static! {
	static ref LUA_SHARED: LuaShared = LuaShared::import();
}
struct LuaShared {
	lual_newstate: Symbol<'static, unsafe extern "C" fn() -> LuaState>,
	lual_openlibs: Symbol<'static, unsafe extern "C" fn(state: LuaState)>,
	lual_loadfile: Symbol<'static, unsafe extern "C" fn(state: LuaState, path: LuaString) -> LuaInt>,
	lual_loadstring: Symbol<'static, unsafe extern "C" fn(state: LuaState, path: LuaString) -> LuaInt>,
	lua_getfield: Symbol<'static, unsafe extern "C" fn(state: LuaState, index: LuaInt, k: LuaString)>,
	lua_pushvalue: Symbol<'static, unsafe extern "C" fn(state: LuaState, index: LuaInt)>,
	lua_pushboolean: Symbol<'static, unsafe extern "C" fn(state: LuaState, bool: LuaInt)>,
	lua_tolstring: Symbol<'static, unsafe extern "C" fn(state: LuaState, index: LuaInt, out_size: *mut LuaSize) -> LuaString>,
	lua_pcall: Symbol<'static, unsafe extern "C" fn(state: LuaState, nargs: LuaInt, nresults: LuaInt, errfunc: LuaInt) -> LuaInt>,
	lua_close: Symbol<'static, unsafe extern "C" fn(state: LuaState)>,
}
impl LuaShared {
	fn import() -> Self {
		unsafe {
			let library = Self::find_library();
			let library = Box::leak(Box::new(library)); // Keep this library referenced forever

			macro_rules! find_symbol {
				( $symbol:literal ) => {
					Self::find_symbol(library, concat!($symbol, "\0").as_bytes())
				};
			}

			Self {
				lual_newstate: find_symbol!("luaL_newstate"),
				lual_openlibs: find_symbol!("luaL_openlibs"),
				lual_loadfile: find_symbol!("luaL_loadfile"),
				lual_loadstring: find_symbol!("luaL_loadstring"),
				lua_getfield: find_symbol!("lua_getfield"),
				lua_pushvalue: find_symbol!("lua_pushvalue"),
				lua_pushboolean: find_symbol!("lua_pushboolean"),
				lua_tolstring: find_symbol!("lua_tolstring"),
				lua_pcall: find_symbol!("lua_pcall"),
				lua_close: find_symbol!("lua_close"),
			}
		}
	}

	unsafe fn find_symbol<T>(library: &'static Library, name: &[u8]) -> Symbol<'static, T> {
		match library.get(name) {
			Ok(symbol) => symbol,
			Err(err) => panic!("Failed to find symbol \"{}\"\n{:#?}", String::from_utf8_lossy(name), err)
		}
	}

	unsafe fn find_library() -> Library {
		#[cfg(target_os = "windows")]
		let result = Library::new("lua_shared.dll");

		#[cfg(not(target_os = "windows"))]
		let result = Library::new("lua_shared_srv.so").or_else(|_| Library::new("lua_shared.so"));

		match result {
			Ok(library) => library,
			Err(error) => {
				#[cfg(target_os = "windows")]
				eprintln!("Failed to load lua_shared.dll, tier0.dll or vstdlib.dll!");

				#[cfg(not(target_os = "windows"))]
				eprintln!("Failed to load lua_shared_srv.so/lua_shared.so, libtier0_srv.so/libtier0.so or libvstdlib_srv.so/libvstdlib.so!");

				#[cfg(target_pointer_width = "32")]
				eprintln!("Make sure you are using the 32-bit module binaries from the 32-bit branch of Garry's Mod.");

				#[cfg(target_pointer_width = "64")]
				eprintln!("Make sure you are using the 64-bit module binaries from the 64-bit branch of Garry's Mod.");

				#[cfg(not(target_os = "windows"))]
				eprintln!("The binaries must be placed in the same directory as the executable, or be in the system's PATH.");

				#[cfg(not(target_os = "linux"))]
				eprintln!("You may need to add the directory of the current executable to the LD_LIBRARY_PATH environment variable.");

				eprintln!("Executable path: {:?}", std::env::current_exe().ok());

				panic!("{:#?}", error);
			}
		}
	}
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct LuaState(*const std::ffi::c_void);
unsafe impl Send for LuaState {}
impl LuaState {
	pub(crate) unsafe fn new() -> Result<Self, LuaError> {
		let lua = (LUA_SHARED.lual_newstate)();
		(LUA_SHARED.lual_openlibs)(lua);
		if lua.0.is_null() {
			Err(LuaError::MemoryAllocationError)
		} else {
			Ok(lua)
		}
	}

	#[inline]
	pub(crate) unsafe fn push_value(&self, index: LuaInt) {
		(LUA_SHARED.lua_pushvalue)(*self, index)
	}

	#[inline]
	pub(crate) unsafe fn get_field(&self, index: LuaInt, k: LuaString) {
		(LUA_SHARED.lua_getfield)(*self, index, k)
	}

	#[inline]
	pub(crate) unsafe fn push_boolean(&self, boolean: bool) {
		(LUA_SHARED.lua_pushboolean)(*self, if boolean { 1 } else { 0 })
	}

	#[inline]
	pub(crate) unsafe fn pcall(&self, nargs: LuaInt, nresults: LuaInt, errfunc: LuaInt) -> LuaInt {
		(LUA_SHARED.lua_pcall)(*self, nargs, nresults, errfunc)
	}

	pub(crate) unsafe fn get_binary_string(&self, index: LuaInt) -> Option<&[u8]> {
		let mut len: usize = 0;
		let ptr = (LUA_SHARED.lua_tolstring)(*self, index, &mut len);

		if ptr.is_null() {
			return None;
		}

		Some(std::slice::from_raw_parts(ptr as *const u8, len))
	}

	pub(crate) unsafe fn get_string(&self, index: LuaInt) -> Option<std::borrow::Cow<'_, str>> {
		let mut len: usize = 0;
		let ptr = (LUA_SHARED.lua_tolstring)(*self, index, &mut len);

		if ptr.is_null() {
			return None;
		}

		Some(String::from_utf8_lossy(std::slice::from_raw_parts(ptr as *const u8, len)))
	}

	pub(crate) unsafe fn load_string(&self, src: LuaString) -> Result<(), LuaError> {
		let lua_error_code = (LUA_SHARED.lual_loadstring)(*self, src);
		if lua_error_code == 0 {
			Ok(())
		} else {
			Err(LuaError::from_lua_state(*self, lua_error_code))
		}
	}

	pub(crate) unsafe fn load_file(&self, path: LuaString) -> Result<(), LuaError> {
		let lua_error_code = (LUA_SHARED.lual_loadfile)(*self, path);
		if lua_error_code == 0 {
			Ok(())
		} else {
			Err(LuaError::from_lua_state(*self, lua_error_code))
		}
	}

	#[inline]
	pub(crate) unsafe fn close(&self) {
		(LUA_SHARED.lua_close)(*self)
	}
}
impl std::ops::Deref for LuaState {
    type Target = *const std::ffi::c_void;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
