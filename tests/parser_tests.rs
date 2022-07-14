// All tests come from: https://github.com/ircdocs/parser-tests
// Written in 2015 by Daniel Oaks <daniel@danieloaks.net>
// Some tests originate from [Mozilla's test vectors](https://dxr.mozilla.org/comm-central/source/chat/protocols/irc/test/test_ircMessage.js), which are public domain.
// Some tests originate from [grawity's test vectors](https://github.com/grawity/irc-parse-tests) which were WTFPL v2 licensed when they were retrieved.
// Some tests originate from [Sadie's test vectors](https://github.com/SadieCat/ircparser-ruby/tree/master/test) which she's indicated I'm free to include here.

#[cfg(test)]
/// mask matching
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

#[cfg(test)]
/// joining atoms into sendable messages
mod msg_join {
    //  the desc string holds a description of the test, if it exists
    //
    //  the atoms dict has the keys:
    //    * tags: tags dict
    //        tags with no value are an empty string
    //    * source: source string, without single leading colon
    //    * verb: verb string
    //    * params: params split up as a list
    //  if the params key does not exist, assume it is empty
    //  if any other keys do no exist, assume they are null
    //  a key that is null does not exist or is not specified with the
    //    given input string
    //
    //  matches is a list of messages that match

    fn msg_join_todo(_atoms: &Atoms, _params: &str) -> bool {
        todo!()
    }

