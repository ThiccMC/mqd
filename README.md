# `mqd`

A database that suppose to query minecraft server using `gamedig` crate.

## Protocol

Connect using standard `TCP` protocol. All message are encoded in UTF-8 sequence. All numbers are decimal

- `query`
  > ```
  >   The message with have a repeated sequence of
  >   v Delimiter
  >   | v Error code (u8)
  >   | | v Comma seperator
  >   | | |v Current player (u32)
  >   | | || v Max player
  >   | | || |
  >   :255,1,20
  > ```
  >
  > No more than 32 entries.
- `cron`
  > Either `ok` or `nope`. You guessed it.
  > It will try 3 times, then put in database as not 0, with no player and maximum of 0. "Quá tam ba bận" =\)\)
- `freeze`
  > cron basedn't database save
- `yeet`
  > yeet the server? it might only yeet you instead
