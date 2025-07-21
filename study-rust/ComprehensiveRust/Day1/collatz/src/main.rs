fn collatz_length(mut n: i32) -> u32 {
	let mut len = 1;
	while n > 1{
        	n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        	len += 1;
    	}
    	//len 도 가능!
	return len;
}

fn main() {
    	println!("Length: {}", collatz_length(33));
}
