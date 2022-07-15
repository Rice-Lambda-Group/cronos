//! A module for defining the incoming messages and other associated parsers of
//! an IRC server.

use std::{collections::HashMap, io::Read, iter::Peekable};

use crate::IrcError;

#[derive(Clone, Debug, PartialEq, Eq)]
/// A parsed incoming message, containing the heterogenous fields as well as the
/// common homogenous fields of a message.
pub struct Message {
    /// The server which was the original source of this message. The source
    /// will always be `None` for messages received directly from a client.
    source: Option<Vec<u8>>,
    /// The specific sub-type of the message and its parameters.
    kind: MessageKind,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
/// The types of incoming IRC messages that can be received by this server.
///
/// String fields in messages are handled as an array of bytes, as the IRC
/// protocol makes no guarantees that character encodings will be correct.
///
/// Since it's assumed that the received messages will come from a stream of
/// bytes, incoming messages own their own buffers, instead of having a view
/// over some other data.
pub enum MessageKind {
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
    /// The message was malformed in a way that is not covered by the IRC error
    /// codes, but it was still incorrect.
    Malformed,
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::Io(err.kind())
    }
}

impl From<&std::io::Error> for ParseError {
    fn from(err: &std::io::Error) -> Self {
        ParseError::Io(err.kind())
    }
}

impl Message {
    /// Read an incoming line of a message from a source, and attempt to create
    /// an `InMessage` from it.
    ///
    /// Will consume all bytes from `source` until it encounters a CRLF
    /// (carriage-return, line-feed) pair of bytes.
    ///
    /// # Errors
    ///
    /// This function will return an error in one of 3 cases:
    /// * The message received is incorrectly formatted, or otherwise cannot be
    ///     parsed.
    ///     In this case, the return value will be of variant
    ///     `Err(ParseError::Irc)`.
    /// * There was an I/O error while trying to read the incoming input.
    ///     In this case, the return value will be of variant
    ///     `Err(ParseError::Io)`.
    /// * The connection was closed, or the end of the reader was reached, while
    ///     parsing the line.
    ///     In this case, the return value will be of variant
    ///     `Err(ParseError::End)`.
    ///
    /// # Panics
    /// fuk u clippy
    pub fn parse_line(source: &mut dyn Read) -> Result<Message, ParseError> {
        let mut bytes = source.bytes().peekable();

        // check for tags
        let tags: HashMap<Vec<u8>, Vec<u8>> = match bytes.peek().ok_or(ParseError::End)? {
            Ok(b'@') => todo!(),
            _ => HashMap::new(),
        };
        // check for source identifier
        let source: Option<Vec<u8>> = match bytes.peek().ok_or(ParseError::End)? {
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

        // lastly, extract the parameters. `params` will have
        let params: Vec<Vec<u8>> = {
            let mut params = Vec::new();
            loop {
                let next_byte = bytes.peek().ok_or(ParseError::End)?.as_ref()?;
                if *next_byte == b'\r' {
                    // end of message
                    bytes.next().unwrap().unwrap();
                    if bytes.next().ok_or(ParseError::End)?? != b'\n' {
                        return Err(ParseError::Malformed);
                    }
                    break;
                }
                params.push(Message::parse_param(&mut bytes)?);
                consume_spaces(&mut bytes)?;
            }
            params
        };

        Ok(Message {
            source,
            kind: match std::convert::AsRef::<[u8]>::as_ref(&verb.to_ascii_uppercase()) {
                b"NICK" => match params.len() {
                    0 => Err(ParseError::Irc(IrcError::NeedMoreParams(verb)))?,
                    1 => MessageKind::Nick(params[0].clone()),
                    _ => Err(ParseError::Irc(IrcError::ErroneousNickname(
                        params[0].clone(),
                    )))?,
                },
                _ => Err(ParseError::Irc(IrcError::UnknownCommand(verb)))?,
            },
        })
    }

    /// Parse a single parameter string. I
    fn parse_param(
        bytes: &mut Peekable<std::io::Bytes<&mut dyn Read>>,
    ) -> Result<Vec<u8>, ParseError> {
        // Characters which cannot be part of the non-trailing parameter.
        const ESCAPES: &[u8] = b" \r\n\x00";
        let mut param = Vec::new();
        match bytes.peek().ok_or(ParseError::End)? {
            Ok(b':') => {
                bytes.next().unwrap().unwrap();
                // read up until we reach the end
                while !b"\r\n\x00".contains(bytes.peek().ok_or(ParseError::End)?.as_ref()?) {
                    param.push(bytes.next().unwrap().unwrap());
                }
            }
            Ok(byte) => {
                if ESCAPES.contains(byte) {
                    return Ok(param);
                }

                while !ESCAPES.contains(bytes.peek().ok_or(ParseError::End)?.as_ref()?) {
                    param.push(bytes.next().unwrap().unwrap());
                }
            }
            Err(e) => return Err(e.into()),
        }

        Ok(param)
    }
}

/// Helper function to consume all spaces in an iterator.
/// Will not consume any bytes which are not the ASCII space byte (b' ').
fn consume_spaces(iter: &mut Peekable<std::io::Bytes<&mut dyn Read>>) -> Result<(), ParseError> {
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
    fn parse_helper(input: &[u8], expected: &Result<Message, ParseError>) {
        let mut cursor = Cursor::new(input);
        assert_eq!(Message::parse_line(&mut cursor), *expected);
    }

    #[test]
    /// Test that a nickname command is parsed correctly.
    fn parse_nick() {
        parse_helper(
            b"NICK johnny5\r\n",
            &Ok(Message {
                source: None,
                kind: MessageKind::Nick("johnny5".bytes().collect()),
            }),
        );
    }

    #[test]
    /// Test that a nickname message is correctly parsed when it contains spaces
    /// and colons.
    fn parse_nick_spaces_colons() {
        parse_helper(
            b"NICK :Reginald P: Floorbuster\r\n",
            &Ok(Message {
                source: None,
                kind: MessageKind::Nick("Reginald P: Floorbuster".bytes().collect()),
            }),
        )
    }
}
