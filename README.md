# Rust Basics — Practice Repo

Eight small exercises, **37 tests**, and one goal: make you comfortable with the
Rust you need to read and write a Solana program.

This repository is **public**. You do not need an invitation, and you do not get
write access to it. **Fork it**, solve the exercises in your fork, and open a
pull request. The pull request is your submission; the instructor grades its
checks.

The exercises match the two decks:

| Deck | Exercises | Tests |
| --- | --- | ---: |
| Rust Basics 1 | ex01 – ex04 | 19 |
| Rust Basics 2 | ex05 – ex08 | 18 |
| **Total** | **ex01 – ex08** | **37** |

## Quick start

1. Click **Fork** at the top of this page. This makes your own copy on GitHub.
2. Clone **your fork**, not the class repository:

```bash
git clone https://github.com/<your-username>/Solana-Rust-Bootcamp-EN-.git
cd Solana-Rust-Bootcamp-EN-
```

3. Keep the class repository as `upstream`, so you can pull updates later:

```bash
git remote add upstream https://github.com/dev3pack/Solana-Rust-Bootcamp-EN-.git
```

4. Run the tests:

```bash
cargo test
```

The first run compiles the crate and fails every test, because every function
ends in `todo!()`. That is expected. Pick one exercise, read its task, and make
its tests pass.

## Work on your own branch

Create a branch in your fork. Do not work on `main`.

```bash
git checkout -b solutions/your-name
```

`main` stays as the starter. Your branch holds your answers. This also keeps
your work separate from everyone else's.

Run one exercise at a time while you work:

```bash
cargo test --test ex01
```

Run everything before you push:

```bash
cargo test
```

Push to **your fork**:

```bash
git add .
git commit -m "ex01-ex08 solutions"
git push -u origin solutions/your-name
```

You will see `unused variable` warnings from the starter functions. They
disappear as you replace each `todo!()` with a real implementation.

## Submit your work

After you push your branch, open a pull request:

1. Go to your fork on GitHub.
2. Click **Contribute** → **Open pull request**.
3. Base repository: `dev3pack/Solana-Rust-Bootcamp-EN-`.
4. Base branch: `main`.
5. Head repository: your fork.
6. Compare branch: `solutions/your-name`.
7. Title: `Solutions — your name`.
8. Click **Create pull request**.

Do **not** merge the pull request. It is a submission, not a change to the class
repo. The instructor grades the checks and then closes it.

Your fork is public, like this repository. Do not put passwords, private keys, or
personal data in it.

## What is in the repo

```
src/
  lib.rs        the crate root; declares the eight modules
  ex01.rs       variables and types
  ex02.rs       functions and control flow
  ex03.rs       ownership
  ex04.rs       borrowing
  ex05.rs       structs and impl
  ex06.rs       enums and match
  ex07.rs       Result and ?
  ex08.rs       capstone: mini bank
tests/
  ex01.rs ... ex08.rs   the 37 tests; do not edit
tasks/
  ex01.md ... ex08.md   what to build in each exercise
.github/workflows/tests.yml
tests.sha256             checksums used by CI to detect edited tests
AI_POLICY.md
```

## How to work

1. Read `tasks/exNN.md`.
2. Open `src/exNN.rs`.
3. Replace one `todo!()` at a time.
4. Run `cargo test --test exNN`.
5. Read the compiler error from top to bottom: error code, arrow, help.
6. When the exercise is green, move to the next one.
7. When all 37 tests pass, push your branch and open the pull request.

The tests are the specification. If a test and a task description disagree,
the test wins. If you believe a test is wrong, ask the instructor; do not edit
it.

## Rules

- **Keep your changes in `src/`.** Do not edit `tests/`, `tests.sha256`, or
  `.github/`.
- **Do not edit anything in `tests/`.** CI runs `sha256sum -c tests.sha256`
  before the tests. A changed test file fails the build even if the code passes.
- **Do not delete or rename tests.** The grader counts passed tests out of 37.
- **No `HashMap` and no traits in ex08.** The capstone uses a `Vec<String>` for
  history on purpose. Keep it simple.
- **No panics for expected failures.** Use `Result` in ex07 and ex08. A
  `todo!()`, `unwrap()` on user input, or `panic!()` is not error handling.
- **AI is allowed as a tutor, not as a solution machine.** Read `AI_POLICY.md`.
  If you cannot explain your code in a two-minute call, it does not count.

## Grading

Each test is worth one point. The automatic grade is:

```
points = number of passing tests
```

Your part is graded out of the tests for that part: 19 for Rust Basics 1 and 18
for Rust Basics 2. A mentor also samples solutions for unneeded `unwrap()` and
`clone()` calls, and may ask you to explain a solution. Passing tests with code
you cannot explain is not a pass.

The grade comes from the checks on your **pull request**, not from a push
directly to the class repository.

## CI and the checksum

Every pull request to the class repo runs:

1. `sha256sum -c tests.sha256` — verifies that the test files are unchanged.
2. `cargo test --locked` — runs all 37 tests.

Pushes to your fork may run the workflow too, if Actions are enabled there. The
pull request run is the one that counts. If this is your first pull request,
GitHub may ask the instructor to approve the workflow; that is normal.

CI is red at the start, because `todo!()` panics. It turns green when all 37
tests pass. To run the checksum yourself on macOS:

```bash
shasum -a 256 -c tests.sha256
```

If the checksum step fails, restore the tests:

```bash
git checkout -- tests/
```

Then run the tests again. If you added a test of your own, put it in a separate
file outside `tests/` or remove it before opening the pull request; only the
official 37 tests count.

## Common compiler errors

| Error | What it means | First thing to try |
| --- | --- | --- |
| `E0382` | You used a value after it moved | Borrow it with `&`, or clone it if you really need two owners |
| `E0502` | A read and a write are alive at the same time | Finish the read before you write |
| `E0308` | Mismatched types | Compare the function signature with the value you return |
| `E0004` | A `match` is missing a case | Add the missing arm, or use `_` only when a catch-all is correct |
| `E0499` | More than one mutable borrow | Keep one `&mut` alive at a time |
| `E0596` | Cannot borrow as mutable | Declare the binding with `let mut` |

`todo!()` is not a compiler error. It compiles and panics at runtime with
"not yet implemented". If you see that panic, the function is still a stub.

## Resources

- The Rust Book, chapters 3 to 9: <https://doc.rust-lang.org/book/>
- Rustlings, short drills: <https://github.com/rust-lang/rustlings>
- Rust standard library: <https://doc.rust-lang.org/std/>
- Solana docs: <https://solana.com/docs>
- Anchor docs: <https://www.anchor-lang.com/docs>

## Questions

Open an issue in the **class repository**:
<https://github.com/dev3pack/Solana-Rust-Bootcamp-EN-/issues>

Issues opened in your own fork are not seen by the instructor. Include the
exercise number, the exact command you ran, and the full compiler error. Do not
paste only the last line; the first line and the arrow matter.
