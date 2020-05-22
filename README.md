PGen
====

This is a very opinionated password generator. It takes only one option (length), and generates 5 passwords:

	phil@fry ~> pgen
	dyrumiwusy	zbamsyztet	qMBkkmjRPb	dMazV2mKWm	h1c3AceR9.

The first is (mostly, sometimes) pronounceable, the second is all lower case, the third is upper and lower case, the fourth includes digits, and the last adds a sigle special character. The idea is to pick the right password based on the specific security/convenience tradeoffs where it's being used, or the silly and misguided requirements of the application.

FAQ
===

*Why isn't this just 20 lines of Python?*
I'm really trying to get familiar with Rust, so I'm forcing myself to use it.

*Why can't I configure the passwords I see and how they are built?*
Yeah, that's a good idea. That's probably the next step.

*Can you really have FAQs if no one has ever asked anything?*
Apparently.
