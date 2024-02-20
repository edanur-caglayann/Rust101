enum HttpDurum {
    Basarili(u16), // Örneğin 200
    Yonlendirme(u16), // 301
    IstemciHatasi(u16), // 404
    SunucuHatasi(u16), // 500
}

fn durum_mesaji(durum: HttpDurum) {
    match durum {
        HttpDurum::Basarili(kod) => println!("Başarılı: {}",kod),
        HttpDurum::Yonlendirme(kod) => println!("Yönlendirme: {}",kod),
        HttpDurum::IstemciHatasi(kod) => println!("İstemci Hatası: {}",kod),
        HttpDurum::SunucuHatasi(kod) => println!("Sunucu Hatası: {}",kod),
    }
}

fn main() {
    let basarili_durum = HttpDurum::Basarili(200);
    let yonlendirme_durum = HttpDurum::Yonlendirme(301);
    let istemci_hata_durum = HttpDurum::IstemciHatasi(404);
    let sunucu_hata_durum = HttpDurum::SunucuHatasi(500);
}
/*
enumları kontrol ederken match kullanırız.
 */