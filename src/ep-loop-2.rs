fn main() {
    let mut sayac = 0;

    // Loop'tan bir sonuç döndürebiliyoruz.
    let sonuc = loop {
        sayac+=1;

        if sayac == 10 {
            break sayac * 2;
        }
    };

    println!("Döngüden dönen sonuc: {}", sonuc);

}