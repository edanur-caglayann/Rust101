fn main() {
    let renkler: [&str;3] = ["Kırmızı", "Yeşil", "Mavi"];
    for renk in renkler.iter(){
        println!("Renk: {}", renk);
    }

    for sayi in 1..4 { // = koyarsak 4 de dahil olmuş olur.
        println!("Sayi: {}",sayi);
    }

    let say = 7;
    for sayi in 1.. say{
        println!("Sayi: {}",sayi)
    }
}