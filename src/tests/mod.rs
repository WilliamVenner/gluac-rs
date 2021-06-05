#[cfg(test)]
fn check_stack(compiler: crate::compiler::BytecodeCompiler) {
	assert_eq!(compiler.stack_size(), 3);
	assert_eq!(compiler.get_type(compiler.stack_size()), "function");
}

#[test]
fn hello_world() {
	let compiler = crate::compiler().unwrap();

	let bytecode = compiler.compile_string(lua_string!(r#"print("Hello, world!")"#), true).unwrap();
	assert_eq!(bytecode, [27, 76, 74, 2, 10, 43, 2, 0, 3, 0, 2, 0, 4, 54, 0, 0, 0, 39, 2, 1, 0, 66, 0, 2, 1, 75, 0, 1, 0, 18, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33, 10, 112, 114, 105, 110, 116]);

	check_stack(compiler);
}

#[test]
fn hello_world_file() {
	let compiler = crate::compiler().unwrap();

	let bytecode = compiler.compile_file(lua_string!(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/tests/hello_world.lua").to_string_lossy().to_string()), true).unwrap();
	assert_eq!(bytecode, [27, 76, 74, 2, 10, 43, 2, 0, 3, 0, 2, 0, 4, 54, 0, 0, 0, 39, 2, 1, 0, 66, 0, 2, 1, 75, 0, 1, 0, 18, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33, 10, 112, 114, 105, 110, 116]);

	check_stack(compiler);
}

#[test]
fn syntax_error() {
	let compiler = crate::compiler().unwrap();

	let result = compiler.compile_string(lua_string!(r#"Invalid Lua code"#), true);
	assert!(match result {
		Ok(_) => false,
		Err(_error) => matches!(crate::LuaError::SyntaxError(Some(r#"[string "Invalid Lua code"]:1: '=' expected near 'Lua'"#.to_string())), _error),
	});

	check_stack(compiler);
}

#[test]
fn multiple_compilations() {
	let compiler = crate::compiler().unwrap();

	for _ in 1..10 {
		let bytecode = compiler.compile_string(lua_string!(r#"print("Hello, world!")"#), true).unwrap();
		assert_eq!(bytecode, [27, 76, 74, 2, 10, 43, 2, 0, 3, 0, 2, 0, 4, 54, 0, 0, 0, 39, 2, 1, 0, 66, 0, 2, 1, 75, 0, 1, 0, 18, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33, 10, 112, 114, 105, 110, 116]);
	}

	check_stack(compiler);
}

#[test]
fn concurrent_compilations() {
	let compiler = std::sync::Arc::new(crate::compiler().unwrap());

	let mut handles = vec![];
	for _ in 1..10 {
		let compiler = compiler.clone();
		handles.push(std::thread::spawn(move || {
			let bytecode = compiler.compile_string(lua_string!(r#"print("Hello, world!")"#), true).unwrap();
			assert_eq!(bytecode, [27, 76, 74, 2, 10, 43, 2, 0, 3, 0, 2, 0, 4, 54, 0, 0, 0, 39, 2, 1, 0, 66, 0, 2, 1, 75, 0, 1, 0, 18, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33, 10, 112, 114, 105, 110, 116]);
		}));
	}

	assert!(handles.into_iter().try_for_each(|handle| handle.join()).is_ok());

	check_stack(std::sync::Arc::try_unwrap(compiler).unwrap());
}

#[test]
fn multiple_syntax_error_compilations() {
	let compiler = crate::compiler().unwrap();

	for _ in 1..10 {
		let result = compiler.compile_string(lua_string!(r#"Invalid Lua code"#), true);
		assert!(match result {
			Ok(_) => false,
			Err(_error) => matches!(crate::LuaError::SyntaxError(Some(r#"[string "Invalid Lua code"]:1: '=' expected near 'Lua'"#.to_string())), _error),
		});
	}

	check_stack(compiler);
}

#[test]
fn concurrent_syntax_error_compilations() {
	let compiler = std::sync::Arc::new(crate::compiler().unwrap());

	let mut handles = vec![];
	for _ in 1..10 {
		let compiler = compiler.clone();
		handles.push(std::thread::spawn(move || {
			let result = compiler.compile_string(lua_string!(r#"Invalid Lua code"#), true);
			assert!(match result {
				Ok(_) => false,
				Err(_error) => matches!(crate::LuaError::SyntaxError(Some(r#"[string "Invalid Lua code"]:1: '=' expected near 'Lua'"#.to_string())), _error),
			});
		}));
	}

	assert!(handles.into_iter().try_for_each(|handle| handle.join()).is_ok());

	check_stack(std::sync::Arc::try_unwrap(compiler).unwrap());
}
