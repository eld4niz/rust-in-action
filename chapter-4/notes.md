## Ownership

- Ownership is a stretched metaphor. There is no relationship to property rights.
Within Rust, ownership relates to cleaning values when these are no longer
needed. For example, when a function returns, the memory holding its local
variables needs to be freed. Owners cannot prevent other parts of the program
from accessing their values or report data theft to some overarching Rust
authority.

- A value’s lifetime is the period when accessing that value is valid behavior. A function’s
local variables live until the function returns, while global variables might live
for the life of the program.

- To borrow a value means to access it. This terminology is somewhat confusing as
there is no obligation to return the value to its owner. Its meaning is used to
emphasize that while values can have a single owner, it’s possible for many parts
of the program to share access to those values.

Movement within Rust code refers to movement of ownership, rather than the movement of data. Ownership is a term used within the Rust community to refer to the compile-time process that checks that every use of a value is valid and that every value is destroyed cleanly.

### Every value in Rust is owned
