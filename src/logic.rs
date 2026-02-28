use std::marker::PhantomData;

pub enum False {}
impl False {
    pub fn elim<A>(self) -> A {
        match self {}
    }
}

pub struct True;

pub struct And<A, B>(A, B);
impl<A, B> And<A, B> {
    pub fn intro(a: A, b: B) -> Self { And(a, b) }
    pub fn left(self) -> A { self.0 }
    pub fn right(self) -> B { self.1 }
}

pub enum Or<A, B> { Left(A), Right(B) }
impl<A, B> Or<A, B> {
    pub fn elim<C>(self, f: impl FnOnce(A) -> C, g: impl FnOnce(B) -> C) -> C {
        match self { Or::Left(a) => f(a), Or::Right(b) => g(b) }
    }
}

/// 型レベル関数: F を型 T に適用して型を返す
pub trait TypeFn<T> {
    type Result;
}

/// 等号 (A = B): refl() でのみ構成可能（A = B のとき）
pub struct Eq<A, B>(PhantomData<(A, B)>);

impl<A> Eq<A, A> {
    pub fn refl() -> Self { Eq(PhantomData) }
}

impl<A, B> Eq<A, B> {
    /// Transport (Leibniz代入): A = B ならば F(A) → F(B)
    ///
    /// SAFETY: Eq<A,B> は refl() でのみ構成可能で、A = B が保証される。
    /// したがって F::Result<A> と F::Result<B> は同一の型。
    pub fn transport<F: TypeFn<A> + TypeFn<B>>(
        self,
        fa: <F as TypeFn<A>>::Result,
    ) -> <F as TypeFn<B>>::Result {
        unsafe {
            let result = std::mem::transmute_copy(&fa);
            std::mem::forget(fa);
            result
        }
    }

    /// 対称律: A = B ならば B = A
    pub fn symm(self) -> Eq<B, A> {
        // SAFETY: Eq<A,B> は A = B を保証するので Eq<B,A> も有効
        Eq(PhantomData)
    }
}

// --- 全称量化 ---

/// ∀N. Prop(N) — 全ての正の2進自然数 N に対して Prop(N) が成り立つ
pub struct ForAll<Prop>(PhantomData<Prop>);

impl<Prop> ForAll<Prop> {
    /// 公理: 構造的帰納法の原理
    ///
    /// SAFETY: 呼び出し前に以下を検証すること:
    ///   1. Prop(One) が成り立つ       （基底: 奇数）
    ///   2. ∀N. Prop(N) → Prop(DblP1<N>)（帰納: 奇数）
    ///   3. ∀K. Prop(K) → Prop(Dbl<K>)  （帰納: 偶数）
    ///
    /// Rustの型チェッカーは帰納法の停止性を検証しないため、
    /// この原理は unsafe な公理として導入する。
    pub unsafe fn by_induction() -> Self {
        ForAll(PhantomData)
    }
}
