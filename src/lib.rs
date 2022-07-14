#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]

pub mod incoming;

/// A simple type alias for a result whose error case is an IRC error.
pub type IrcResult<T> = Result<T, IrcError>;

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
/// The set of errors which can be created (and then sent to the client) as part
/// of an IRC process.
///
/// The tag integer of each error is the reference error number from the IRC
/// specification.
pub enum IrcError {
    /* TODO fill out the rest of the error numbers here */
    /// A message was requested for a command which this server does not know
    /// how to process.
    ///
    /// This error is sometimes referred to by its number, 421.
    UnknownCommand(Vec<u8>),
    /// The client did not give a nickname when sending a `NICK` message to the
    /// server.
    ///
    /// This error is sometimes referred to by its number, 431.
    NoNicknameGiven,
    /// The client gave a nickname that was illegal, such as one containing
    /// illegal characters.
    /// The only field of this error is the attempted nickname which was
    /// erroneous.
    ///
    /// Note that the standard misspells "erroneous" as "erroneus."
    /// When possible, we use the correct spelling.
    /// This error is sometimes referred to by its number, 432.
    ErroneousNickname(Vec<u8>),
    /// The client attempted to name themselves a nickname that was already in
    /// use.
    /// The only field of this error is the attempted nickname that was already
    /// in use.
    ///
    /// This error is sometimes referred to by its number, 433.
    NicknameInUse(Vec<u8>),
    /// When merging with another server, two different clients collided because
    /// they have the same nickname.
    /// The correct solution to this problem is to kick both users immediately.
    ///
    /// This error is sometimes referred to by its number, 436.
    NicknameCollision(Vec<u8>),
    /// A message was sent to this server, but too few parameters were supplied
    /// for the message to be properly parsed.
    /// The only field of this variant is the command which did not receive
    /// enough parameters.
    ///
    /// This error is sometimes referred to by its number, 461.
    NeedMoreParams(Vec<u8>),
}
