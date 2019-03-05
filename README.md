# Press v2 (press-rs)

Press will be a mondern disk, image, and os deployment application written in Rust. 

Press uses a declarative configuration format for describing tables, partitions, and volumes.

Press can work with physcial disks or block device images of various formats. 

Press supports operating system deployment for many Linux distributions, Microsoft Windows, and others.

Press is under heavy development and is not currently functional. Hopefully that will be changing soon. 

Presnetly, the code includes structures for dealing with MBR and GPT partition tables and runs on Linux. My hope is to have multi-platform support (OSX, Win32, BSD) in the future.

This version of Press is a rewrite. 

There is currently a working implemention, written in python, which can be found here:

https://github.com/jr0d/press

-- Happy hacking