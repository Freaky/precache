## precache

Basically a concurrent Rustified version of:

    find <dir> | xargs cat >/dev/null 2>&1

Also included is a Ruby workalike for comparison purposes.

In my tests, Rust is a bit faster, but Ruby uses less memory.  Hmm...
