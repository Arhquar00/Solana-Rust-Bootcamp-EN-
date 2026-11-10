//! ex04 - Borrowing.
//!
//! Read `tasks/ex04.md` first. Replace each `todo!()` with an implementation.

/// Read a slice of amounts and return their sum.
///
/// The slice is borrowed, so the caller keeps ownership.
pub fn total(amounts: &[u64]) -> u64 {
    let mut sum = 0;
    for amount in amounts {
        sum += *amount;
    }
    sum
}

/// Add a fee to a balance through a mutable reference.
///
/// The function returns nothing. The caller's value must change.
pub fn add_fee(amount: &mut u64, fee: u64) {
    *amount += fee;
}

/// Count how many accounts are writable.
pub fn count_writable(flags: &[bool]) -> usize {
    let mut count = 0;
    for flag in flags {
        if *flag {
            count += 1;
        }
    }
    count
}

/// Append a note to a history through a mutable reference.
pub fn append_note(history: &mut Vec<String>, note: &str) {
    history.push(note.to_string());
}
