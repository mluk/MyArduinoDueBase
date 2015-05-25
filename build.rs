use std::process::Command;
use std::env;
use std::path::Path;


// Arduino ARM Compiler: /Users/mluk/arm-cs-tools/arm-none-eabi/bin
const PATH_ARM_EABI : &'static str = "/Users/mluk/arm-cs-tools/arm-none-eabi/bin";

enum CompileCommand {
AR, CXX, GCC, OBJCOPY, RUSTC
}

impl CompileCommand{
fn name(&self) -> String {
	match self {
			&CompileCommand::AR => format!("{}/ar", PATH_ARM_EABI),
			&CompileCommand::CXX => format!("{}/gcc", PATH_ARM_EABI),
			&CompileCommand::GCC => format!("{}/g++", PATH_ARM_EABI),
			&CompileCommand::OBJCOPY => format!("{}/objcopy", PATH_ARM_EABI),
			&CompileCommand::RUSTC => "rustc".to_string()
		}
}

fn execute(&self,args : &[&str]){
    Command::new(self.name()).args(args).status().unwrap();
}
}



use CompileCommand::*;

fn main(){
   println!("{}", CompileCommand::AR.name());
   RUSTC.execute(&["/Users/mluk/rustwithc/build2.rs"]);




	//	let out_dir = env::var("OUT_DIR").unwrap();

	// note that there are a number of downsides to this approach, the comments
	// below detail how to improve the portability of these commands.

	//rustc -C opt-level=2 -Z no-landing-pads --target thumbv7m-none-eabi -g --emit obj -L libcore-thumbv7m -o my_rust_file.o my_rust_file.rs

	//					Command::new("rustc").args(&["-C", "opt-level=2", "-Z", "no-landing-pads", "--target", "thumbv7m-none-eabi", "-g", "--emit", "obj", "-L", "/Users/mluk/rust/libcore-thumbv7m", "-o", "huhu.o", "src/main.rs"])
	//			.status().unwrap();
	// rustc -C opt-level=2 -Z no-landing-pads --target thumbv7m-none-eabi -g src/libcore/lib.rs --out-dir libcore-thumbv7m


}