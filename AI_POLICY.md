# AI Policy

AI is allowed in this course. Copying an answer is not.

The rule is one sentence:

> **Ask the AI to explain the error, not to solve it.**

If you cannot explain your solution in a two-minute call, it does not count.
This is not about catching you. It is about making sure the skill stays in your
head after the cohort ends.

## What is allowed

- Ask why the compiler rejected a line, and ask what the error code means.
- Ask for a smaller example that demonstrates a concept.
- Ask for the difference between `&` and `&mut`, or `String` and `&str`.
- Ask it to review your explanation of a solution.
- Ask for a hint when you are stuck, then close the chat and write the code
  yourself.

Example of a good prompt:

```text
This is my code and this is the full compiler error. Explain what moved,
which line caused it, and what my options are. Do not give me the corrected
function.
```

## What is not allowed

- Pasting an exercise and asking for the complete solution.
- Copying code you do not understand into `src/`.
- Asking the AI to rewrite the tests so they pass.
- Using AI to write your two-minute explanation.
- Hiding AI use when a mentor asks how you solved something.

## How to use a hint well

1. Read the compiler error from top to bottom.
2. Write down, in your own words, what you think the error means.
3. Ask the AI to confirm or correct that explanation.
4. Close the chat.
5. Write the fix yourself, without the chat open.
6. Run the test.
7. Explain the fix out loud in one or two sentences.

If step 5 is impossible without the chat, you have not learned it yet. Go back
to the task and the Rust Book.

## A two-minute explanation

A mentor may ask you to explain any function you submitted. A passing
explanation covers:

- what the function receives and what it returns;
- why the parameters use `&`, `&mut`, or an owned value;
- what happens on the error path;
- why the test would fail if one line were removed.

If you can do that, the code is yours. If you cannot, the grade does not count,
even if the tests are green.

## Citing AI

If AI helped you understand a concept, say so in the pull request description:

```text
Used an AI chat to understand E0502. Wrote the fix myself.
```

That is honest and it is not a penalty. Claiming you wrote something you copied
is a penalty.

## The point

The compiler is your first auditor. AI can be a second tutor, but it cannot be
your hands. The goal of this repo is not 37 green checkmarks. The goal is to be
able to open a real Anchor program, read it, and change it without guessing.
