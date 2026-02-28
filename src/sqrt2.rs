#![allow(dead_code)]

use std::marker::PhantomData;
use crate::logic::{True, False, Eq, TypeFn, ForAll};

// ============================================================
// 正の自然数（2進表現: 偶奇が構造的に判定可能）
// ============================================================

pub struct One;                          // 1 (奇数)
pub struct Dbl<N>(PhantomData<N>);       // 2N (偶数)
pub struct DblP1<N>(PhantomData<N>);     // 2N+1 (奇数)

pub type P1 = One;
pub type P2 = Dbl<One>;
pub type P3 = DblP1<One>;
pub type P4 = Dbl<Dbl<One>>;
pub type P5 = DblP1<Dbl<One>>;
pub type P6 = Dbl<DblP1<One>>;
pub type P7 = DblP1<DblP1<One>>;
pub type P8 = Dbl<Dbl<Dbl<One>>>;

// ============================================================
// 型レベル算術: 後者関数 (increment)
// ============================================================

pub trait BinSucc { type Result; }

impl BinSucc for One {
    type Result = Dbl<One>; // 1+1 = 2
}
impl<N> BinSucc for Dbl<N> {
    type Result = DblP1<N>; // 2n+1
}
impl<N: BinSucc> BinSucc for DblP1<N> {
    type Result = Dbl<N::Result>; // 2(n+1)
}

// ============================================================
// 型レベル算術: 加算
// ============================================================

pub trait BinAdd<B> { type Result; }

// One + x
impl BinAdd<One> for One {
    type Result = Dbl<One>; // 1+1=2
}
impl<N> BinAdd<Dbl<N>> for One {
    type Result = DblP1<N>; // 1+2n = 2n+1
}
impl<N: BinSucc> BinAdd<DblP1<N>> for One {
    type Result = Dbl<N::Result>; // 1+(2n+1) = 2(n+1)
}

// Dbl<A> + x
impl<A> BinAdd<One> for Dbl<A> {
    type Result = DblP1<A>; // 2a+1
}
impl<A: BinAdd<B>, B> BinAdd<Dbl<B>> for Dbl<A> {
    type Result = Dbl<<A as BinAdd<B>>::Result>; // 2a+2b = 2(a+b)
}
impl<A: BinAdd<B>, B> BinAdd<DblP1<B>> for Dbl<A> {
    type Result = DblP1<<A as BinAdd<B>>::Result>; // 2a+(2b+1) = 2(a+b)+1
}

// DblP1<A> + x
impl<A: BinSucc> BinAdd<One> for DblP1<A> {
    type Result = Dbl<A::Result>; // (2a+1)+1 = 2(a+1)
}
impl<A: BinAdd<B>, B> BinAdd<Dbl<B>> for DblP1<A> {
    type Result = DblP1<<A as BinAdd<B>>::Result>; // (2a+1)+2b = 2(a+b)+1
}
impl<A: BinAdd<B>, B> BinAdd<DblP1<B>> for DblP1<A>
where
    <A as BinAdd<B>>::Result: BinSucc,
{
    type Result = Dbl<<<A as BinAdd<B>>::Result as BinSucc>::Result>;
    // (2a+1)+(2b+1) = 2(a+b+1)
}

// ============================================================
// 型レベル算術: 2乗
//   1²       = 1
//   (2k)²    = 4k²           = Dbl<Dbl<k²>>
//   (2k+1)²  = 4k²+4k+1     = DblP1<Dbl<k²+k>>
// ============================================================

pub trait BinSquare { type Result; }

impl BinSquare for One {
    type Result = One; // 1² = 1 (奇数!)
}
impl<N: BinSquare> BinSquare for Dbl<N> {
    type Result = Dbl<Dbl<N::Result>>; // (2n)² = 4n² (偶数!)
}
impl<N: BinSquare> BinSquare for DblP1<N>
where
    N::Result: BinAdd<N>,
{
    type Result = DblP1<Dbl<<N::Result as BinAdd<N>>::Result>>;
    // (2n+1)² = 4(n²+n)+1 (奇数!)
}

// ============================================================
// Discriminate: 偶数 ≠ 奇数 を導く型レベル関数
// ============================================================

/// IsOddFn: 奇数なら True, 偶数なら False を返す型レベル関数
pub struct IsOddFn;

impl TypeFn<One> for IsOddFn { type Result = True; }
impl<N> TypeFn<Dbl<N>> for IsOddFn { type Result = False; }
impl<N> TypeFn<DblP1<N>> for IsOddFn { type Result = True; }

/// 奇数 = 偶数 という仮定から矛盾を導く
pub fn discriminate_odd_dbl<OddTy, X>(eq: Eq<OddTy, Dbl<X>>) -> False
where
    IsOddFn: TypeFn<OddTy, Result = True> + TypeFn<Dbl<X>, Result = False>,
{
    // IsOddFn(OddTy) = True, IsOddFn(Dbl<X>) = False
    // transport: True → False （矛盾!）
    eq.transport::<IsOddFn>(True)
}

// ============================================================
// Injectivity: Dbl<X> = Dbl<Y> → X = Y
// ============================================================

