fn main() {
let mut counter = 0;

	loop {
		counter += 1;
		if counter == 2 {
		continue;
		}
				println!("Counter {}", counter);
	if counter == 5 {
		break;
	}
	}
}
