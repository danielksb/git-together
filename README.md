# git-together

This is a custom git command. It allows you to see which files were committed together with a target file in the past. The result is sorted by the amount of times they appear in the same commit.

Usage
-----

```bash
> git together src/libcollections/slice.rs
125 src/libcollections/vec.rs
125 src/libcollections/str.rs
103 src/libcollections/string.rs
95 src/libcore/slice.rs
62 src/libstd/os.rs
61 src/libcollections/lib.rs
59 src/libcore/iter.rs
56 src/libstd/io/mod.rs
55 src/libcore/fmt/mod.rs
53 src/libstd/lib.rs
```


Installation
-----------

The crate isn't published yet, so you need to compile the program yourself.

```
git clone https://github.com/danielksb/git-together.git
cd git-together
cargo install
```