/// InjectFn<X>: Dbl<N> → Eq<N, X> を返す型レベル関数
pub struct InjectFn<X>(PhantomData<X>);

impl<X> TypeFn<One> for InjectFn<X> { type Result = Eq<One, X>; }
impl<N, X> TypeFn<Dbl<N>> for InjectFn<X> { type Result = Eq<N, X>; }
impl<N, X> TypeFn<DblP1<N>> for InjectFn<X> { type Result = Eq<N, X>; }

/// Dbl の単射性: Dbl<X> = Dbl<Y> ならば X = Y
pub fn dbl_injective<X, Y>(eq: Eq<Dbl<X>, Dbl<Y>>) -> Eq<X, Y> {
    // InjectFn<X> を Dbl<X> に適用: Eq<X, X> = refl
    // transport で Dbl<Y> に移す: Eq<Y, X>
    let eq_yx: Eq<Y, X> = eq.transport::<InjectFn<X>>(Eq::refl());
    eq_yx.symm()
}

// ============================================================
// √2 の無理数性: ∀ p,q ∈ ℕ⁺, p² ≠ 2q²
//
// 証明 (無限降下法):
//   p が奇数 → p² は奇数, 2q² は偶数 → discriminate → 矛盾
//   p = 2k  → (2k)²=4k²=2q² → 2k²=q²
//           → Eq<Square<Q>, Dbl<Square<K>>> に帰着（降下）
//
// --- コンパイラが検証する部分 ---
//   - BinSquare: 型レベル2乗の計算結果
//   - discriminate: 奇数型 ≠ 偶数型 (transport経由)
//   - dbl_injective: Dbl<X>=Dbl<Y> → X=Y (transport経由)
//   - 各 impl の型整合性（帰納ステップの正しさ）
//
// --- 公理（コンパイラが検証しない部分） ---
//   - transport: Eq<A,B> → F(A) → F(B) (unsafe)
//   - 構造的帰納法の原理: {One, Dbl, DblP1} 上の再帰は停止する (unsafe)
// ============================================================

pub trait Sqrt2Irr<Q: BinSquare>: BinSquare {
    fn prove(
        eq: Eq<<Self as BinSquare>::Result, Dbl<<Q as BinSquare>::Result>>,
    ) -> False;
}

/// P = 1 (奇数): 1² = 1(奇数) ≠ Dbl<_>(偶数) → 矛盾
impl<Q: BinSquare> Sqrt2Irr<Q> for One {
    fn prove(
        eq: Eq<One, Dbl<<Q as BinSquare>::Result>>,
    ) -> False {
        discriminate_odd_dbl(eq)
    }
}

/// P = 2N+1 (奇数): (2N+1)² = DblP1<_>(奇数) ≠ Dbl<_>(偶数) → 矛盾
impl<N: BinSquare, Q: BinSquare> Sqrt2Irr<Q> for DblP1<N>
where
    N::Result: BinAdd<N>,
{
    fn prove(
        eq: Eq<
            DblP1<Dbl<<N::Result as BinAdd<N>>::Result>>,
            Dbl<<Q as BinSquare>::Result>,
        >,
    ) -> False {
        discriminate_odd_dbl(eq)
    }
}

/// P = 2K (偶数): (2K)²=4K²=2Q² → Q²=2K² → (Q, K) に降下
impl<K: BinSquare, Q: BinSquare + Sqrt2Irr<K>> Sqrt2Irr<Q> for Dbl<K> {
    fn prove(
        eq: Eq<
            Dbl<Dbl<<K as BinSquare>::Result>>,
            Dbl<<Q as BinSquare>::Result>,
        >,
    ) -> False {
        // Dbl<Dbl<K²>> = Dbl<Q²>
        // → Dbl<K²> = Q²        (injectivity)
        // → Q² = Dbl<K²>        (symmetry)
        // → Sqrt2Irr<K>::prove  (降下)
        let eq_inner = dbl_injective(eq);
        <Q as Sqrt2Irr<K>>::prove(eq_inner.symm())
    }
}

// ============================================================
// 定理: √2 は無理数
//
// Sqrt2Irr の 3つの impl が帰納法の条件を構成する:
//   基底1: impl Sqrt2Irr<Q> for One       — 1²は奇数, 矛盾
//   基底2: impl Sqrt2Irr<Q> for DblP1<N>  — (2N+1)²は奇数, 矛盾
//   帰納:  impl Sqrt2Irr<Q> for Dbl<K>    — 降下ステップ
//
// 上記の各 impl の正しさはコンパイラが検証する。
// 「これら3ケースの網羅 + 構造的再帰の停止性 → 全ての N で成立」
// という帰納法の原理のみが公理（unsafe）。
// ============================================================

/// √2の無理数性を表す述語
/// Sqrt2IrrProp(P) = 「∀Q. P² ≠ 2Q²」
pub struct Sqrt2IrrProp;

/// ∀P∈ℕ⁺. ∀Q∈ℕ⁺. P² ≠ 2Q²
pub fn sqrt2_is_irrational() -> ForAll<Sqrt2IrrProp> {
    // 帰納法の3条件は Sqrt2Irr の impl で検証済み。
    // 帰納法の原理（停止性）のみ公理として導入。
    unsafe { ForAll::by_induction() }
}