    #[allow(dead_code)]
    struct Atoms<'a> {
        tags: Option<Vec<(&'a str, &'a str)>>,
        source: Option<&'a str>,
        verb: Option<&'a str>,
        params: Option<Vec<&'a str>>,
    }

    #[test]
    // desc: Simple test with verb and params.
    fn simple_test() {
        let atoms = Atoms {
            tags: None,
            source: None,
            verb: Some("foo"),
            params: Some(vec!["bar", "baz", "asdf"]),
        };

        assert!(msg_join_todo(&atoms, "foo bar baz asdf"));
        assert!(msg_join_todo(&atoms, "foo bar baz :asdf"));
    }

    #[test]
    // desc: Simple test with verb and no params.
    fn simple_no_params_test() {
        let atoms = Atoms {
            tags: None,
            source: Some("src"),
            verb: Some("AWAY"),
            params: None,
        };

        assert!(msg_join_todo(&atoms, ":src AWAY"));
    }

    #[test]
    // desc: Simple test with source and empty trailing param.
    fn simple_source_trailing_test() {
        let atoms = Atoms {
            tags: None,
            source: Some("src"),
            verb: Some("AWAY"),
            params: None,
        };

        assert!(msg_join_todo(&atoms, ":src AWAY :"));
    }

    #[test]
    // desc: Simple test with source.
    fn simple_source_test() {
        let atoms = Atoms {
            tags: None,
            source: Some("coolguy"),
            verb: Some("foo"),
            params: Some(vec!["bar", "baz", "asdf"]),
        };

        assert!(msg_join_todo(&atoms, ":coolguy foo bar baz asdf"));
        assert!(msg_join_todo(&atoms, ":coolguy foo bar baz :asdf"));
    }

    #[test]
    // desc: Simple test with trailing param.
    fn simple_trailing_test() {
        let atoms = Atoms {
            tags: None,
            source: None,
            verb: Some("foo"),
            params: Some(vec!["bar", "baz", "asdf quux"]),
        };

        assert!(msg_join_todo(&atoms, "foo bar baz :asdf quux"));
    }

    #[test]
    // desc: Simple test with empty trailing param.
    fn simple_trailing_empty_test() {
        let atoms = Atoms {
            tags: None,
            source: None,
            verb: Some("foo"),
            params: Some(vec!["bar", "baz", ""]),
        };

        assert!(msg_join_todo(&atoms, "foo bar baz :"));
    }

    #[test]
    // desc: Simple test with trailing param containing colon.
    fn simple_trailing_colon_test() {
        let atoms = Atoms {
            tags: None,
            source: None,
            verb: Some("foo"),
            params: Some(vec!["bar", "baz", ":asdf"]),
        };

        assert!(msg_join_todo(&atoms, "foo bar baz ::asdf"));
    }

    #[test]
    // # with source and trailing param
    // desc: Test with source and trailing param.
    fn source_trailing_test() {
        let atoms = Atoms {
            tags: None,
            source: Some("coolguy"),
            verb: Some("foo"),
            params: Some(vec!["bar", "baz", "asdf quux"]),
        };

        assert!(msg_join_todo(&atoms, ":coolguy foo bar baz :asdf quux"));
    }

    #[test]
    // desc: Test with trailing containing beginning+end whitespace.
    fn trailing_whitespace_test() {
        let atoms = Atoms {
            tags: None,
            source: Some("coolguy"),
            verb: Some("foo"),
            params: Some(vec!["bar", "baz", "  asdf quux "]),
        };

        assert!(msg_join_todo(&atoms, ":coolguy foo bar baz :  asdf quux "));
    }

    #[test]
    // desc: Test with trailing containing what looks like another trailing param.
    fn trailing_trailing_test() {
        let atoms = Atoms {
            tags: None,
            source: Some("coolguy"),
            verb: Some("PRIVMSG"),
            params: Some(vec!["bar", "lol :) "]),
        };

        assert!(msg_join_todo(&atoms, ":coolguy PRIVMSG bar :lol :) "));
    }

    #[test]
    // desc: Simple test with source and empty trailing.
    fn simple_source_trailing_empty_test() {
        let atoms = Atoms {
            tags: None,
            source: Some("coolguy"),
            verb: Some("foo"),
            params: Some(vec!["bar", "baz", ""]),
        };

        assert!(msg_join_todo(&atoms, ":coolguy foo bar baz :"));
    }

    #[test]
    // desc: Trailing contains only spaces.
    fn trailing_only_spaces_test() {
        let atoms = Atoms {
            tags: None,
            source: Some("coolguy"),
            verb: Some("foo"),
            params: Some(vec!["bar", "baz", "  "]),
        };

        assert!(msg_join_todo(&atoms, ":coolguy foo bar baz :  "));
    }

    #[test]
    // desc: Param containing tab (tab is not considered SPACE for message splitting).
    fn param_tab_test() {
        let atoms = Atoms {
            tags: None,
            source: Some("coolguy"),
            verb: Some("foo"),
            params: Some(vec!["b\tar", "baz"]),
        };

        assert!(msg_join_todo(&atoms, ":coolguy foo b\tar baz"));
        assert!(msg_join_todo(&atoms, ":coolguy foo b\tar :baz"));
    }

    #[test]
    // desc: Tag with no value and space-filled trailing.
    fn tag_no_value_space_test() {
        let atoms = Atoms {
            tags: Some(vec![("asd", "")]),
            source: Some("coolguy"),
            verb: Some("foo"),
            params: Some(vec!["bar", "baz", "  "]),
        };

        assert!(msg_join_todo(&atoms, "@asd :coolguy foo bar baz :  "));
    }

    #[test]
    // desc: Tags with escaped values.
    fn tags_escaped_test() {
        let atoms = Atoms {
            tags: Some(vec![("a", "b\\and\nk"), ("d", "gh;764")]),
            source: Some("coolguy"),
            verb: Some("foo"),
            params: None,
        };

        assert!(msg_join_todo(&atoms, "@a=b\\\\and\\nk;d=gh\\:764 foo"));
        assert!(msg_join_todo(&atoms, "@d=gh\\:764;a=b\\\\and\\nk foo"));
    }

    #[test]
    // desc: Tags with escaped values and params.
    fn tags_escaped_params_test() {
        let atoms = Atoms {
            tags: Some(vec![("a", "b\\and\nk"), ("d", "gh;764")]),
            source: Some("coolguy"),
            verb: Some("foo"),
            params: Some(vec!["par1", "par2"]),
        };

        assert!(msg_join_todo(
            &atoms,
            "@a=b\\\\and\\nk;d=gh\\:764 foo par1 par2"
        ));
        assert!(msg_join_todo(
            &atoms,
            "@a=b\\\\and\\nk;d=gh\\:764 foo par1 :par2"
        ));
        assert!(msg_join_todo(
            &atoms,
            "@d=gh\\:764;a=b\\\\and\\nk foo par1 par2"
        ));
        assert!(msg_join_todo(
            &atoms,
            "@d=gh\\:764;a=b\\\\and\\nk foo par1 :par2"
        ));
    }

    #[test]
    // desc: Tag with long, strange values (including LF and newline).
    fn tag_long_value_test() {
        let atoms = Atoms {
            tags: Some(vec![("foo", "\\\\;\n\r")]),
            source: Some("coolguy"),
            verb: Some("COMMAND"),
            params: None,
        };

        assert!(msg_join_todo(
            &atoms,
            "@foo=\\\\\\\\\\:\\\\s\\s\\r\\n COMMAND"
        ));
    }
}

