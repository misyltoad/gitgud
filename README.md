# GitGud

A little wrapper for git that simplifies some otherwise relatively complex operations such as:

* get
    * (Cloning or updating), resetting and cleaning up an external git repo into/in the current folder (working directory) and if specified to a specific branch.
* clean
    * Cleaning up a git repo.
* update
    * Updating, resetting and cleaning up the current repo and if specified switches to a specific branch. 

Useful for git newbies or if you want to make your build scripts a little simpler.

Primarily recommended for build scripts as it avoids auto merges.

It should also be fairly portable as it is written in Rust.

*This application requires git to be installed and in the PATH.*