#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EmptyContentMode {
    #[default]
    Keep,
    DropEmpty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RowExpansionMode {
    #[default]
    None,
    Multiples,
    PrimeFactors,
    MultiplesAndPrimeFactors,
}

impl RowExpansionMode {
    pub fn with_multiples(self) -> Self {
        match self {
            Self::None => Self::Multiples,
            Self::PrimeFactors => Self::MultiplesAndPrimeFactors,
            other => other,
        }
    }

    pub fn with_prime_factors(self) -> Self {
        match self {
            Self::None => Self::PrimeFactors,
            Self::Multiples => Self::MultiplesAndPrimeFactors,
            other => other,
        }
    }

    pub fn uses_multiples(self) -> bool {
        matches!(self, Self::Multiples | Self::MultiplesAndPrimeFactors)
    }

    pub fn uses_prime_factors(self) -> bool {
        matches!(self, Self::PrimeFactors | Self::MultiplesAndPrimeFactors)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColumnRequestState {
    #[default]
    NotRequested,
    RequestedPendingResolution,
    Resolved,
}

impl ColumnRequestState {
    pub fn is_requested(self) -> bool {
        !matches!(self, Self::NotRequested)
    }

    pub fn is_pending(self) -> bool {
        matches!(self, Self::RequestedPendingResolution)
    }

    pub fn is_resolved(self) -> bool {
        matches!(self, Self::Resolved)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FractionInputVisibility {
    #[default]
    ShowInputs,
    HideInputs,
}

impl FractionInputVisibility {
    pub fn inputs_visible(self) -> bool {
        matches!(self, Self::ShowInputs)
    }
}
