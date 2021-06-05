#[test]
fn hello_world() {
	let compiler = crate::compiler().unwrap();
	let bytecode = compiler.compile_string(lua_string!(r#"print("Hello, world!")"#), true).unwrap();
	assert_eq!(bytecode, [27, 76, 74, 2, 10, 43, 2, 0, 3, 0, 2, 0, 4, 54, 0, 0, 0, 39, 2, 1, 0, 66, 0, 2, 1, 75, 0, 1, 0, 18, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33, 10, 112, 114, 105, 110, 116]);
}

#[test]
fn hello_world_file() {
	let compiler = crate::compiler().unwrap();
	let bytecode = compiler.compile_file(lua_string!(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/tests/hello_world.lua").to_string_lossy().to_string()), true).unwrap();
	assert_eq!(bytecode, [27, 76, 74, 2, 10, 43, 2, 0, 3, 0, 2, 0, 4, 54, 0, 0, 0, 39, 2, 1, 0, 66, 0, 2, 1, 75, 0, 1, 0, 18, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33, 10, 112, 114, 105, 110, 116]);
}

#[test]
fn syntax_error() {
	let compiler = crate::compiler().unwrap();
	let result = compiler.compile_string(lua_string!(r#"Invalid Lua code"#), true);
	assert!(match result {
		Ok(_) => false,
		Err(_error) => matches!(crate::LuaError::SyntaxError(Some(r#"[string "Invalid Lua code"]:1: '=' expected near 'Lua'"#.to_string())), _error),
	});
}
