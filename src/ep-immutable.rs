fn main(){
    let x: i32 = 5;
    let y = 4; // Değişkenin tipini belirtmezsek onu "immutable" olarak kabul eder.
    println!("İlk değer: {}", x);
    println!("y: {}",y);
    /* x = 6; Bu satır derlenmez çünkü
    x immutable bir değişkendir ve
    değeri değiştirilemez.
    Hafızadaki o yer sadece okumak için açık üzerine yazı yazılamıyor
    bu yüzden üzerine yeni değer atanamaz.
    */

}