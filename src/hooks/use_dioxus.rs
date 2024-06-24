use dioxus_lib::prelude::{Writable, use_signal, ReadOnlySignal, Readable};

/// Alternative to [use_memo]
/// Benefits:
/// - No unnecessary rerenders
/// Downsides:
/// - T needs to be Clone (cannot be avoided)
pub fn use_computed<T: 'static + Clone, D: PartialEq + 'static>(
    deps: D,
    init: impl FnOnce() -> T,
) -> T {
    use_computed_with_prev(deps, |_| init())
}


/// Alternative to [use_memo]
/// Benefits:
/// - No unnecessary rerenders
/// - Prev value is `T` instead of `&mut T`
/// Downsides:
/// - T needs to be Clone (cannot be avoided)
pub fn use_computed_with_prev<T: 'static + Clone, D: PartialEq + 'static>(
    deps: D,
    init: impl FnOnce(Option<T>) -> T,
) -> T {
    struct Memoized<T, D> {
        value: T,
        deps: D,
    }
    let mut memo_signal = use_signal::<Option<Memoized<T, D>>>(|| None);
    let mut memo = memo_signal.write();

    let deps_have_changed = memo
        .as_ref()
        .map(|memo| &memo.deps)
        != Some(&deps);

    let new_value = if deps_have_changed {
        let prev_value = memo
            .take()
            .map(|memo| memo.value);
        Some(init(prev_value))
    } else {
        None
    };

    if let Some(new_value) = new_value {
        let new_memoized_value = Memoized {
            value: new_value,
            deps,
        };
        *memo = Some(new_memoized_value);
    }

    memo.as_ref().unwrap().value.clone()
}



/// Alternative to [use_memo]
/// Benefits:
/// - No unnecessary rerenders
/// Downsides:
/// - D needs to be Clone (cannot be avoided)
pub fn use_computed_signal<T: 'static, D: PartialEq + Clone + 'static>(
    deps: D,
    init: impl Fn() -> T,
) -> ReadOnlySignal<T> {
    use_computed_signal_with_prev(deps, move |_| init())
}


/// Alternative to [use_memo]
/// Benefits:
/// - No unnecessary rerenders
/// - Access the previous computed value
/// Downsides:
/// - D needs to be Clone (cannot be avoided)
/// - Prev value is `&mut T` instead of `T`
pub fn use_computed_signal_with_prev<T: 'static, D: PartialEq + Clone + 'static>(
    deps: D,
    init: impl Fn(Option<&mut T>) -> T,
) -> ReadOnlySignal<T> {
    let mut deps_signal = use_signal::<D>(|| deps.clone());
    let mut value_signal = use_signal::<T>(|| init(None));

    let deps_have_changed = *deps_signal.peek() != deps;

   if deps_have_changed {
        let mut memoized_deps = deps_signal.write();
        let mut memoized_value = value_signal.write();

        let new_value = init(Some(&mut *memoized_value));

        *memoized_value = new_value;
        *memoized_deps = deps;
    }


    value_signal.into()
}