extern crate gcc;



fn main(){
	gcc::Config::new().file("huhu.c").compile("libhuhu.a");


}