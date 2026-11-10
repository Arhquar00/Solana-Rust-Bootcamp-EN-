//! ex07 - `Result` and the `?` operator.
//!
//! Read `tasks/ex07.md` first. Replace each `todo!()` with an implementation.

/// Everything that can go wrong in a transfer.
#[derive(Debug, PartialEq)]
pub enum TransferError {
    ZeroAmount,
    SameAccount,
    InsufficientFunds { available: u64, required: u64 },
}

/// Check the rules that do not depend on the balance.
///
/// Return, in this order:
/// 1. `ZeroAmount` when `amount == 0`
/// 2. `SameAccount` when `from == to`
/// 3. `Ok(())` otherwise
pub fn validate(from: &str, to: &str, amount: u64) -> Result<(), TransferError> {
    if amount == 0 {
        return Err(TransferError::ZeroAmount);
    }
    if from == to {
        return Err(TransferError::SameAccount);
    }
    Ok(())
}

/// Validate a transfer and return the sender's remaining balance.
///
/// Call `validate` first and propagate its error with `?`. Then return
/// `InsufficientFunds` when `amount > balance`. On success return
/// `Ok(balance - amount)`.
pub fn transfer(
    balance: u64,
    from: &str,
    to: &str,
    amount: u64,
) -> Result<u64, TransferError> {
    validate(from, to, amount)?;
    if amount > balance {
        return Err(TransferError::InsufficientFunds {
            available: balance,
            required: amount,
        });
    }
    Ok(balance - amount)
}

/// Pay through `transfer` and propagate any error with `?`.
///
/// This function exists to show the shape of a function that uses `?`.
pub fn pay(balance: u64, from: &str, to: &str, amount: u64) -> Result<u64, TransferError> {
    let remaining = transfer(balance, from, to, amount)?;
    Ok(remaining)
}