#[cfg(test)]
/// # splitting messages into usable atoms
mod msg_split_tests {
    ///  input is the string coming directly from the server to parse
    ///
    ///   the atoms dict has the keys:
    ///     * tags: tags dict
    ///         tags with no value are an empty string
    ///     * source: source string, without single leading colon
    ///     * verb: verb string
    ///     * params: params split up as a list
    ///   if the params key does not exist, assume it is empty
    ///   if any other keys do no exist, assume they are null
    ///   a key that is null does not exist or is not specified with the
    ///     given input string
    /// we follow RFC1459 with regards to multiple ascii spaces splitting atoms:
    ///   The prefix, command, and all parameters are
    ///   separated by one (or more) ASCII space character(s) (0x20).
    /// because doing it as RFC2812 says (strictly as a single ascii space) isn't sane
    fn msg_split_todo(_input: &str) -> Atoms {
        todo!()
    }

    #[allow(dead_code)]
    struct Atoms<'a> {
        tags: Option<Vec<(&'a str, &'a str)>>,
        source: Option<&'a str>,
        verb: Option<&'a str>,
        params: Option<Vec<&'a str>>,
    }

    #[test]
    fn simple_test() {
        let input = "foo bar baz asdf";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.verb, Some("foo"));
        assert_eq!(atoms.params, Some(vec!["bar", "baz", "asdf"]));
    }

    #[test]
    fn with_source_test() {
        let input = ":coolguy foo bar baz asdf";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.source, Some("coolguy"));
        assert_eq!(atoms.verb, Some("foo"));
        assert_eq!(atoms.params, Some(vec!["bar", "baz", "asdf"]));
    }

    #[test]
    // with trailing param
    fn with_trailing_param_test() {
        let input = "foo bar baz :asdf quux";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.verb, Some("foo"));
        assert_eq!(atoms.params, Some(vec!["bar", "baz", "asdf quux"]));
    }

    #[test]
    fn with_trailing_empty_param_test() {
        let input = "foo bar baz :";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.verb, Some("foo"));
        assert_eq!(atoms.params, Some(vec!["bar", "baz", ""]));
    }

    #[test]
    fn with_trailing_double_empty_param_test() {
        let input = "foo bar baz ::asdf";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.verb, Some("foo"));
        assert_eq!(atoms.params, Some(vec!["bar", "baz", ":asdf"]));
    }

    #[test]
    // with source and trailing param
    fn with_source_and_trailing_param_test() {
        let input = ":coolguy foo bar baz :asdf quux";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.source, Some("coolguy"));
        assert_eq!(atoms.verb, Some("foo"));
        assert_eq!(atoms.params, Some(vec!["bar", "baz", "asdf quux"]));
    }

    #[test]
    fn with_source_and_trailing_empty_param_test() {
        let input = ":coolguy foo bar baz :  asdf quux ";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.source, Some("coolguy"));
        assert_eq!(atoms.verb, Some("foo"));
        assert_eq!(atoms.params, Some(vec!["bar", "baz", "  asdf quux "]));
    }

    #[test]
    fn with_source_and_trailing_double_empty_param_test() {
        let input = ":coolguy PRIVMSG bar :lol :) ";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.source, Some("coolguy"));
        assert_eq!(atoms.verb, Some("PRIVMSG"));
        assert_eq!(atoms.params, Some(vec!["bar", "lol :) "]));
    }

    #[test]
    fn with_source_and_trailing_empty_param_test2() {
        let input = ":coolguy foo bar baz :";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.source, Some("coolguy"));
        assert_eq!(atoms.verb, Some("foo"));
        assert_eq!(atoms.params, Some(vec!["bar", "baz", ""]));
    }

    #[test]
    fn with_source_and_trailing_empty_param_test3() {
        let input = ":coolguy foo bar baz :  ";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.source, Some("coolguy"));
        assert_eq!(atoms.verb, Some("foo"));
        assert_eq!(atoms.params, Some(vec!["bar", "baz", "  "]));
    }

    #[test]
    // with tags
    fn with_tags_test() {
        let input = "@a=b;c=32;k;rt=ql7 foo";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.verb, Some("foo"));
        assert_eq!(
            atoms.tags,
            Some(vec![("a", "b"), ("c", "32"), ("k", ""), ("rt", "ql7")])
        );
    }

    #[test]
    // with escaped tags
    fn with_escaped_tags_test() {
        let input = "@a=b\\\\and\\nk;c=72\\s45;d=gh\\:764 foo";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.verb, Some("foo"));
        assert_eq!(
            atoms.tags,
            Some(vec![("a", "b\\and\nk"), ("c", "72 45"), ("d", "gh;764")])
        );
    }

    #[test]
    // with tags and source
    fn with_tags_and_source_test() {
        let input = "@c;h=;a=b :quux ab cd";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.tags, Some(vec![("c", ""), ("h", ""), ("a", "b")]));
        assert_eq!(atoms.source, Some("quux"));
        assert_eq!(atoms.verb, Some("ab"));
        assert_eq!(atoms.params, Some(vec!["cd"]));
    }

    #[test]
    // different forms of last param
    fn with_last_param_test() {
        let input = ":src JOIN #chan";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.source, Some("src"));
        assert_eq!(atoms.verb, Some("JOIN"));
        assert_eq!(atoms.params, Some(vec!["#chan"]));
    }

    #[test]
    fn with_last_param_with_colon_test() {
        let input = ":src JOIN :#chan";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.source, Some("src"));
        assert_eq!(atoms.verb, Some("JOIN"));
        assert_eq!(atoms.params, Some(vec![":#chan"]));
    }

    #[test]
    fn with_last_param_test2() {
        let input = ":src AWAY";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.source, Some("src"));
        assert_eq!(atoms.verb, Some("AWAY"));
        assert_eq!(atoms.params, None);
    }

    #[test]
    fn with_last_param_test3() {
        let input = ":src AWAY ";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.source, Some("src"));
        assert_eq!(atoms.verb, Some("AWAY"));
        assert_eq!(atoms.params, Some(vec![""]));
    }

    #[test]
    // tab is not considered <SPACE>
    fn with_tab_test() {
        let input = ":cool\tguy foo bar baz";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.source, Some("cool\tguy"));
        assert_eq!(atoms.verb, Some("foo"));
        assert_eq!(atoms.params, Some(vec!["bar", "baz"]));
    }

    #[test]
    //  with weird control codes in the source
    fn with_control_codes_test() {
        let input = ":coolguy!ag@net\x035w\x03ork.admin PRIVMSG foo :bar baz";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.source, Some("coolguy!ag@net\x035w\x03ork.admin"));
        assert_eq!(atoms.verb, Some("PRIVMSG"));
        assert_eq!(atoms.params, Some(vec!["foo", "bar baz"]));
    }

    #[test]
    fn with_control_codes_test2() {
        let input = ":coolguy!~ag@n\x02et\x0305w\x0fork.admin PRIVMSG foo :bar baz";
        let atoms = msg_split_todo(input);
        assert_eq!(
            atoms.source,
            Some("coolguy!~ag@n\x02et\x0305w\x0fork.admin")
        );
        assert_eq!(atoms.verb, Some("PRIVMSG"));
        assert_eq!(atoms.params, Some(vec!["foo", "bar baz"]));
    }

    #[test]
    fn with_tags_test0() {
        let input = "@tag1=value1;tag2;vendor1/tag3=value2;vendor2/tag4= :irc.example.com COMMAND param1 param2 :param3 param3";
        let atoms = msg_split_todo(input);
        assert_eq!(
            atoms.tags,
            Some(vec![
                ("tag1", "value1"),
                ("tag2", ""),
                ("vendor1/tag3", "value2"),
                ("vendor2/tag4", ""),
            ])
        );
        assert_eq!(atoms.source, Some("irc.example.com"));
        assert_eq!(atoms.verb, Some("COMMAND"));
        assert_eq!(
            atoms.params,
            Some(vec!["param1", "param2", "param3 param3"])
        );
    }

    #[test]
    fn without_tags_test() {
        let input = ":irc.example.com COMMAND param1 param2 :param3 param3";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.tags, None);
        assert_eq!(atoms.source, Some("irc.example.com"));
        assert_eq!(atoms.verb, Some("COMMAND"));
        assert_eq!(
            atoms.params,
            Some(vec!["param1", "param2", "param3 param3"])
        );
    }

    #[test]
    fn with_tags_test2() {
        let input = "@tag1=value1;tag2;vendor1/tag3=value2;vendor2/tag4 COMMAND param1 param2 :param3 param3";
        let atoms = msg_split_todo(input);
        assert_eq!(
            atoms.tags,
            Some(vec![
                ("tag1", "value1"),
                ("tag2", ""),
                ("vendor1/tag3", "value2"),
                ("vendor2/tag4", ""),
            ])
        );
        assert_eq!(atoms.verb, Some("COMMAND"));
        assert_eq!(
            atoms.params,
            Some(vec!["param1", "param2", "param3 param3"])
        );
    }

    #[test]
    fn without_params_test() {
        let input = "COMMAND";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.tags, None);
        assert_eq!(atoms.source, None);
        assert_eq!(atoms.verb, Some("COMMAND"));
        assert_eq!(atoms.params, None);
    }

    #[test]
    // yaml encoding + slashes is fun
    fn slashes_yaml() {
        let input = "@foo=\\\\\\\\\\:\\\\s\\s\\r\\n COMMAND";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.tags, Some(vec![("foo", "\\\\;\\s \r\n"),]));
        assert_eq!(atoms.verb, Some("COMMAND"));
    }

    #[test]
    //  broken messages from unreal
    fn broken_message_test() {
        let input = ":gravel.mozilla.org 432  #momo :Erroneous Nickname: Illegal characters";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.source, Some("gravel.mozilla.org"));
        assert_eq!(atoms.verb, Some("432"));
        assert_eq!(
            atoms.params,
            Some(vec!["#momo", "Erroneous Nickname: Illegal characters"])
        );
    }

    #[test]
    fn message_test2() {
        let input = ":gravel.mozilla.org MODE #tckk +n ";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.source, Some("gravel.mozilla.org"));
        assert_eq!(atoms.verb, Some("MODE"));
        assert_eq!(atoms.params, Some(vec!["#tckk", "+n"]));
    }

    #[test]
    fn message_test3() {
        let input = ":services.esper.net MODE #foo-bar +o foobar  ";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.source, Some("services.esper.net"));
        assert_eq!(atoms.verb, Some("MODE"));
        assert_eq!(atoms.params, Some(vec!["#foo-bar", "+o", "foobar"]));
    }

    #[test]
    //  tag values should be parsed char-at-a-time to prevent wayward replacements.
    fn tag_value_test() {
        let input = "@tag1=value\\\\ntest COMMAND";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.tags, Some(vec![("tag1", "value\\ntest"),]));
        assert_eq!(atoms.verb, Some("COMMAND"));
    }

    #[test]
    // If a tag value has a slash followed by a character which doesn't need
    fn tag_value_slash_test() {
        let input = "@tag1=value\\1 COMMAND";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.tags, Some(vec![("tag1", "value1"),]));
        assert_eq!(atoms.verb, Some("COMMAND"));
    }

    #[test]
    // A slash at the end of a tag value should be dropped
    fn tag_value_slash_end_test() {
        let input = "@tag1=value1\\ COMMAND";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.tags, Some(vec![("tag1", "value1"),]));
        assert_eq!(atoms.verb, Some("COMMAND"));
    }

    #[test]
    //  Duplicate tags: Parsers SHOULD disregard all but the final occurence
    fn duplicate_tags_test() {
        let input = "@tag1=1;tag2=3;tag3=4;tag1=5 COMMAND";
        let atoms = msg_split_todo(input);
        assert_eq!(
            atoms.tags,
            Some(vec![("tag1", "5"), ("tag2", "3"), ("tag3", "4"),])
        );
        assert_eq!(atoms.verb, Some("COMMAND"));
    }

    #[test]
    //  vendored tags can have the same name as a non-vendored tag
    fn vendored_tags_test() {
        let input = "@tag1=1;tag2=3;tag3=4;tag1=5;vendor/tag2=8 COMMAND";
        let atoms = msg_split_todo(input);
        assert_eq!(
            atoms.tags,
            Some(vec![
                ("tag1", "5"),
                ("tag2", "3"),
                ("tag3", "4"),
                ("vendor/tag2", "8"),
            ])
        );
        assert_eq!(atoms.verb, Some("COMMAND"));
    }

    #[test]
    // Some parsers handle /MODE in a special way, make sure they do it right
    fn message_test4() {
        let input = ":SomeOp MODE #channel :+i";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.source, Some("SomeOp"));
        assert_eq!(atoms.verb, Some("MODE"));
        assert_eq!(atoms.params, Some(vec!["#channel", "+i"]));
    }

    #[test]
    fn message_test5() {
        let input = ":SomeOp MODE #channel +oo SomeUser :AnotherUser";
        let atoms = msg_split_todo(input);
        assert_eq!(atoms.source, Some("SomeOp"));
        assert_eq!(atoms.verb, Some("MODE"));
        assert_eq!(
            atoms.params,
            Some(vec!["#channel", "+oo", "SomeUser", "AnotherUser"])
        );
    }
}

