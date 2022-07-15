# Strigoi
An Irc server implmented in Rust


# Goals
- Easy to use
- No nightly features unless needed
      - Able: I'd like to propose no nightly features period 
            As well as `#![deny(unsafe_code)]` and `#![deny(missing_docs)]`
- Make IRC more assessible
- Support TLS and SASL
- Support "Modern IRC"
- Should connect to others servers to form "Hub", "Core Hub", "Leaf", and "Services"
- Support IRCv3
- UTF-8 support


# MVP
- MVP: handle client to server communication
- MVP+1: handle server to server communication + channel management
- MVP+2: implement SASL (which is not part of the original RFCS iirc)

# TODO
- [ ] Connection setup
- [X] Logging on the server side to better help us find and diagnose problems

      - [ ] Define what needs to be `DEBUG`, `INFO`, `TRACE`, `WARN` and `ERROR`s
- [ ] Server-to-Server protocol
- [ ] Client-to-Client protocol
- [ ] Client-to-Server protocol
- [ ] Connection Registration
- [ ] Channels
- [ ] Basic IRC commands
- [ ] Make a parser for Messaging Parsing
- [ ] Support TLS and SASL
- [ ] Support https://defs.ircdocs.horse/
- [ ] Features:  
      - [ ] AWAYLEN  
      - [ ] CASEMAPPING  
      - [ ] CHANLIMIT  
      - [ ] CHANMODES  
      - [ ] CHANNELLEN  
      - [ ] CHANTYPES  
      - [ ] ELIST  
      - [ ] EXCEPTS  
      - [ ] HOSTLEN  
      - [ ] INVEX  
      - [ ] KICKLEN  
      - [ ] MAXLIST  
      - [ ] MAXTARGETS  
      - [ ] MODES  
      - [ ] NETWORK  
      - [ ] NICKLEN  
      - [ ] PREFIX  
      - [ ] SAFELIST  
      - [ ] SILENCE  
      - [ ] STATUSMSG  
      - [ ] TARGMAX  
      - [ ] TOPICLEN  
      - [ ] USERLEN  
