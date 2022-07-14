//! A module for defining the incoming messages and other associated parsers of 
//! an IRC server.

use std::{io::Read, collections::HashMap, iter::Peekable};

use crate::IrcError;

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
/// The incoming IRC messages that can be received by this server. 
/// 
/// String fields in messages are handled as an array of bytes, as the IRC 
/// protocol makes no guarantees that character encodings will be correct. 
/// 
/// Since it's assumed that the received messages will come from a stream of 
/// bytes, incoming messages own their own buffers, instead of having a view 
/// over some other data.
pub enum InMessage {
    /// Notify the server of a user's nickname. The only field is the bytes of 
    /// the user's nickname.
    Nick(Vec<u8>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    /// We were unable to parse this message because the message was incorrect, 
    /// and the client should accordingly be notified.
    Irc(IrcError),
    /// We were unable to parse this message because of an input/output error. 
    /// The error kind is given.
    Io(std::io::ErrorKind),
    /// The input stream suddenly ended (such as by sending an EOF).
    End,
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::Io(err.kind())
    }
}

impl InMessage {
    /// Read an incoming line of a message from a source, and attempt to create 
    /// an `InMessage` from it. 
    /// 
    /// Will consume all bytes from `source` until it encounters a CRLF 
    /// (carriage-return, line-feed) pair of bytes.
    pub fn parse_line(source: &mut dyn Read) -> Result<InMessage, ParseError> {
        let mut bytes = source.bytes().peekable();

        // check for tags
        let tags: HashMap<Vec<u32>, Vec<u32>> = match bytes.peek().ok_or(ParseError::End)? {
            Ok(b'@') => todo!(),
            _ => HashMap::new(),
        };
        // check for source identifier
        let source: Option<Vec<u32>> = match bytes.peek().ok_or(ParseError::End)? {
            Ok(b':') => todo!(),
            _ => None,
        };

        // Extract a command.
        // here, we assume there are no leading spaces.
        let verb = {
            let mut buf = Vec::new();
            let mut byte = bytes.next().ok_or(ParseError::End)??;
            while byte.is_ascii_alphanumeric() {
                buf.push(byte);
                byte = bytes.next().ok_or(ParseError::End)??;
            }
            consume_spaces(&mut bytes)?;
            buf
        };

        // verb is now a sequence of alphanumeric bytes.

        // lastly, extract the parameters
        let params: Vec<Vec<u8>> = {
            let mut params = Vec::new();
            // TODO actually finish this
            params
        };

        // now that we've extracted all the strings, put it together in a more 
        // readable message
        todo!();
    }
}

/// Helper function to consume all spaces in an iterator. 
/// Will not consume any bytes which are not the ASCII space byte (b' ').
fn consume_spaces(
    iter: &mut Peekable<std::io::Bytes<&mut dyn Read>>
) -> Result<(), ParseError> {
    while let Ok(b' ') = iter.peek().ok_or(ParseError::End)? {
        iter.next().unwrap()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use std::io::Cursor;

    use super::*;

    /// Helper function for testing the parser. `input` is 
    fn parse_helper(input: &[u8], expected: Result<InMessage, ParseError>) {
        let mut cursor = Cursor::new(input);
        assert_eq!(InMessage::parse_line(&mut cursor), expected);
    }

    #[test]
    /// Test that a nickname command is parsed correctly.
    fn parse_nick() {
        parse_helper(
            b"NICK johnny5\r\n", 
            Ok(InMessage::Nick("johnny5".bytes().collect()))
        );
    }
}