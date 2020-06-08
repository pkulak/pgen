PGen
====

This is a very opinionated password generator. It can generate five types of password, in increasing complexity, and otherwise only takes length as an option.

	phil@fry ~> pgen
	dyrumiwusy	zbamsyztet	qMBkkmjRPb	dMazV2mKWm	h1c3AceR9.

The first is (mostly, sometimes) pronounceable, the second is all lower case, the third is upper and lower case, the fourth includes digits, and the last adds a sigle special character. The idea is to pick the right password based on the specific security/convenience tradeoffs of where it's being used, or the silly and misguided requirements of the application.

Installation
============

PGen is available for Arch in the AUR as [pgen](https://aur.archlinux.org/packages/pgen/). Or you can build it yourself with

	cargo build --release

Options
=======

-**l** or --**length**: How long should the password be? Defaults to 10.

-**c** or --**complexity**: Print out a single password only. Values are from 1 to 5, inclusive. Defaults to not set (or 0).

FAQ
===

**Why isn't this just 20 lines of Python?**  
I'm really trying to get familiar with Rust, so I'm forcing myself to use it.

**Why can't I configure the passwords I see and how they are built?**  
Yeah, that's a good idea. That's probably the next step.

**Some of this code is really not how you're supposed to write Rust...**  
Please let me know! This is my first project, and I'd love to know what mistakes I'm making. PRs welcome, of course, or just yell at me.

**Can you really have FAQs if no one has ever asked anything?**  
Apparently.
