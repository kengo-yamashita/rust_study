fn main() {
    println!("Hello, world!");

    // Hello, Takashi!と表示する機能を追加する

    println!(
        "Hello, {}!",
        "Kenngo",
    );

    // 円の面積を計算して表示する機能を追加する

    println!(
        "半径 {:.1}、円周率 {:.3}、面積 {:.3}",
        3.2,
        std::f64::consts::PI,
        // 面積計算式
        3.2f64.powi(2) * std::f64::consts::PI,
    );
}
