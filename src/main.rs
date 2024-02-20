mod r#ep{

}

fn main() {
    let x: i32 = -5;
    let y: u32 = 10;

    let float: f64 = 3.14;

    let c: char = 'z';

    let s: &str = "Merhaba Rust!";
    let string: String = String::from("Merhaba Rust!");

    let is_active: bool = true;

    let tuple: (i32, f64, char) = (42, 6.12, 'a');

    let array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Tamsayı: {} ve {}", x, y);
    println!("Kayan Nokta Sayısı: {} ", float);
    println!("Karakter: {} ", c);
    println!("Statik String: {} ", s);
    println!("String: {} ", string);
    println!("Boolean: {}", is_active);
    println!("Tuple:({},{},{})", tuple.0, tuple.1, tuple.2);
    println!("Array'in ilk elemanı: {} ", array[1]);

    println!("Array içeriği:");
    for n in array.iter() {
        println!("{}", n);
    }
}
