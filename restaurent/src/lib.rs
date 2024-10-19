mod front_of_house {
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }

    mod hosting {
        fn greet(){}
        fn assign_seat(){}
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
