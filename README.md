# tg

```
$ tg
Usage: tg [options] <tag> <command> [...]

Show information about tagged repositories

Availible tags: @cyborg, @python, @rust

Subcommands:
    add         Add a repository to the tag
    journal     Show commits in order for a given period
    list        Show repository paths
    remove      Remove a repository from the tag
    status      Show repository statuses

Options:
    --config    Path to a configuration file [default: ~/.tg]
    --data      Path to tag database [default: ~/.tg-data]
    --version   Show the version number
    --help      Show this message

$ tg @cyborg add /home/sam/Development/borntyping/example

$ tg @cyborg journal --today
2015-03-02 11:12 8f99b18 (cyborg) Fixed tests
2015-03-02 11:43 5dd0d7d (cyborg-demo) Made some changes
2015-03-02 11:58 e9d8fdc (cyborg) rustup

$ tg @cyborg list
- cyborg                /home/sam/Development/borntyping/cyborg
- cyborg-demo           /home/sam/Development/borntyping/cyborg-demo
- example               /home/sam/Development/borntyping/example
- mutiny                /home/sam/Development/borntyping/mutiny
- rust-psutil           /home/sam/Development/borntyping/rust-psutil
- rust-simple_logger    /home/sam/Development/borntyping/rust-simple_logger

$ tg @cyborg remove /home/sam/Development/borntyping/example

$ tg @cyborg status
✓ cyborg
✓ cyborg-demo
✗ mutiny                travis-ci build is failing
✓ rust-psutil
✗ rust-simple_logger    uncommitted changes
```
