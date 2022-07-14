#![feature(arbitrary_enum_discriminant)]

pub mod incoming;

/// A simple type alias for a result whose error case is an IRC error.
pub type IrcResult<T> = Result<T, IrcError>;

#[repr(u16)]
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
/// The set of errors which can be created (and then sent to the client) as part 
/// of an IRC process.
/// 
/// The tag integer of each error is the reference error number from the IRC 
/// specification.
pub enum IrcError {
    /* TODO fill out the rest of the error numbers here */
    /// The client did not give a nickname when sending a `NICK` message to the 
    /// server.
    NoNicknameGiven = 431,
    /// The client gave a nickname that was illegal, such as one containing 
    /// illegal characters.
    /// The only field of this error is the attempted nickname which was 
    /// erroneous.
    /// 
    /// Note that the standard misspells "erroneous" as "erroneus." 
    /// When possible, we use the correct spelling.
    ErroneousNickname(Vec<u32>) = 432,
    /// The client attempted to name themselves a nickname that was already in 
    /// use. 
    /// The only field of this error is the attempted nickname that was already 
    /// in use.
    NicknameInUse(Vec<u32>) = 433,
    /// When merging with another server, two different clients collided because 
    /// they have the same nickname. 
    /// The correct solution to this problem is to kick both users immediately.
    NicknameCollision(Vec<u32>) = 436,
}