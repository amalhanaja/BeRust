use std::io;

use rand::Rng;

fn main() {
    println!("Selamat datang di game tebak angka!");
    let computer = rand::thread_rng().gen_range(1..=100);
    println!("Angka Computer: {}", computer);

    let mut score = 0;
    loop {
        println!("Masukkan tebakkan anda:");
        let mut tebakkan = String::new();
        io::stdin()
            .read_line(&mut tebakkan)
            .expect("Masukkan tebakkan dengan benar");

        let tebakkan = match tebakkan.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => {
                println!("Masukkan tebakkan dengan benar");
                continue;
            }
        };

        score += 1;

        match tebakkan.cmp(&computer) {
            std::cmp::Ordering::Less => println!("Terlalu kecil"),
            std::cmp::Ordering::Equal => break,
            std::cmp::Ordering::Greater => println!("Terlalu besar"),
        }
    }

    println!("Selamat, Tebakkan anda benar");
    println!("Anda telah menebak sebanyak: {score}");
}
