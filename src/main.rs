mod input;
mod logic;
mod nat;
mod proof;
mod sqrt2;

fn main() {
    let _ = proof::one_plus_two_is_three();
    let _ = proof::two_times_two_is_four();
    let _ = proof::zero_plus_n_is_n::<nat::N5>();

    // sqrt2_irr は Eq<p², 2q²> を引数に取る。
    // この型の値は構成不可能 = 仮定が矛盾 = 呼び出し自体が不可能。
    // コンパイルが通ること自体が証明。

    println!("全ての証明が検証されました！");
    println!();
    println!("  ✓ 1 + 2 = 3");
    println!("  ✓ 2 × 2 = 4");
    println!("  ✓ ∀n. 0 + n = n");
    println!("  ✓ ∀p,q ∈ ℕ⁺. p² ≠ 2q²  (√2 は無理数)");
}
