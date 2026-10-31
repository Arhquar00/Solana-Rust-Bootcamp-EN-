//! ex01 - Variables and types.
//!
//! Read `tasks/ex01.md` first. Replace each `todo!()` with an implementation.

/// The number of lamports in one SOL.
///
/// In a real Solana program this constant comes from
/// `solana_program::native_token::LAMPORTS_PER_SOL`. Here we define it locally
/// so the exercise has no dependencies.
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

/// Convert whole SOL into lamports.
///
/// `sol_to_lamports(2)` should return `2_000_000_000`.
pub fn sol_to_lamports(sol: u64) -> u64 {
    todo!()
}

/// Convert lamports into whole SOL, dropping any remainder.
///
/// `lamports_to_sol(2_500_000_000)` should return `2`.
pub fn lamports_to_sol(lamports: u64) -> u64 {
    todo!()
}

/// Add a bonus to a balance and return the new balance.
pub fn apply_bonus(balance: u64, bonus: u64) -> u64 {
    todo!()
}

/// Double a value.
///
/// Use shadowing in your implementation: declare a new `value` instead of
/// mutating the parameter.
pub fn double(value: u64) -> u64 {
    todo!()
}

/// Return `true` when the balance is zero.
pub fn is_empty(balance: u64) -> bool {
    todo!()
}
