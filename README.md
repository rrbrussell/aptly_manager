# aptly_manager
My tool for automating the aptly tasks required to maintain my local debian and ubuntu mirrors.

## To use for yourself.
In libs/aptly_lib/src is the source code for the aptly_lib crate which manages the actual
data storage format.

As of version 0.1.0 all of the mirror information needs to be hard coded.

I need to implement the parser side of a TOML parser to read a control file.
There are TOML lexers available but converting the TOML AST tree they make into understandable
datastructures requires a full parser.