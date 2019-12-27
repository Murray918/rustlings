// option1.rs
// This example panics because the second time it calls `pop`, the `vec`
// is empty, so `pop` returns `None`, and `unwrap` panics if it's called
// on `None`. Handle this in a more graceful way than calling `unwrap`!
// Scroll down for hints :)

pub fn pop_too_much() -> bool {
    let mut list = vec![3];
    let default = 0;
    let last = list.pop().unwrap_or(default);
    println!("The last item in the list is {:?}", last);

    let second_to_last = list.pop().unwrap_or(default);
    println!(
        "The second-to-last item in the list is {:?}",
        second_to_last
    );
    if last | second_to_last > default {
         true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_panic() {
        assert!(pop_too_much());
    }
}

// Try using a `match` statement where the arms are `Some(thing)` and `None`.
// Or set a default value to print out if you get `None` by using the
// function `unwrap_or`.
// Or use an `if let` statement on the result of `pop()` to both destructure
// a `Some` value and only print out something if we have a value!
