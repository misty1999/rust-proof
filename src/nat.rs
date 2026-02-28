// ========================================
// ペアノ自然数 (型レベル)
// 自然数を型として表現し、算術をコンパイル時に計算する
// ========================================

use std::marker::PhantomData;

/// 0
pub struct Zero;

/// 後者関数: Succ<N> = N + 1
pub struct Succ<N>(PhantomData<N>);

// 便利な型エイリアス
pub type N0 = Zero;
pub type N1 = Succ<N0>;
pub type N2 = Succ<N1>;
pub type N3 = Succ<N2>;
pub type N4 = Succ<N3>;
pub type N5 = Succ<N4>;

// --- 型レベル加算 ---

/// Add<B>::Result = Self + B
pub trait Add<B> {
    type Result;
}

/// 0 + B = B
impl<B> Add<B> for Zero {
    type Result = B;
}

/// Succ<A> + B = Succ<A + B>
impl<A: Add<B>, B> Add<B> for Succ<A> {
    type Result = Succ<A::Result>;
}

// --- 型レベル乗算 ---

/// Mul<B>::Result = Self * B
pub trait Mul<B> {
    type Result;
}

/// 0 * B = 0
impl<B> Mul<B> for Zero {
    type Result = Zero;
}

/// Succ<A> * B = B + (A * B)
impl<A, B> Mul<B> for Succ<A>
where
    A: Mul<B>,
    B: Add<A::Result>,
{
    type Result = B::Result;
}
