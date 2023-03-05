/// A single term (ie. x₁, ¬x₁, etc)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Term(u64);

/// A term identifier: TermID(x₁) == TermID(¬x₁)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TermID(u64);

impl TermID {
    /// Turn back into a term
    /// x₁.into::<TermID>().literal() == x₁
    pub fn literal(self) -> Term {
        Term(self.0 << 1)
    }

    /// Turn back into a negated term
    /// x₁.into::<TermID>().literal() == ¬x₁
    pub fn negate(self) -> Term {
        Term((self.0 << 1) | 1)
    }
}

impl From<Term> for TermID {
    fn from(value: Term) -> Self {
        TermID(value.0 >> 1)
    }
}
