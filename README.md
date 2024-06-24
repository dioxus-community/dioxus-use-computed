[![Discord Server](https://img.shields.io/discord/899851952891002890.svg?logo=discord&style=flat-square)](https://discord.gg/sKJSVNSCDJ)

# `dioxus-use-computed` 🦀🧠

Alternative to the built-in [Dioxus](https://dioxuslabs.com/) hooks `use_memo` and `use_reactive`.

The main idea is to make resource-expensive computations in the most efficient way possible. Avoiding unnecessary rerenders and wrappers.

```rs
fn app() -> Element {
    let mut value = use_signal(|| 2);

    rsx!(
        Counter {
            value: value()
        }
        PrevCounter {
            value: value()
        }
        CounterSignal {
            value: value()
        }
        PrevCounterSignal {
            value: value()
        }
    )
}

#[component]
fn Counter(value: usize) -> Element {
    let double = use_computed(value, || value * 2);

    rsx!( p {  "Double: {double}" } )
}

#[component]
fn PrevCounter(value: usize) -> Element {
    let double = use_computed_with_prev(value, |prev_value| prev_value.unwrap_or(value) * 2);

    rsx!( p { "Previous Double: {double}" } )
}

#[component]
fn CounterSignal(value: usize) -> Element {
    let double = use_computed_signal(value, || value * 2);

    rsx!( p { "Double Signal: {double}" } )
}

#[component]
fn PrevCounterSignal(value: usize) -> Element {
    let double = use_computed_signal_with_prev(value, |prev| prev.cloned().unwrap_or(value) * 2);

    rsx!( p { "Previous Double Signal: {double}" } )
}
```