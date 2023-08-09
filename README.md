# Project Title

A tool for reproducible behaviour testing in a black-box kinda way,
inspired by [fasterthanlime][1]'s project [rue][2] and the [video][3]
he made about it.

## Description

I'm kinda just getting started here, but this is currently a way of producing
syscall logs that are consistent across application executions (in linux),
as a base for a larger project about testing software behavioural consistency
over time.

The longer aim is to be able to 'mock' the world around an application; the
network interfaces, storage, kernel, et cetera, and log the interactions with
them in a way that is reproducible.

### y tho?

In software with external dependencies (that is to say, almost all modern
software) we rely heavily on trust and assumptions about how the library code,
language host, and environment interact with each other. As supply chain
attacks continue to grow in frequency, verifying that the software does what
you believe it does, and only what you believe it does, becomes increasingly
important. Yesterday's safety is not today's safety; but yesterday's
behaviour can be compared with today's behaviour.

This is a conceptual compliment to repeatability tooling; if you want a
similarly idealistic take on that, check out [Timeless][4]

## Getting Started

This is a pretty standard issue rust project, and I have not released it
formally yet. Clone the repo in your favorite way and 'cargo run'/'cargo build'
should do what you expect. A more in depth guide will appear when it is closer
to 'production ready'

### Dependencies

For a variety of reasons, this is Linux only. Other than that, all dependencies
should be correctly managed by Cargo; if this assumption does not hold, please
open an issue.

### Usage

`testchamber --help` should produce a little help text; more on usage and
usecases later

#### What's that stuff in testapps

I needed a few simple, mostly deterministic programs to verify that these
ideas were holding water before moving forward. Printgist was a snippet
of forgotten provenance, hello-world is what you expect it to be.

## Help

### My output is nondeterministic!
Then your program is probably also nondeterministic. I am working on tooling
around this; some ideas include:

- Another memory table implementation that further abstracts away ordering
  from logging
- Syscall filters to ignore calls that are not particularly informative but
  likely to produce variance, like `flock`
- Entrypoint control, so we only log the part of your program that you expect
  to be deterministic

## Next Steps

This is a first attempt at "idiomatic" rust for me, and I am still futzing
with the specifics. Once all the interfaces are extracted, adding tests
is next, and then tackling a few of the above features. Then I'll finally
declare a version 0.1 and produce a crate and a release process.

## License

This current state is licensed MIT, as it is highly derivative of an MIT
licensed project. Future versions may shift towards something more GPL-like

## Acknowledgments

I mentioned [rue][1] and [timeless][4] above; I also used a pretty cool
[readme template][5] I found on github. Shouts out to the Linux community
as well; this would be much more difficult on other OSes.

[1] https://fasterthanli.me
[2] https://github.com/fasterthanlime/rue/blob/main/src/main.rs
[3] https://www.youtube.com/watch?v=engduNoI6DE
[4] https://github.com/polydawn/timeless
[5] https://gist.githubusercontent.com/DomPizzie/7a5ff55ffa9081f2de27c315f5018afc/raw/d59043abbb123089ad6602aba571121b71d91d7f/README-Template.md
