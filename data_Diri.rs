fn main() {
	let data_diri: (&str, u32, f64, bool) = ("User", 24, 167.7, false);

		println!("Data diri");
		println!("Nama {}", data_diri.0);
		println!("Umur {}", data_diri.1);
		println!("Tinggi {}", data_diri.2);
		println!("Status Nikah {}", data_diri.3);
}
