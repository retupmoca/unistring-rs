# unistring

A few helper functions for manipulating unicode strings in a terminal/monospace
font environment. This provides a trait on str to get a string's terminal width,
and a couple of basic line-wrapping helpers.

Currently uses the GNU libunistring library (although I suspect it's feasible to
rewrite into pure rust, and I may attempt that at some point)
