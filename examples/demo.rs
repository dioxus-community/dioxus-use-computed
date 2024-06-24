use dioxus::prelude::*;
use dioxus_use_computed::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut value = use_signal(|| 2);

    rsx!(
        p {
            "Value: {value}"
        }
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
        button {
            onclick: move |_| value += 1,
            "Increase"
        }
    )
}

#[component]
fn Counter(value: usize) -> Element {
    let double = use_computed(value, || value * 2);

    rsx!(
        p {
            "Double: {double}"
        }
    )
}

#[component]
fn PrevCounter(value: usize) -> Element {
    let double = use_computed_with_prev(value, |prev_value| prev_value.unwrap_or(value) * 2);

    rsx!(
        p {
            "Previous Double: {double}"
        }
    )
}

#[component]
fn CounterSignal(value: usize) -> Element {
    let double = use_computed_signal(value, || value * 2);

    rsx!(
        p {
            "Double Signal: {double}"
        }
    )
}

#[component]
fn PrevCounterSignal(value: usize) -> Element {
    let double = use_computed_signal_with_prev(value, |prev| prev.cloned().unwrap_or(value) * 2);

    rsx!(
        p {
            "Previous Double Signal: {double}"
        }
    )
}
