#![allow(dead_code)]

use std::marker::PhantomData;
use crate::logic::{True, False, Eq, TypeFn};

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
// Sealed trait + BinNat: 閉じた自然数型（閉鎖性）
//
// sealed trait パターンにより、外部からの BinNat 実装を禁止。
// BinNat を実装できるのは One, Dbl<N>, DblP1<N> のみ。
// ============================================================

mod sealed {
    pub trait Sealed {}
}

pub trait BinNat: sealed::Sealed {}

impl sealed::Sealed for One {}
impl<N: BinNat> sealed::Sealed for Dbl<N> {}
impl<N: BinNat> sealed::Sealed for DblP1<N> {}

impl BinNat for One {}
impl<N: BinNat> BinNat for Dbl<N> {}
impl<N: BinNat> BinNat for DblP1<N> {}

// ============================================================
// Accessible: 整礎性（well-foundedness）
//
// BinNat の全ての値が有限の深さを持つことを型構造で表現する。
// sealed trait により Accessible の実装も One/Dbl/DblP1 に限定され、
// 各 impl は「N: Accessible ならば Dbl<N>/DblP1<N>: Accessible」
// という構造的に増加する形なので、停止性が保証される。
//
// ┌─────────────────────────────────────────────────┐
// │  停止性を人間が検証すべき箇所はこの3つの impl のみ  │
// └─────────────────────────────────────────────────┘
//
// 帰納法の原理は閉鎖性（sealed）+ 整礎性（Accessible）から従う。
// ============================================================

pub trait Accessible: BinNat + Sized {
    /// 帰納法の適用: Accessible な値に対して InductionScheme を適用
    /// 再帰はこのメソッドのみで発生する
    fn induct<F: InductionScheme>() -> F::Result<Self>;
}

impl Accessible for One {
    fn induct<F: InductionScheme>() -> F::Result<One> {
        F::case_one()
    }
}

impl<N: Accessible> Accessible for Dbl<N> {
    fn induct<F: InductionScheme>() -> F::Result<Dbl<N>> {
        let ih = N::induct::<F>();
        F::case_dbl::<N>(ih)
    }
}

impl<N: Accessible> Accessible for DblP1<N> {
    fn induct<F: InductionScheme>() -> F::Result<DblP1<N>> {
        let ih = N::induct::<F>();
        F::case_dblp1::<N>(ih)
    }
}

/// 構造的帰納法のスキーム
///
/// 3つのケース（基底: One, 帰納: Dbl, DblP1）を提供する。
/// 再帰は Accessible::induct が担当するため、
/// InductionScheme の実装では再帰しない。
pub trait InductionScheme {
    type Result<N: Accessible>;
    fn case_one() -> Self::Result<One>;
    fn case_dbl<N: Accessible>(ih: Self::Result<N>) -> Self::Result<Dbl<N>>;
    fn case_dblp1<N: Accessible>(ih: Self::Result<N>) -> Self::Result<DblP1<N>>;
}

// ============================================================
// 型レベル算術: 後者関数 (increment)
// ============================================================

pub trait BinSucc: BinNat { type Result: BinNat; }

impl BinSucc for One {
    type Result = Dbl<One>; // 1+1 = 2
}
impl<N: BinNat> BinSucc for Dbl<N> {
    type Result = DblP1<N>; // 2n+1
}
impl<N: BinNat + BinSucc> BinSucc for DblP1<N> {
    type Result = Dbl<N::Result>; // 2(n+1)
}

// ============================================================
// 型レベル算術: 加算
// ============================================================

pub trait BinAdd<B: BinNat>: BinNat { type Result: BinNat; }

// One + x
impl BinAdd<One> for One {
    type Result = Dbl<One>; // 1+1=2
}
impl<N: BinNat> BinAdd<Dbl<N>> for One {
    type Result = DblP1<N>; // 1+2n = 2n+1
}
impl<N: BinNat + BinSucc> BinAdd<DblP1<N>> for One {
    type Result = Dbl<N::Result>; // 1+(2n+1) = 2(n+1)
}

// Dbl<A> + x
impl<A: BinNat> BinAdd<One> for Dbl<A> {
    type Result = DblP1<A>; // 2a+1
}
impl<A: BinNat + BinAdd<B>, B: BinNat> BinAdd<Dbl<B>> for Dbl<A> {
    type Result = Dbl<<A as BinAdd<B>>::Result>; // 2a+2b = 2(a+b)
}
impl<A: BinNat + BinAdd<B>, B: BinNat> BinAdd<DblP1<B>> for Dbl<A> {
    type Result = DblP1<<A as BinAdd<B>>::Result>; // 2a+(2b+1) = 2(a+b)+1
}

// DblP1<A> + x
impl<A: BinNat + BinSucc> BinAdd<One> for DblP1<A> {
    type Result = Dbl<A::Result>; // (2a+1)+1 = 2(a+1)
}
impl<A: BinNat + BinAdd<B>, B: BinNat> BinAdd<Dbl<B>> for DblP1<A> {
    type Result = DblP1<<A as BinAdd<B>>::Result>; // (2a+1)+2b = 2(a+b)+1
}
impl<A: BinNat + BinAdd<B>, B: BinNat> BinAdd<DblP1<B>> for DblP1<A>
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

pub trait BinSquare: BinNat { type Result: BinNat; }

impl BinSquare for One {
    type Result = One; // 1² = 1 (奇数!)
}
impl<N: BinNat + BinSquare> BinSquare for Dbl<N> {
    type Result = Dbl<Dbl<N::Result>>; // (2n)² = 4n² (偶数!)
}
impl<N: BinNat + BinSquare> BinSquare for DblP1<N>
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