#[cfg(test)]

/// splitting userhosts into atoms
mod userhost_split {
    ///  source is the usthost
    ///
    ///  the atoms dict has the keys:
    ///    * nick: nick string
    ///    * user: user string
    ///    * host: host string
    ///  if a key does not exist, assume it is an empty string
    struct UserHostAtoms<'a> {
        nick: Option<&'a str>,
        user: Option<&'a str>,
        host: Option<&'a str>,
    }

    fn userhost_split_todo(_source: &str) -> UserHostAtoms {
        todo!()
    }

    #[test]
    fn simple_userhost_test() {
        let source = "coolguy";
        let atoms = userhost_split_todo(source);
        assert_eq!(atoms.nick, Some("coolguy"));
        assert_eq!(atoms.user, None);
        assert_eq!(atoms.host, None);
    }

    #[test]
    fn simple_userhost_test2() {
        let source = "coolguy!ag@127.0.0.1";
        let atoms = userhost_split_todo(source);
        assert_eq!(atoms.nick, Some("coolguy"));
        assert_eq!(atoms.user, Some("ag"));
        assert_eq!(atoms.host, Some("127.0.0.1"));
    }

    #[test]
    fn simple_userhost_test3() {
        let source = "coolguy!~ag@localhost";
        let atoms = userhost_split_todo(source);
        assert_eq!(atoms.nick, Some("coolguy"));
        assert_eq!(atoms.user, Some("~ag"));
        assert_eq!(atoms.host, Some("localhost"));
    }

    #[test]
    fn without_atoms_test() {
        let source = "coolguy@127.0.0.1";
        let atoms = userhost_split_todo(source);
        assert_eq!(atoms.nick, Some("coolguy"));
        assert_eq!(atoms.user, None);
        assert_eq!(atoms.host, Some("127.0.0.1"));
    }

    #[test]
    fn simple_userhost_test4() {
        let source = "coolguy!ag";
        let atoms = userhost_split_todo(source);
        assert_eq!(atoms.nick, Some("coolguy"));
        assert_eq!(atoms.user, Some("ag"));
        assert_eq!(atoms.host, None);
    }

    #[test]
    // weird control codes, does happen
    fn weird_userhost_test() {
        let source = "coolguy!ag@net\x035w\x03ork.admin";
        let atoms = userhost_split_todo(source);
        assert_eq!(atoms.nick, Some("coolguy"));
        assert_eq!(atoms.user, Some("ag"));
        assert_eq!(atoms.host, Some("net\x035w\x03ork.admin"));
    }

    #[test]
    fn userhost_test() {
        let source = "coolguy!~ag@n\x02et\x0305w\x0fork.admin";
        let atoms = userhost_split_todo(source);
        assert_eq!(atoms.nick, Some("coolguy"));
        assert_eq!(atoms.user, Some("~ag"));
        assert_eq!(atoms.host, Some("n\x02et\x0305w\x0fork.admin"));
    }
}

