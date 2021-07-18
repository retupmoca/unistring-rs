# unistring

A few helper functions for manipulating unicode strings in a terminal/monospace
font environment.

Currently uses the GNU libunistring library and provides a trait on &str to get
a string's terminal width, and a couple of basic line-wrapping helpers.
