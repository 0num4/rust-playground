pub fn gacha() {
    println!("ガチャガチャ");
    kakuritu(20.0);
    // pickup_in_atari();
    nya();
}

pub fn kakuritu(count: f64) {
    // let atari_kakuritu = 0.05; // 5%
    let hazure: f64 = 0.95; // 95%
    let kakuritu = hazure.powf(count);
    print!("10連引いて外れる確率は{:.1}%です。", (kakuritu * 100.0));
    println!("確率");
}

// pub fn pickup_in_atari() {
//     // キャラの中から59%の確率でピックアップ当たる。
//     let atari: f64 = 0.05;
//     kakatari*atari*atari*atari*atari*atari*atari*atari*atari*atari;
//     let atari_10ren = atari.powf(10.0);
//     // let pickup_probability = 0.59;
//     // let total_propability = atari_10ren * pickup_probability;
//     print!("ピックアップキャラが当たる確率は{:.1}%です。", (atari_10ren * 100.0));
//     println!("確率");
// }

pub fn nya() {
    // ガチャの確率（5%）
    let gacha_probability: f64 = 0.05;
    // ガチャを何回引くか（10回）
    let num_attempts: i32 = 10;

    // 少なくとも1回当たる確率を計算
    let at_least_one_win_probability = 1.0 - gacha_probability.powi(num_attempts);

    println!(
        "5%のガチャを{}連引いて、少なくとも1回当たる確率は {:.2}% です。",
        num_attempts,
        at_least_one_win_probability * 100.0
    );
}
