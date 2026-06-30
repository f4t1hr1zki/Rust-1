//() berarti start/ memulai secara otomatis dari sebuah fungsi
//{} tempat untuk menaruh seluruh fungsi yang akan di jalankan

use std::cmp::Ordering;
use std::io; // gunakan use sebagai input fungsi eksternal
use rand::Rng; //untuk fungsi random gen number

fn main() { //fungsi yang mengawali sebuah barisan kode
    println!("Guess the number!");

//NumberGen
let secret_number = 
 	rand::thread_rng() 
 		.gen_range(1..=20);
 		
//input guess    
loop {
    println!("Please Input the Number!");

    let mut guess = String::new(); // buat mutable(tidak terkunci/bisa di ubah) 
    							   //var(guess) menjadi/= String::new
    io::stdin() //Sebuah fungsi dari std::io yang baru di import
    .read_line(&mut guess) //.read_line adalah fungsi yang berasal dari stdin. 
    						//& adalah sebuah tag memori dan bisa juga tag variable dan bersifat ro
    .expect("Failed to read line"); //sebuah fungsi universal yang membuat kerja dari kode
    								//dihentikan secara paksa. ini juga tempat result di hasilkan

//Parse(Konversi Data) dan trim(penghilangan enter dan spasi)
	    let guess: u32 = match guess.trim() .parse() {
	    	Ok(num) => num, // kalau ok akan mengeluarkan angka
	    	Err(_) => continue, // Mirip if err != nil { continue }
	    };

   println!("You Guessed: {guess}"); // hasil dari sebuah var guess yang di tag di dalamnya

//Fungsi Yang membandingkan besar kecil angka random dengan input user
	match guess.cmp(&secret_number) {
			Ordering::Less => println!("To Small"), //angka input kalau kecil dari angka perbandingan
			Ordering::Greater => println!("To Big"), //cuma bedanya kalau ini terlalu besar
			Ordering::Equal => { 
				println!("You Win"); //kalau pas akan break //equal yaitu pas
				break;
			}
		}
	}
}