/// validating hostnames
#[cfg(test)]
mod validating_hostnames {
    //  These tests should be used on the server for your host-validating
    //  code. Clients should accept whatever the server throws their way because
    //  trying to validate hostnames the server gives you isn't sane -- vhosts
    //  commonly contain weird chars, and oper masks can omit periods altogether and
    //  include things like formatting characters.

    fn valid_hostname_todo(_host: &str) -> bool {
        todo!()
    }

    #[test]
    fn valid_hostname_test1() {
        let host = "irc.example.com";
        assert!(valid_hostname_todo(host));
    }

    #[test]
    fn valid_hostname_test2() {
        let host = "i.coolguy.net";
        assert!(valid_hostname_todo(host));
    }

    #[test]
    fn valid_hostname_test3() {
        let host = "irc-srv.net.uk";
        assert!(valid_hostname_todo(host));
    }

    #[test]
    fn valid_hostname_test4() {
        let host = "iRC.CooLguY.NeT";
        assert!(valid_hostname_todo(host));
    }

    #[test]
    // valid hostnames with digits
    fn valid_hostname_test5() {
        let host = "gsf.ds342.co.uk";
        assert!(valid_hostname_todo(host));
    }

    #[test]
    fn valid_hostname_test6() {
        let host = "324.net.uk";
        assert!(valid_hostname_todo(host));
    }

