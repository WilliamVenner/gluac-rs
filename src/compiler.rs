use crate::{Bytecode, LuaError, Mutex, MutexGuard, lua::{self, LUA_GLOBALSINDEX, LuaString}, lua_string};

pub struct BytecodeCompiler(Mutex<lua::LuaState>);
impl BytecodeCompiler {
	pub(crate) unsafe fn new() -> Result<Self, LuaError> {
		let lua_state = lua::LuaState::new()?;

		// Push string.dump onto the stack
		lua_state.get_field(LUA_GLOBALSINDEX, lua_string!("string"));
		lua_state.get_field(-1, lua_string!("dump"));

		Ok(Self(Mutex::new(lua_state)))
	}

	#[cfg(feature = "parking_lot")]
	#[inline]
	fn lock(&self) -> Result<MutexGuard<'_, lua::LuaState>, LuaError> {
		Ok(self.0.lock())
	}

	#[cfg(not(feature = "parking_lot"))]
	#[inline]
	fn lock(&self) -> Result<MutexGuard<'_, lua::LuaState>, LuaError> {
		self.0.lock().map_err(|_| LuaError::PoisonError)
	}

	#[cfg(feature = "parking_lot")]
	#[inline]
	/// Returns if the Mutex guarding the underlying Lua state is currently locked.
	pub fn is_locked(&self) -> bool {
		self.0.is_locked()
	}

	#[cfg(not(feature = "parking_lot"))]
	/// Returns if the Mutex guarding the underlying Lua state is currently locked.
	///
	/// This is currently implemented using `std::sync::Mutex::try_lock()` and matching against `TryLockError::WouldBlock`
	pub fn is_locked(&self) -> bool {
		match self.0.try_lock() {
			Ok(_) => false,
			Err(err) => matches!(err, std::sync::TryLockError::WouldBlock)
		}
	}

	#[inline]
	unsafe fn precompile(&self, lua_state: lua::LuaState) {
		lua_state.push_value(-1); // Copy the string.dump reference onto the stack again (saves us getting it from _G every time)
	}

	unsafe fn compile(&self, lua_state: lua::LuaState, strip_debug: bool) -> Result<Bytecode, LuaError> {
		lua_state.push_boolean(strip_debug); // Push strip_debug argument onto the stack
		let lua_error_code = lua_state.pcall(2, 1, 0); // Call string.dump
		if lua_error_code == 0 {
			let mut bytecode = lua_state.get_binary_string(-1).unwrap().to_vec();
			bytecode.pop(); // Pop the final NUL terminator off the string
			Ok(bytecode)
		} else {
			Err(LuaError::from_lua_state(lua_state, lua_error_code))
		}
	}

	/// Loads a string of Lua source code into the Lua state and compiles it to bytecode.
	///
	/// This function takes a `LuaString` (basically just a `*const char` in C) - you can use the `gluac::lua_string!()` macro to create one.
	pub fn compile_string(&self, src: LuaString, strip_debug: bool) -> Result<Bytecode, LuaError> {
		let lua_state = self.lock()?;
		unsafe {
			self.precompile(*lua_state);
			lua_state.load_string(src)?;
			self.compile(*lua_state, strip_debug)
		}
	}

	/// Loads a file from its path into the Lua state and compiles it to bytecode.
	///
	/// This function takes a `LuaString` (basically just a `*const char` in C) - you can use the `gluac::lua_string!()` macro to create one.
	pub fn compile_file(&self, path: LuaString, strip_debug: bool) -> Result<Bytecode, LuaError> {
		let lua_state = self.lock()?;
		unsafe {
			self.precompile(*lua_state);
			lua_state.load_file(path)?;
			self.compile(*lua_state, strip_debug)
		}
	}
}
impl std::ops::Drop for BytecodeCompiler {
    fn drop(&mut self) {
        if let Ok(lua_state) = self.lock() {
			if !(*lua_state).is_null() {
				unsafe { lua_state.close(); }
			}
		}
    }
}