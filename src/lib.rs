// pub mod add_two_numbers;
pub mod longest_substring_without_repeating_characters;

#[cfg(test)]
mod tests {
    // mod add_two_numbers {
    //     use crate::add_two_numbers;

    //     #[test]
    //     fn example1() {
    //     }
    // }

    mod longest_substring_without_repeating_characters {
        use crate::longest_substring_without_repeating_characters::length_of_longest_substring;

        #[test]
        fn example1() {
            assert_eq!(length_of_longest_substring("abcabcbb".into()), 3);
        }

        #[test]
        fn example2() {
            assert_eq!(length_of_longest_substring("bbbbb".into()), 1);
        }

        #[test]
        fn example3() {
            assert_eq!(length_of_longest_substring("pwwkew".into()), 3);
        }

        #[test]
        fn example4() {
            assert_eq!(length_of_longest_substring("z,jsdfzkhsdflkjshdfsjkhsdkzxcvbnm,asdkfjgh;qpwoeirutyjfahlskjdhfkjasdghjbasdfsbjhbfkjbshekrgbwriurwtjuoirytwue".into()), 27);
        }
    }
}
