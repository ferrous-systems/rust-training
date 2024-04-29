# Tradeoffs

## OK, but what's the catch?

## You can't write C in Rust

* You have to think about memory up-front
  * Who owns any given value?
  * Who needs to borrow it and when?
  * Does it live long enough to satisfy those borrows?
  * Are you borrowing something that might move?

## Rust exposes underlying complexity

* There are at least six kinds of "String" in Rust
  * Owned or Borrowed, Rust-native, C-compatible and OS API-compatible
* There is no garbage collector - you manage your own memory
  * Maybe you'd be OK with the performance of Go, or C# or Java?

## Rust doesn't interact well with C++ code

* Rust doesn't understand classes or templates
* Neither Rust nor C++ have a stable ABI
* Projects do exist to auto-generate bindings, like [cxx](https://crates.io/crates/cxx)

## Touching the hardware requires `unsafe`

Hardware is a blob of shared mutable state and you have to manually verify your
access to it is correct

## What you have works just fine

If it's safe enough, maintainable enough and fast enough, then you should keep it!

Definitely don't do too many new things at once.

## It's early days for building critical-systems in Rust

Ferrocene is good, but C and Ada have a multi-decade head start

## Is the juice worth the squeeze?

## Only you can decide!

But we can show you what other people have found...

## Some quotes...

<div class="columns">
<div>

* Microsoft
* Mozilla
* Google
* Cloudflare
* Amazon

</div>
<div>

* Linux Kernel developers
* Dropbox
* Meta
* Infineon

</div>
</div>

## Microsoft

> Tools and guidance are demonstrably not preventing this class of
> vulnerabilities; memory safety issues have represented almost the same
> proportion of vulnerabilities assigned a CVE for over a decade. We feel that
> using memory-safe languages will mitigate this in ways that tools and training
> have not been able to.

-- [MSRC Blog, July 2019](https://msrc.microsoft.com/blog/2019/07/we-need-a-safer-systems-programming-language)

---

> We believe Rust changes the game when it comes to writing safe systems
> software. Rust provides the performance and control needed to write low-level
> systems, while empowering software developers to write robust, secure
> programs.

-- [MSRC Blog, July 2019](https://msrc.microsoft.com/blog/2019/07/why-rust-for-safe-systems-programming)

---

> Speaking of languages, it's time to halt starting any new projects in C/C++
> and use Rust for those scenarios where a non-GC language is required. For the
> sake of security and reliability, the industry should declare those languages
> as deprecated.

-- [CTO of Azure, September 2022](https://twitter.com/markrussinovich/status/1571995117233504257)

## Mozilla

> With the release of Firefox 48, we shipped the very first browser component to
> be written in the Rust programming language — an MP4 parser for video files.
> Streaming media files in your browser can be particularly risky if you don’t
> know or trust the source of the file, as these can maliciously take advantage
> of bugs in a browser’s code. Rust’s memory-safe capabilities prevent these
> vulnerabilities from being built into the code in the first place.

-- [Firefox Blog, March 2017](https://blog.mozilla.org/en/products/firefox/put-trust-rust-shipping-now-firefox)

## Google

> As the amount of new memory-unsafe code entering Android has decreased, so too
> has the number of memory safety vulnerabilities. From 2019 to 2022 it has
> dropped from 76% down to 35% of Android’s total vulnerabilities. 2022 is the
> first year where memory safety vulnerabilities do not represent a majority of
> Android’s vulnerabilities.

-- [Google Security Blog, December 2022](https://security.googleblog.com/2022/12/memory-safe-languages-in-android-13.html)

---

> More than 2/3 of respondents are confident in contributing to a Rust codebase
> within two months or less when learning Rust.
>
> Anecdotally, these ramp-up numbers are in line with the time we’ve seen for
> developers to adopt other languages, both inside and outside of Google.

-- [Google Open Source Blog, June 2023](https://opensource.googleblog.com/2023/06/rust-fact-vs-fiction-5-insights-from-googles-rust-journey-2022.html)

---

> Rust teams at Google are as productive as ones using Go, and more than twice
as productive as teams using C++.

and

> In every case, we've seen a decrease by more than 2x in the amount of effort
> required to both build the services written in Rust, as well as maintain and
> update those services. [...] C++ is very expensive for us to maintain.

-- [Lars Bergstrom at Rust Nation 2024](https://www.youtube.com/watch?v=QrrH2lcl9ew)

---

> Our goal in bringing Rust into Chromium is to provide a simpler and safer way
> to satisfy the rule of two, in order to speed up development and improve the
> security of Chrome. And we believe that we can use third-party Rust libraries
> to work toward this goal.

-- [Google Security Blog, January 2023](https://security.googleblog.com/2023/01/supporting-use-of-rust-in-chromium.html)

Note:

The *rule of two* is, pick up to two of:

* Untrusted Input
* No Sandbox
* Unsafe Language

## Cloudflare

> In production, Pingora consumes about 70% less CPU and 67% less memory
> compared to our old service with the same traffic load.

-- [Cloudflare Blog, September 2022](https://blog.cloudflare.com/how-we-built-pingora-the-proxy-that-connects-cloudflare-to-the-internet)

## Amazon

> Here at AWS, we love Rust, too, because it helps AWS write highly performant,
> safe infrastructure-level networking and other systems software. ... we also
> use Rust to deliver services such as S3, EC2, CloudFront, Route 53, and more
> ... Our Amazon EC2 team uses Rust as the language of choice for new AWS Nitro
> System components...

-- [AWS Open Source Blog](https://aws.amazon.com/blogs/opensource/why-aws-loves-rust-and-how-wed-like-to-help)

## Linux Kernel

> Like we mentioned last time, the Rust support is still to be considered
> experimental. However, support is good enough that kernel developers can start
> working on the Rust abstractions for subsystems and write drivers and other
> modules.

-- [Linux Kernel Mailing List, January 2022](https://lore.kernel.org/lkml/20220117053349.6804-1-ojeda@kernel.org)

## Dropbox

> We wrote Nucleus in Rust! Rust has been a force multiplier for our team, and
> betting on Rust was one of the best decisions we made. More than performance,
> its ergonomics and focus on correctness has helped us tame sync’s complexity.
> We can encode complex invariants about our system in the type system and have
> the compiler check them for us.

-- [Dropbox.Tech, March 2022](https://dropbox.tech/infrastructure/rewriting-the-heart-of-our-sync-engine)

## Meta

> [Our Rust Engineers] came from Python and Javascript backgrounds. They
> appreciated Rust’s combination of high performance with compile-time error
> detection. As more success stories, such as performance improvements at two to
> four orders of magnitude, circulated within the company, interest grew in
> using Rust for back-end service code and exploring its use in mobile apps as
> well.

-- [Engineering at Meta, April 2021](https://engineering.fb.com/2021/04/29/developer-tools/rust)

## Infineon

> With Infineon's support, we can expect Rust's usage in Embedded Systems to
> become more widespread, standardizing the usage of Rust in the industry while
> engaging with the Rust FOSS community.

-- [Infineon Developer Community Blog, April 2023](https://community.infineon.com/t5/Blogs/Infineon-leads-the-way-Enabling-Rust-for-MCUs-in-the-semiconductor-industry/ba-p/410425)
