# Purpose

To experiment with using a build.rs script to embed the current commit into the binary.

# Info

Got it working! This is so cool...

I've long thought it'd be _super_ useful to be able to see the actual commit used for building the binary. With my python code, I can just `cd` into the directory running the code and do a `git log` to see the current working commit. But with a binary?

Sure, many build setups allow you to manually enter the new version. But it's possible to forget to do so, or to fail to do so reliably. And of course, specifying the current commit in the code, and committing that code, automatically makes it behind-by-one.

Thanks to ChatGPT-4 for walking me through the solution!

# Usage

For development: 

```
% cargo run -- --version 
Version `15188e6c54cd0e790d841efca18ea0467367e9f8`
```

(well, that _was_ accurate at the time 🙂)

That `cargo run -- --version` is the equivalent of creating the binary (say, "my_app"), and then running `./my_app --version`
