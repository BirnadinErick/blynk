# blynk 😉

A dead simple commandline utility to remind the user to blink within a given interval of time.

[![Continuous integration for master branch](https://github.com/BirnadinErick/blynk/actions/workflows/ci-master.yml/badge.svg)](https://github.com/BirnadinErick/blynk/actions/workflows/ci-master.yml)

> Skip to the [Installation](index.md#installation) for quickstart. Continue,
> if you are interested in the origin of `blynk`.

---

## The Origin

### Why?

Being a freelancer, I work a lot, actually more than I hafta. 
Which means excess amount of screen-time. Then, the usual stuffs like reddit scrooling, 
Netflix binging, Telegram chats etc. While these are fun and all, they come at a cost, 
actually many cost to the health.

Screen-time is a concerning *health-deterioting-factor* in my opinion. 

>The more you stare, the more you ***wear*** 😉.

for me, I started noticing eye problems like dried-up , 
red-gushing and sometimes hard to focus
nearby objects. From few lookups, I found not blinking the eye for 
a long period might cause these.
Not might anymore, it is. Long screen-time also means, you lack to blink. 

Although `blink frequently` is a well-heard fact, I had to 
realize the **hard** way. So I set Window's *Clock* App and 
ran timer for 25 minutes with label ...

>"Blink 😉 ryt fuqing NOW!"

This was a solution, but being a *nict-picker*, I couldn't fight the fact, 
it is just a timer, but
devours almost 30 MB of memory and 5% of CPU time
(of 2.7GHz; that's almost 13x10<sup>6</sup> clock-cycles)
and now-and-then network ping(I don't like this one a bit, 🙅‍♂️) 
by the app itself, occasionally.

So, I started this side-project, THE `blynk`. 

### What?

`blynk` is a self-contained binary executable. It is a commandline utility with
a very small footprint(atleast it tries to 😉). The goal of this project when I
started is a very much dead-simple & just-works reminder that just reminds me 
to blink.

`blynk` toats a notification to the user in between given interval with a 
given message. Iterates this steps till it is terminated/killed!

### How?

`blynk` achieves its functionality via [rust](https://www.rust-lang.org/) 
and a cargo [notify-rust](https://docs.rs/crate/notify-rust/latest).

---

## Installation

`blynk` is a self-contained binary, thus no installation required 🎉!

Refer to 

1. download the latest release from the [RELEASE](https://www.github.com/BirnadinErick/blynk) page
2. extract the downloaded artifact
3. place `blynk` binary in any desired folder

> place `blynk` binary in the **PATH** directories for convienience run

### Notes on the Update-procedure

As of July 2022, `blynk` can be updated to latest version by iterating above steps again and 
replacing the binary.

But, future releases may notify for new ***major*** releases and might even have
option to do the procedure itself.

### Notes in supported version

`blynk` of [latest](https://www.github.com/BirnadinErick/blynk) is always
supported and any previous versions will be deprecated at 
the time of release!

---

## Misc

### Contribution and Involvement

Any kind of contributions are welcome! 
Please refer [Contributing Guide](/CONTRIBUTING.md) and 
the [Code of Conduct](/CODE_OF_CONDUCT.md).

### License

`blynk` is lincensed under [**Apache License version 2.0.0**](/LICENSE.md)

### Adiue 🙋‍♂️

Well, that's it for now, till then this is me the BE, signing off 😘.
👨‍💻 in Jaffna, Sri Lanka with ❤.
