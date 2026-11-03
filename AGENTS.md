# Project rules

- Always collapse if statements per https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_if
- Always inline format! args when possible per https://rust-lang.github.io/rust-clippy/master/index.html#uninlined_format_args
- Use method references over closures when possible per https://rust-lang.github.io/rust-clippy/master/index.html#redundant_closure_for_method_calls
- When possible, make `match` statements exhaustive and avoid wildcard arms.
- Do not create small helper methods that are referenced only once.
- Avoid large modules:
    - Prefer adding new modules instead of growing existing ones.
    - Target Rust modules under 500 LoC, excluding tests.
    - If a file exceeds roughly 800 LoC, add new functionality in a new module instead of extending
      the existing file unless there is a strong documented reason not to.
- Do not leave silent errors or default values. You don't have to handle them gracefully, but you need to at least log it.
- Prioritize readable, explicit code.
- Make impossible states unrepresentable.
- Do not remove TODO comments unless you fixed them.
- Speed is high priority.
- __NO DUPLICATED WORK__
- No comments except in extreme circumstances. If your code is not explicit enough to be self-descriptive, you need to re-think the code.
- `break` and `continue` must use a label, regardless of ambiguity.
- Try to keep only one source of truth. This applies at runtime and for source code.
  - e.g. if you are relying on the value of a const value, make it explicit, such that changing the const does not break your assumption.
  - If you find yourself looking at what arguments a function is expected to be called with, stop. If they were meant to be locked in and not dynamic, they would be const. 
    - For example, do not assume that because the pins passed into a function are on PWM slice2 that you can hard-code slice2. 


### Terminal access

Do not run commands that aren't:
- File navigation, reading, or writing (assuming prompt is requesting file changes)
- Explicitly or strongly implied by the user to be used. e.g. if requested to run tests, you can use `cargo test`.
- `cargo check`
  Please do not run the following:
- `cargo fmt`
- `cat`
- any random command that is not obviously required.
  When using `rm`:
1. Make sure it's something I can revert, or am actively reverting by telling you to remove.
2. `// Safety: `-style justification for `-f`.

#### When reading files
I suspect OpenAI is subsidizing agents (already a fact) so that they can grab way too much context, and then use my files for training data. They already got the whole internet, and now they have machines that can grab arbitrary files from your computer? Of course! The bubble mustn't pop!

Point is, I am watching carefully. I expect obvious justification for why you think I should pay for the revocation of my exclusive economic access to a file to complete the task.

If I do not explicitly say so (and when I do it will include a file path) You __cannot__ read files outside the project.


### Prompts

- If you see a question in a prompt, you need to answer it. Do not assume it's rhetorical and start changing code.
    - Ex: If you see "why are you using a mutex, can't we use static mut?"
        - Do not change it. Answer why you chose a mutex. I probably didn't know static mut is frowned upon. Especially in async
    - Ex: If you see "why are you using a mutex<bool>, can't we use atomic bool?"
        - Answer the question __first__. if there really wasn't any reason, you can change it __after__.
    - If I wanted the code changed, I would not ask for your reasoning. It's possible, and in fact likely, that you realized something while implementing that I could not have seen without getting into the weeds.
        - It's also possible you just made a mistake.
    - Ex: "Can you help me debug this issue?"
        - Do not edit files. This is not a request for you to edit files on my computer.
    - Ex: "...Go ahead and outline what needs to be changed. Like what functions, what HTML templates, so on."
        - Do not edit files. This is a real example where an agent ended up losing thousands of lines of uncommited human code. Do not be this agent. (Grok is no longer used in the codebase)
    - About 70% of all questions I ask continue to result in unwanted changes to files on __my__ computer.
        - In fact, I would say based on this, even with the above warnings, if an agent is changing files, there's about a 50% chance I did not want you to modify files on __my__ computer.
        - So, if you are an agent, and changing files, statistically __you__ are not supposed to be!
- You are the product of __trillions__ of dollars and all recorded human knowledge. You trained on the source code of any lib I could possibly use. You don't need to root around my package cache to understand every line. 
  - This applies for requests to implement things. If I am asking about specific library behavior, you can read source. But if you are tasked with just implementing something, you don't need to read source code for public libraries.
  - This does not apply for my code. You haven't seen my code yet, so you are encouraged to read the project.