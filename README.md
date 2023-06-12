
# as I stare my enemy down its eyes. A flash of inspiration came to me


“Even the finest sword plunged into salt water will eventually rust.”
― Sun Tzu The Art of War

## about
This is a PCI compliant (see interface spec) checkers engine that is written completely in rust. Of course all regulations and rules of chess are observed

runs in reasonable time at depth 15 however this speed can be drastically improved if I can figure out why the engine is evaluating 125x the positions it needs to.


 
# how to run
run `cargo run --release` in the root directory of the project

**it is highly recomended to build with release mode**

If you have trouble building for your target os try removeing `mimalloc` from the Cargo.toml file and line 24 of main.rs

## Don't have rust or you dont want to build from souce?
Head over to the (https://github.com/Recontaminated/oxidie_checkers/releases/) and grab the binary for your os.


# get it? oxidize? anyways
written in response to a monster's chess engine
I knew about rust and meddled with it before but never officially dug into it.

I'm learning it in 4 days to spite this other person.
[![wakatime](https://wakatime.com/badge/user/902e7fa8-1568-4cdd-9c52-fa04a942d34b/project/1a973f79-6613-4d5e-a118-e4946a61863c.svg)](https://wakatime.com/badge/user/902e7fa8-1568-4cdd-9c52-fa04a942d34b/project/1a973f79-6613-4d5e-a118-e4946a61863c)

# for max
you can test by running the binary then running `go depth 10`. 

Depth works up to 15 in reasonable time 

make moves against engine with `move a1b2`. positions are identical to [this](https://upload.wikimedia.org/wikipedia/commons/thumb/b/b6/SCD_algebraic_notation.svg/1200px-SCD_algebraic_notation.svg.png) image

or if you want you can ask for all moves by running `allmoves` then `movenum {from index} {to index}`

Interface for ai rival showoff coming very soon


# todo
- fix the 125x over evaluation
- move ordering?
- null move pruning



