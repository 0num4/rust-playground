pub fn gacha() {
    println!("ガチャガチャ");
    kakuritu(20.0)
}

pub fn kakuritu(count: f64) {
    // let atari_kakuritu = 0.05; // 5%
    let hazure: f64 = 0.95; // 95%
    let kakuritu = hazure.powf(count);
    print!("10連引いて外れる確率は{:.1}%です。", (kakuritu * 100.0));
    println!("確率");
}
