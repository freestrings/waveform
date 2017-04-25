waveform
===

Generating waveform images from mp3 in Rust. MIT Licensed.

Why
===

To enjoy rust! -:)

Install Prerequisites
===

You must install on your computer the `rust`.

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

Install waveform
===

```bash
$ git clone git@github.com:freestrings/waveform.git
$ cd waveform
$ cargo build --release
$ echo "export PATH=$PWD/target/release:\$PATH" > .waveform
$ source .waveform
$ waveform --help
waveform 0.1
Changseok Han <freestrings@gmail.com>

USAGE:
    waveform [FLAGS] [OPTIONS] <INPUT>...

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Sets the level of verbosity

OPTIONS:
    -b, --background <BACKGROUND>    The background hex color. the default value is #000000
    -f, --foreground <FOREGROUND>    The foreground hex color. the default value is #ffffff
    -h, --height <HEIGTH>            The image height. the default height is 120 pixel
    -o, --output <OUTPUT>            The output directory
    -w, --width <WIDTH>              The image width. the default width is 512 pixel

ARGS:
    <INPUT>...    mp3 file pathes. ex) ./waveform file1 file2
```

Examples
===

```bash
$ waveform ~/Music/Dio/Holy\ \Diver/02.\ \Holy\ \Diver.mp3
```
![waveform](./resources/02%20Holy%20Diver.mp3.png)

Apply a background or foreground color to enjoy.

```bash
$ waveform ~/Music/Dio/Holy\ \Diver/02.\ \Holy\ \Diver.mp3 --foreground "#c40b30"
```

![waveform_fg](./resources/02%20Holy%20Diver.mp3.fg.png)

Change a image size.

```bash
$ waveform ~/Music/Dio/Holy\ \Diver/02.\ \Holy\ \Diver.mp3 \
    --background "#8A1944" --foreground "#3CD746" \
    --width 800 --height 50 \
    --output ~/Downloads/out
```

![waveform_scale](./resources/02%20Holy%20Diver.mp3.scale.png)

Batch transform
===

```bash
$ find . -name "*.mp3" -printf "\"%p\"\n" | xargs waveform -o ~/Downloads/out -f "#c40b30" -v
Done 1/9 "/home/han/Music/Dio/Holy Diver/01. Stand Up and Shout.mp3"
Done 2/9 "/home/han/Music/Dio/Holy Diver/02. Holy Diver.mp3"
Done 3/9 "/home/han/Music/Dio/Holy Diver/03. Gypsy.mp3"
Done 4/9 "/home/han/Music/Dio/Holy Diver/04. Caught in the Middle.mp3"
Done 5/9 "/home/han/Music/Dio/Holy Diver/05. Don’t Talk to Strangers.mp3"
Done 6/9 "/home/han/Music/Dio/Holy Diver/06. Straight Through the Heart.mp3"
Done 7/9 "/home/han/Music/Dio/Holy Diver/07. Invisible.mp3"
Done 8/9 "/home/han/Music/Dio/Holy Diver/08. Rainbow in the Dark.mp3"
Done 9/9 "/home/han/Music/Dio/Holy Diver/09. Shame on the Night.mp3"
```