# Security Policy

## What this repository is, before anything else

`learn-rust` teaches Rust from the first line. Under `units/` sit 73 crates, one
per lesson, whose exercise bodies are `todo!()` and whose tests stay red until
somebody solves them. Under `solutions/` sits a crate of the same name for every
one of them, with the exercises worked out. `xtask/` holds the check run that
reads the tree. Every text stands in German and then in English.

Three measurements shape everything below.

Nothing here is deployed or published. Every manifest carries `publish = false`.
There is no package on a registry, no service, no endpoint, no account, and no
stored data belonging to anybody.

Nothing here depends on anything outside the standard library. `Cargo.lock` at
the root and `units/Cargo.lock` list only the local packages of this repository
and not one crate from outside it. Together with the toolchain pinned in
`rust-toolchain.toml`, that is the whole supply chain.

The code is meant to be copied. That is the point of a teaching repository, and
it is also the one place where a fault here reaches past this repository.

## Where to report

Use GitHub's private vulnerability reporting:

<https://github.com/iderex/learn-rust/security/advisories/new>

That door opens today. Measured rather than assumed:

```console
$ gh api repos/iderex/learn-rust/private-vulnerability-reporting
{"enabled":true}
```

The report goes to me and stays out of public view while it is open. If you
would rather not use the form, open a normal issue saying that you have
something to report and nothing about what it is, and I will open the private
thread from my side.

Useful in a report: the path of the file, the toolchain you used, and the
shortest thing you can hand me that shows the problem. A failing test, a Miri
run, or eight lines of `main` are worth more than a description.

## What I do not promise

I do not promise a time to first answer. This is one person's teaching
repository, and a deadline I cannot keep would be worse than none at all: a
reporter who was told to expect a reply by some point and does not get one
spends the wait wondering whether the report arrived at all. Knowing from the
start that the answer comes when it comes is the more honest position, and it is
the one I can actually hold. The advisory thread stays where you filed it, and
it is the first place I look.

## What would actually be a vulnerability here

Four kinds, in the order I care about them.

**Unsound code that teaches.** Stage 10 is about `unsafe`, raw pointers,
undefined behaviour and FFI. `units/10-06-ffi-mit-extern-c` declares `abs` and
`strlen` behind `extern "C"` and its solution calls them; `10-01` and `10-02`
work with what `unsafe` allows and with raw pointers. Where an `unsafe` block
names the condition it relies on and that condition does not in fact hold, or
where a solution under `solutions/` reaches undefined behaviour while its own
text says it does not, that is the report I most want. A reader who learns a
wrong pattern here writes it again somewhere it matters.

**Something in the tree that misbehaves on a reader's machine.** Following
CONTRIBUTING.md means cloning this repository and running the check run over
both workspaces, which compiles and runs code from here. `06-08-build-rs` and
its solution carry a build script that runs before any test and writes into
`OUT_DIR`. `cargo test --workspace` runs the solution tests, and two of those,
for `09-08` and `09-09`, bind a real TCP listener on `127.0.0.1:0`. All of that
is described in the files themselves. If any of it does something its own text
does not say, writes outside its package's `OUT_DIR`, binds an address other
than loopback, reaches the network, or opens a file it was never handed, that is
a defect and I want to hear about it.

**A way past what CI is allowed to do.** `.github/workflows/prueflauf.yml` runs
on every pull request and sends `cargo run -p xtask -- ci`, which reads the
command block out of CONTRIBUTING.md and starts each line with `Command::new`,
with no shell in between. A pull request that edits that block therefore changes
what CI executes. By itself that is not a finding: the workflow grants only
`permissions: contents: read`, the repository's default workflow token is read
only and cannot approve pull requests, the repository holds no Actions secrets,
and CI already compiles and runs the pull request's own Rust code, which through
a build script can do whatever that code wants. A finding is a route to
something the workflow is not supposed to hold: write access, a token, another
branch's cache. `xtask/src/befehle.rs` refuses any command line carrying a
quote, a backtick, a dollar sign, an ampersand, a semicolon, a pipe, an angle
bracket, a parenthesis, a brace or a backslash, so that a line cannot read as
one thing and execute as another. A line that gets past that guard and still
manages it is a finding as well.

**The workflows themselves.** There are two. `codeql.yml` pins its actions by
commit SHA and checks out with `persist-credentials: false`. `prueflauf.yml`
uses `actions/checkout@v7`, a tag that can be moved under it. I know about that
inconsistency and have not fixed it yet; a report that shows it or anything else
in those two files being reachable is welcome.

## What is not a vulnerability here

The exercises fail. Their bodies are `todo!()` and their tests are red until
somebody writes the missing code. That red is the feedback this repository is
built on, not a broken build.

The solutions are public and complete. Reading one before doing the exercise
costs the reader the exercise, which README.md says plainly. It is a description
of what happens and not a control I am trying to enforce.

The web server in `09-08` and `09-09` has no TLS, no authentication, no header
parsing, no size limit and no timeout. It answers two paths and a 404, and the
only test that goes over the network binds `127.0.0.1:0`. It is the small
blocking server from the end of the book, deliberately small. A list of what a
production server would have says nothing about this program. Showing it binding
something other than loopback, or reaching a file it was never handed, is a
different report, and I do want that one.

There is `unsafe` code, and there is code that is undefined on purpose.
`10-03-undefiniertes-verhalten` carries an enum of eight steps, four of them
undefined behaviour, and asks the reader to look each one up in the Reference.
`10-01` is called "what unsafe allows". Reporting that this repository contains
`unsafe` describes stage 10 rather than a defect. Reporting that one specific
block is unsound is the first item above.

Scanner output with no argument attached to it. CodeQL runs over this tree on
every push, every pull request and once a week with the `security-and-quality`
pack, so I already see what it says. A pasted alert is not a report. What
somebody reaches through it would be.

Anything about a deployed instance, a published crate, or user data. None of the
three exists. If somebody has taken this code, deployed it, and come to harm,
the thing to report here is the fault in the code and not the deployment.

Wrong translations, broken links, stale chapter numbers in citations, or a unit
that explains something incorrectly with no safety consequence. The tracker
wants all of those. Filing them privately only delays the fix.

## After a report

I read it, decide whether it is a fault of this repository, and tell you which.
If it is, the fix lands on `main` in the open. There is nothing here to protect
by staging a quiet release: no published package, no deployment, and nobody
running a version I could put at risk by fixing it in public. If you want to be
named in the advisory, say so and say how. If you say nothing, I will not name
you.
