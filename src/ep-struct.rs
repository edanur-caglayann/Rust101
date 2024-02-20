struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person1 = Person {
        name: String:: from("Eda"),
        age: 21,
    };

    println!("1-Name: {}, Age: {}", person1.name, person1.age);
    print_person(&person1);
    println!("2-Name: {}, Age: {}", person1.name, person1.age);

}

fn print_person(person: &Person) {
    println!("{} is {} years old", person.name,person.age);
}
/*
& sembolü kullanmazsak yukarıdaki print_person fonksiyonu çalıştıktan sonra
hem kendisi hem de adresini derleyici silerdi. Ancak & sembolünü kullanarak
biz adresin kopyasını vermiş oluyoruz yani ödünç vermiş oluyoruz.
İşimiz bittiğinde geri getiriyoruz gibi.
 */