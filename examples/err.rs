fn foo(i: u8) -> Result<u8, ()> {
	if i < 100 {
		Ok(i)
	} else {
		Err(())
	}
}

fn bar(i: u8) -> Result<u8, ()> {
    let mut v = foo(i)?;
    v += 1;
    Ok(v)
}

fn main() {
    match bar(125) {
        Ok(v) => {
            println!("v -> {}", v);
        }
        Err(err) => {
            println!("{} =>", "错误2");
        }
    }
}
