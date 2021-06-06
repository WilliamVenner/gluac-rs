fn main() {
	use std::io::Write;

	let matches = clap::App::new("gluac")
		.version(env!("CARGO_PKG_VERSION"))
		.about("Compiles Garry's Mod Lua source code to bytecode")
		.author("William Venner <william@venner.io>")
		.arg(
			clap::Arg::with_name("strip")
				.long("strip")
				.short("s")
				.help("Strips debug information from the compiled bytecode")
				.multiple(false)
		)
		.arg(
			clap::Arg::with_name("file")
				.short("f")
				.help("Input file path")
				.conflicts_with("input")
				.required(true)
				.multiple(false)
		)
		.arg(
			clap::Arg::with_name("input")
				.help("The input source code")
				.conflicts_with("file")
				.required(true)
				.raw(true)
		)
		.get_matches();

	let strip_debug = matches.args.get("strip").is_some();

	let compiler = gluac_rs::compiler().expect("Failed to initialize bytecode compiler");
	let bytecode = if let Some(src) = matches.args.get("input") {
		let src = src.vals.iter().map(|os_str| os_str.to_string_lossy().into_owned().into_bytes()).flatten().collect::<Vec<u8>>();
		let src = std::ffi::CString::new(src).expect("Expected input source to not contain any NUL bytes!");
		compiler.compile_string(src.as_ptr(), strip_debug).unwrap()
	} else if let Some(path) = matches.args.get("file") {
		compiler.compile_file(gluac_rs::lua_string!(path.vals[0].to_string_lossy().into_owned()), strip_debug).unwrap()
	} else {
		unreachable!();
	};

	let mut stdout = std::io::stdout();
	stdout.write_all(&bytecode).expect("Failed to write to stdout");
	stdout.flush().expect("Failed to write to stdout");
}
