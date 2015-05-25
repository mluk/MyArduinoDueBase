#[link(name = "huhu", kind="static")]
extern {
	fn hello()->();
}


fn main(){
	unsafe {
		hello();
	}

}
