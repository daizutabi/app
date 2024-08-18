use leptos::{
    ev,
    html::{button, div, span},
    prelude::*,
};

/// A simple counter view.
// A component is really just a function call: it runs once to create the DOM and reactive system
pub fn counter(initial_value: i32, step: u32) -> impl IntoView {
    let count = RwSignal::new(Count::new(initial_value, step));
    Effect::new(move |_| {
        leptos::logging::log!("count = {:?}", count.get());
    });

    // the function name is the same as the HTML tag name
    div()
        // children can be added with .child()
        // this takes any type that implements IntoView as its argument
        // for example, a string or an HtmlElement<_>
        // it can also take an array of types that impl IntoView
        // or a tuple of up to 26 objects that impl IntoView
        .child((
            button()
                // typed events found in leptos::ev
                // 1) prevent typos in event names
                // 2) allow for correct type inference in callbacks
                .on(ev::click, move |_| count.update(Count::clear))
                .child("Clear"),
            button()
                .on(ev::click, move |_| count.update(Count::decrease))
                .child("-1"),
            span().child(("Value: ", move || count.get().value(), "!")),
            button()
                .on(ev::click, move |_| count.update(Count::increase))
                .child("+1"),
        ))
}

#[derive(Debug, Clone)]
pub struct Count {
    value: i32,
    step: i32,
}

impl Count {
    pub fn new(value: i32, step: u32) -> Self {
        Count {
            value,
            step: step as i32,
        }
    }

    pub fn value(&self) -> i32 {
        leptos::logging::log!("value = {}", self.value);
        self.value
    }

    pub fn increase(&mut self) {
        self.value += self.step;
    }

    pub fn decrease(&mut self) {
        self.value += -self.step;
    }

    pub fn clear(&mut self) {
        self.value = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(-2, 1)]
    #[case(0, 1)]
    #[case(2, 1)]
    #[case(3, 2)]
    #[case(4, 3)]
    fn should_increase_count(#[case] initial_value: i32, #[case] step: u32) {
        let mut count = Count::new(initial_value, step);
        count.increase();
        assert_eq!(count.value(), initial_value + step as i32);
    }

    #[rstest]
    #[case(-2, 1)]
    #[case(0, 1)]
    #[case(2, 1)]
    #[case(3, 2)]
    #[case(4, 3)]
    #[trace]
    fn should_decrease_count(#[case] initial_value: i32, #[case] step: u32) {
        let mut count = Count::new(initial_value, step);
        count.decrease();
        assert_eq!(count.value(), initial_value - step as i32);
    }

    #[rstest]
    #[case(-2, 1)]
    #[case(0, 1)]
    #[case(2, 1)]
    #[case(3, 2)]
    #[case(4, 3)]
    #[trace]
    fn should_clear_count(#[case] initial_value: i32, #[case] step: u32) {
        let mut count = Count::new(initial_value, step);
        count.clear();
        assert_eq!(count.value(), 0);
    }
}
