//! All of the various modes of irc

/// A struct of all user modes as a group of booleans
#[derive(Debug)]
pub struct UserModes {
    ///   i - marks a users as invisible;
    pub i: bool,
    ///   s - marks a user for receipt of server notices;
    pub s: bool,
    ///   w - user receives wallops;
    pub w: bool,
    ///   o - operator flag.
    pub o: bool,
}

/// A struct of all channel modes as a group of booleans
#[derive(Debug)]
// TODO: Rename inner fields to be more idiomatic and not one letter names
// TODO: Pack this into a bitfield, either manually or with a macro or some such thing
pub struct ChannelModes {
    ///    o - give/take channel operator privileges;
    pub o: bool,
    ///    p - private channel flag;
    pub p: bool,
    ///    s - secret channel flag;
    pub s: bool,
    ///    i - invite-only channel flag;
    pub i: bool,
    ///    t - topic settable by channel operator only flag;
    pub t: bool,
    ///    n - no messages to channel from clients on the outside;
    pub n: bool,
    ///    m - moderated channel;
    pub m: bool,
    ///    l - set the user limit to channel;
    pub l: bool,
    ///    b - set a ban mask to keep users out;
    pub b: bool,
    ///    v - give/take the ability to speak on a moderated channel;
    pub v: bool,
    ///    k - set a channel key (password).
    pub k: bool,
}
