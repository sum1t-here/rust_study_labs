fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if-let statement whose value is `Some`.

        // Use an `if let` statement to **destructure the Option**.
        // This checks if `optional_target` is `Some` and binds its value to `word`.
        // If it is `None`, the code inside the block would be skipped.
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.

        // Use a while-let statement to **pop elements from the vector**
        // Vec::pop() returns an Option<T> because the vector might be empty.
        // Here, T is Option<i8>, so pop() returns Option<Option<i8>> (nested Option).
        // We use nested pattern matching to only handle Some(Some(integer)).
        while let Some(Some(integer)) = optional_integers.pop() {
            // integer is the unwrapped i8 value
            assert_eq!(integer, cursor);
            cursor -= 1;
        }
        // After the loop, cursor should have counted down to 0
        assert_eq!(cursor, 0);
    }
}
