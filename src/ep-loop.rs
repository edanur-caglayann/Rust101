fn main(){
    // loop, sonsuz döngüdür
    let mut sayac = 0;

    loop {
        sayac+=1;
        print!("sayac = {}",sayac);

        if sayac == 5 {
            print!("Döngüden çıkılıyor..");
            break;
        }
    }
}