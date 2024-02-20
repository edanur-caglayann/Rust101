fn main(){
    let sayi: i32 = 5;

    if sayi > 0 {
        println!("{} pozitif bir sayıdır.",sayi);
    }
    else if sayi < 0 {
        println!("{} negatif bir sayıdır.",sayi);
    }
    else {
        println!("Sayı sıfırdır.");
    }

    if sayi % 2 == 0 {
        println!("{} çift bir sayıdır.",sayi);
    }
        else {
            println!("{} tek bir sayıdır.",sayi);
        }
}