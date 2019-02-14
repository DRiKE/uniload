# uniload: expressing load with unicode! 

uniload displays the load per CPU core of your system. It reads `/proc/stat`,
calculates the share of time each core was busy doing something (including
iowait), and prints a unicode block character for each core.

(**NB:** the font on GitHub seems to be suboptimal. Using the Terminus font, the bottoms of the blocks are perfectly aligned.)

```bash
$ uniload
▃▄▅▃%

```

It takes only one optional parameter, the delay between reading `/proc/stat`,
which by default is 1000ms. The minimum is 100ms, passing anything less than
that will result in a 100ms delay:

```bash
# using 500ms delay
$ uniload 500
█▁▁▁% 

```

# How to build uniload

It's Rust, so get Rust via either https://rustup.rs/ or your distribution's package manager.
Then, use `cargo build --release`, and find the uniload binary in `./target/release/`.
Copy the binary to somewhere in your `$PATH`.


# Why and where should I use this?

The simple output allows you to use it in text-based status bars.
I'm using this in my i3blocks and tmux status bars.

## i3blocks

Add to your i3blocks.conf:
```
[uniload]
interval=5

```

And place/symlink `uniload` in your `i3blocks` directory.


## tmux

This highly depends on your own `.tmux.conf` of course, but as an example from mine:
```
set-option -g status-right '#(uniload) #(uptime | sed "s/.*average: //" | sed s/,//g) | #[fg=bold]%H:%M:%S #[default]'
```
