# About 
My first test *rust* project.
Its not ideal but it works.

# Manual installation

To install *jump* clone this repository to somewhere:
``` bash

    # Via https
    git clone https://github.com/Sh1nJiTEITA/jump.git
    # OR 
    # Via ssh
    git clone git@github.com:Sh1nJiTEITA/jump.git

    cd jump
    cargo install --path .

```
After you will get executable and script inside `${HOME}/.cargo/bin/`

For make this util work you need to add
some lines to your *.bashrc* or *.zshrc*:
``` bash
    # Add cargo path to terminal scope
    export PATH="$HOME/.cargo/bin:$PATH"
    
    # Rename script for comfortable use
    # Dot is needed to execute script inside
    # current terminal session
    alias jump=". jmp"
```

# Usage

To use *Jump* util you need to know some opts: 

``` bash
    # You can always get quick help
    jump -h

    Default usage: Jump JUMP_NAME

    Options:
        -h, --help          Show all flags/opts
        -s, --show          Show saved jumps
        -a, --add JUMP_NAME Add jump point

        -r, --remove JUMP_NAME
                            Remove jump point

        -t, --target JUMP_TARGET
                            Specify target path for specific name
```

Some examples:

``` bash
    
    # Add current terminal path to save file
    jump -a some_name
    # or 
    jump --add some_name
    
    # Jump to saved path
    jump some_name 

    # Remove path by name from save path
    jump -r some_name
    # or 
    jump --remove some_name

    # Add target-path to save file
    jump -a some_name -t /some/other/not/current/path
    # or
    jump --add some_name --target /some/other/not/current/path
    
    # Can be (pseudo-real example)
    jump -a mnt_a -t /mnt/dist_a

    # You can get all saved paths via
    jump -s
    # or 
    jump --show
    
    # --show output
    +-----------+---------------------+
    | Jump name | Jump path           |
    +-----------+---------------------+
    | rust      | /home/USER/Code/rust|
    +-----------+---------------------+
    | 1         | /home/USER          |
    +-----------+---------------------+
```

# TODO
1. Add error handling for target adding (now any path can be added -> no check for directory path)