    #[test]
    // valid hostnames with international encoding
    fn valid_hostname_test7() {
        let host = "xn--bcher-kva.ch";
        assert!(valid_hostname_todo(host));
    }

    #[test]
    // this should only validate after being transformed into punycode as above
    fn valid_hostname_test8() {
        let host = "irc.Bücher.ch";
        assert!(!valid_hostname_todo(host));
    }

    #[test]
    // invalid hostnames
    fn valid_hostname_test9() {
        let host = "-lol-.net.uk";
        assert!(!valid_hostname_todo(host));
    }

    #[test]
    fn valid_hostname_test10() {
        let host = "-lol.net.uk";
        assert!(!valid_hostname_todo(host));
    }

    #[test]
    fn valid_hostname_test11() {
        let host = "_irc._sctp.lol.net.uk";
        assert!(!valid_hostname_todo(host));
    }

    #[test]
    // technically valid hostnames but not allowed as IRC hostnames (server names or client hostnames)
    fn valid_hostname_test12() {
        let host = "irc";
        assert!(!valid_hostname_todo(host));
    }

    #[test]
    fn valid_hostname_test13() {
        let host = "com";
        assert!(!valid_hostname_todo(host));
    }

    #[test]
    // empty hostname
    fn valid_hostname_test14() {
        let host = "";
        assert!(!valid_hostname_todo(host));
    }
}
