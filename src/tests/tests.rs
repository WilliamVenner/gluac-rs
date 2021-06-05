fn verify_hello_world_bytecode(bytecode: &[u8]) {
	#[cfg(target_pointer_width = "64")]
	assert_eq!(bytecode, [27, 76, 74, 2, 10, 43, 2, 0, 3, 0, 2, 0, 4, 54, 0, 0, 0, 39, 2, 1, 0, 66, 0, 2, 1, 75, 0, 1, 0, 18, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33, 10, 112, 114, 105, 110, 116, 0]);

	#[cfg(target_pointer_width = "32")]
	assert_eq!(bytecode, [27, 76, 74, 1, 2, 43, 2, 0, 2, 0, 2, 0, 4, 52, 0, 0, 0, 37, 1, 1, 0, 62, 0, 2, 1, 71, 0, 1, 0, 18, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33, 10, 112, 114, 105, 110, 116, 0]);
}

fn check_stack(compiler: crate::compiler::BytecodeCompiler) {
	assert_eq!(compiler.stack_size(), 3);
	assert_eq!(compiler.get_type(compiler.stack_size()), "function");
}

fn compile_hello_world_string(compiler: &crate::compiler::BytecodeCompiler) {
	let bytecode = compiler.compile_string(lua_string!(r#"print("Hello, world!")"#), true).unwrap();
	verify_hello_world_bytecode(&bytecode);
}

fn compile_syntax_error(compiler: &crate::compiler::BytecodeCompiler) {
	let result = compiler.compile_string(lua_string!(r#"Invalid Lua code"#), true);
	assert!(match result {
		Ok(_) => false,
		Err(_error) => matches!(crate::LuaError::SyntaxError(Some(r#"[string "Invalid Lua code"]:1: '=' expected near 'Lua'"#.to_string())), _error),
	});
}

fn compile_invalid_file(compiler: &crate::compiler::BytecodeCompiler) {
	let result = compiler.compile_file(lua_string!("this file does not exist"), true);
	assert!(match result {
		Ok(_) => false,
		Err(_error) => matches!(crate::LuaError::SyntaxError(Some(r#"cannot open this file does not exist: No such file or directory"#.to_string())), _error),
	});
}

fn compile_hello_world_file(compiler: &crate::compiler::BytecodeCompiler) {
	let bytecode = compiler.compile_file(lua_string!(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/tests/hello_world.lua").to_string_lossy().to_string()), true).unwrap();
	verify_hello_world_bytecode(&bytecode);
}

#[test]
fn hello_world_file() {
	let compiler = crate::compiler().unwrap();

	compile_hello_world_file(&compiler);

	check_stack(compiler);
}

#[test]
fn multiple_hello_world_file() {
	let compiler = crate::compiler().unwrap();

	for _ in 1..10 {
		compile_hello_world_file(&compiler);
	}

	check_stack(compiler);
}

#[test]
fn concurrent_hello_world_file() {
	let compiler = std::sync::Arc::new(crate::compiler().unwrap());

	let mut handles = vec![];
	for _ in 1..10 {
		let compiler = compiler.clone();
		handles.push(std::thread::spawn(move || {
			compile_hello_world_file(&*compiler);
		}));
	}

	assert!(handles.into_iter().try_for_each(|handle| handle.join()).is_ok());

	check_stack(std::sync::Arc::try_unwrap(compiler).unwrap());
}

#[test]
fn invalid_file() {
	let compiler = crate::compiler().unwrap();

	compile_invalid_file(&compiler);

	check_stack(compiler);
}

#[test]
fn multiple_invalid_file() {
	let compiler = crate::compiler().unwrap();

	for _ in 1..10 {
		compile_invalid_file(&compiler);
	}

	check_stack(compiler);
}

#[test]
fn concurrent_invalid_file() {
	let compiler = std::sync::Arc::new(crate::compiler().unwrap());

	let mut handles = vec![];
	for _ in 1..10 {
		let compiler = compiler.clone();
		handles.push(std::thread::spawn(move || {
			compile_invalid_file(&*compiler);
		}));
	}

	assert!(handles.into_iter().try_for_each(|handle| handle.join()).is_ok());

	check_stack(std::sync::Arc::try_unwrap(compiler).unwrap());
}

#[test]
fn hello_world() {
	let compiler = crate::compiler().unwrap();

	compile_hello_world_string(&compiler);

	check_stack(compiler);
}

#[test]
fn syntax_error() {
	let compiler = crate::compiler().unwrap();

	compile_syntax_error(&compiler);

	check_stack(compiler);
}

#[test]
fn multiple_compilations() {
	let compiler = crate::compiler().unwrap();

	for _ in 1..10 {
		compile_hello_world_string(&compiler);
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
			compile_hello_world_string(&*compiler);
		}));
	}

	assert!(handles.into_iter().try_for_each(|handle| handle.join()).is_ok());

	check_stack(std::sync::Arc::try_unwrap(compiler).unwrap());
}

#[test]
fn multiple_syntax_error_compilations() {
	let compiler = crate::compiler().unwrap();

	for _ in 1..10 {
		compile_syntax_error(&compiler);
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
			compile_syntax_error(&*compiler);
		}));
	}

	assert!(handles.into_iter().try_for_each(|handle| handle.join()).is_ok());

	check_stack(std::sync::Arc::try_unwrap(compiler).unwrap());
}

#[test]
fn multiple_mixed_compilations() {
	let compiler = crate::compiler().unwrap();

	for i in 1..10 {
		if i % 2 == 0 {
			compile_hello_world_string(&compiler);
		} else {
			compile_syntax_error(&compiler);
		}
	}

	check_stack(compiler);
}

#[test]
fn concurrent_mixed_compilations() {
	let compiler = std::sync::Arc::new(crate::compiler().unwrap());

	let mut handles = vec![];
	for i in 1..10 {
		let compiler = compiler.clone();
		if i % 2 == 0 {
			handles.push(std::thread::spawn(move || {
				compile_hello_world_string(&*compiler);
			}));
		} else {
			handles.push(std::thread::spawn(move || {
				compile_syntax_error(&*compiler);
			}));
		}
	}

	assert!(handles.into_iter().try_for_each(|handle| handle.join()).is_ok());

	check_stack(std::sync::Arc::try_unwrap(compiler).unwrap());
}
