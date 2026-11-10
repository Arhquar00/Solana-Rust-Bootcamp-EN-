//! ex02 - Functions and control flow.
//!
//! Read `tasks/ex02.md` first. Replace each `todo!()` with an implementation.

/// The base fee for a transaction, in lamports.
///
/// Solana charges 5_000 lamports per signature. This is only the base fee;
/// priority fees are added separately.
pub fn base_fee(signatures: u64) -> u64 {
    signatures * 5_000
}

/// Sum every amount in the slice.
///
/// An empty slice sums to zero.
pub fn total(amounts: &[u64]) -> u64 {
    let mut sum = 0;
    for amount in amounts {
        sum += *amount;
    }
    sum
}

/// Count how many amounts are strictly greater than `threshold`.
pub fn count_above(amounts: &[u64], threshold: u64) -> usize {
    let mut count = 0;
    for amount in amounts {
        if *amount > threshold {
            count += 1;
        }
    }
    count
}

/// Never transfer more than the balance.
///
/// Return the amount that can actually be transferred.
pub fn clamp_transfer(amount: u64, balance: u64) -> u64 {
    if amount > balance {
        balance
    } else {
        amount
    }
}
