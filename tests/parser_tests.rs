// All tests come from: https://github.com/ircdocs/parser-tests
// Written in 2015 by Daniel Oaks <daniel@danieloaks.net>
// Some tests originate from [Mozilla's test vectors](https://dxr.mozilla.org/comm-central/source/chat/protocols/irc/test/test_ircMessage.js), which are public domain.
// Some tests originate from [grawity's test vectors](https://github.com/grawity/irc-parse-tests) which were WTFPL v2 licensed when they were retrieved.
// Some tests originate from [Sadie's test vectors](https://github.com/SadieCat/ircparser-ruby/tree/master/test) which she's indicated I'm free to include here.

#[cfg(test)]
mod mask_match {
    // TODO: remname all of these tests to match the name of the function they're testing
    // As right now they say nothing descriptive
    fn mask_match_todo(_mask: &str, _pattern: &str) -> bool {
        todo!()
    }

    #[test]
    /// tests the mask *@127.0.0.1
    fn mask_one_test() {
        let mask = "*@127.0.0.1";

        // should match
        assert!(mask_match_todo(mask, "coolguy!ab@127.0.0.1"));
        assert!(mask_match_todo(mask, "cooldud3!~bc@127.0.0.1"));

        // should not match
        assert!(!mask_match_todo(mask, "coolguy!ab@127.0.0.1"));
        assert!(!mask_match_todo(mask, "cooldud3!~d@124.0.0.1"));
    }

    #[test]
    /// tests the mask cool*@*
    fn mask_two_test() {
        let mask = "cool*@*";
        // should match
        assert!(mask_match_todo(mask, "coolguy!ab@127.0.0.1"));
        assert!(mask_match_todo(mask, "cooldud3!~bc@127.0.0.1"));
        assert!(mask_match_todo(mask, "cool132!ab@example.com"));

        // should not match
        assert!(!mask_match_todo(mask, "koolguy!ab@127.0.0.5"));
        assert!(!mask_match_todo(mask, "cooodud3!~d@124.0.0.1"));
    }

    #[test]
    /// tests the mask cool!?username@*
    fn mask_three_test() {
        let mask = "cool!?username@*";

        // should match
        assert!(mask_match_todo(mask, "cool!ausername@127.0.0.1"));
        assert!(mask_match_todo(mask, "cool!~username@127.0.0.1"));

        // should not match
        assert!(!mask_match_todo(mask, "cool!username@127.0.0.1"));
    }

    #[test]
    /// tests the mask cool!a?*@*
    fn mask_four_test() {
        let mask = "cool!a?*@*";

        // should match
        assert!(mask_match_todo(mask, "cool!ab@127.0.0.1"));
        assert!(mask_match_todo(mask, "cool!abc@127.0.0.1"));

        // should not match
        assert!(!mask_match_todo(mask, "cool!a@127.0.0.1"));
    }

    #[test]
    /// tests the mask cool[guy]!*@*
    /// Cause failures in fnmatch/glob based matchers
    fn mask_five_test() {
        let mask = "cool[guy]!*@*";

        // should match
        assert!(mask_match_todo(mask, "cool[guy]!guy@127.0.0.1"));
        assert!(mask_match_todo(mask, "cool[guy]!a@example.com"));

        // should not match
        assert!(!mask_match_todo(mask, "coolg!ab@127.0.0.1"));
        assert!(!mask_match_todo(mask, "cool[!ac@127.0.1.1"));
    }
}
