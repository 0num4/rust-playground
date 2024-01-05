pub fn gacha() {
    println!("ガチャガチャ");
    kakuritu()
}

pub fn kakuritu() {
    // let atari_kakuritu = 0.05; // 5%
    let hazure: f64 = 0.95; // 95%
    let kakuritu = hazure.powf(10.0);
    print!("10連引いて外れる確率は{:.1}%です。", (kakuritu * 100.0));
    println!("確率");
}
