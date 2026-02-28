#![allow(dead_code)]

use crate::logic::*;
use crate::nat::*;
use crate::sqrt2::{self, Accessible, BinSquare, Sqrt2Irr};

// ============================================================
// 算術の定理（仮定なし）
// ============================================================

/// 1 + 2 = 3
pub fn one_plus_two_is_three() -> Eq<<N1 as Add<N2>>::Result, N3> {
    Eq::refl()
}

/// 2 × 2 = 4
pub fn two_times_two_is_four() -> Eq<<N2 as Mul<N2>>::Result, N4> {
    Eq::refl()
}

/// ∀n. 0 + n = n
pub fn zero_plus_n_is_n<N>() -> Eq<<Zero as Add<N>>::Result, N> {
    Eq::refl()
}

// ============================================================
// √2 の無理数性（仮定なし）
//
// 定理: ∀ p, q ∈ ℕ⁺. p² ≠ 2q²
//
// Eq<p², 2q²> を仮定として受け取り、False を返す。
// つまり「p² = 2q² → ⊥」= 「p² ≠ 2q²」
// ============================================================

/// ∀ P, Q ∈ ℕ⁺: p² ≠ 2q²
pub fn sqrt2_irr<P: Sqrt2Irr<Q>, Q: Accessible + BinSquare>(
    eq: Eq<<P as BinSquare>::Result, sqrt2::Dbl<<Q as BinSquare>::Result>>,
) -> False {
    P::prove(eq)
}
