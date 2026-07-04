fn main() {
let nilai = [90, 344, 1, 233, 1000];
let mut max = nilai[0];
	for n in nilai {
		if n > max {
			max = n;
		}
	}
	println!("Nilai tertinggi: {}", max)
}
	//Perhitungan Array
	// let nilai = [40, 03, 222, 121];
	// let mut total = 0;
	// 	for n in nilai {
	// 		total += n;
	// 	}
	// 	println!("Total Semua = {}", total);
	
//rev()
	// for i in (1..=7).rev() {
	// 	println!("{}", i);
	// }
	// println!("Ok");
	
	//For With Array
	// let nilai = [20, 90, 400, 20]; 
	// for n in nilai{
	// 	println!("Nilai {}",n)
	// }

//for dengan Range(1..x)
// for a in 0..=1 {
// 	println!("a = {}", a);
// }

	//Penghitung mundur
	// let mut counter = 10;
	// while counter >= 0 {
	// 	println!("Counter {}", counter);
	// 	counter -= 1;
	// }
	// println!("Selesai")

	
//Penggunaan While
// let mut counter = 0;
// while counter < 5 {
// 	println!("Counter {}", counter);
// 	counter += 1;
// }
// println!("Selesai")

	//loop tapi bisa skip angka yang di inginkan
	// let mut angka = 0;
	// loop {
	// 	angka += 1;
	// 	if angka > 50 {
	// 		break;
	// 	}
	// 	if angka % 5 == 0 {
	// 		continue;
	// 	}
	// 	println!("{}", angka);
	// }

//Loop With Float64
// let mut angka: f64 = 2.5;
// 	loop {
// 		println!("{}", angka);
// 	angka += 2.3;
// 	if angka > 200.5 {
// 		break
// 	}
// 	}

		//Penggunaan Loop bisa mengembalikan Nilai
			// let mut counter = 0;
			// let hasil = loop {
			// 	counter += 1;
			// 
			// 		if counter == 5 {
			// 			break counter * 2;
			// 		}
			// };
			// println!("Hasil: {}", hasil)


//Penggunaan Break
	// let mut counter = 0;

	// 	loop {
	// 		counter += 1;
	// 		if counter == 2 {
	// 		continue;
	// 		}
	// 				println!("Counter {}", counter);
	// 	if counter == 5 {
	// 		break;
	// 	}
	// 	}
