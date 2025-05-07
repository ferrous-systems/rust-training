# What is Rust?

---

* The 100-foot view
* Where did Rust come from?
* Who's in charge now?
* Is this a community I can engage with?
* What does Rust run on?
* What does Rust cost?
* Can I build safety-critical systems?

## The 100-foot view

## A free and open-source __systems programming__ language

> A language empowering everyone to build reliable and efficient software.

## Hello, World

```rust
fn main() {
    println!("Hello, world!");
}
```

## You can build...

<div class="columns">
<div>

* Network Services
* Command-line Apps
* Web Apps
* Desktop Apps
* Bootloaders

</div>
<div>

* Device Drivers
* Hypervisors
* Embedded Systems
* Libraries/plugins for applications in other languages

</div>
</div>

## Front-end or Back-end?

It's applicable at *every point* in the stack!

## The Three Words

-   Safety <!-- .element: class="fragment" -->
-   Performance <!-- .element: class="fragment" -->
-   Productivity <!-- .element: class="fragment" -->

## It's enduringly popular

 [Stack Overflow Survey 2024](https://survey.stackoverflow.co/2024/technology#admired-and-desired):

> Rust continues to be the most-admired programming language with an 83% score this year

Note:

Stack Overflow used to use the term *most loved*, which Rust won seven years in
a row. In 2023 they changed the terms to *desired* and *admired*. Rust was the
*most admired* language in 2023 and 2024.

## Cross-platform

* Windows, macOS, Linux
* iOS, Android, Web, QNX, Bare-metal, etc

## Portable

* Source code is portable across multiple architectures:
  * x86, RISC-V and Arm
  * Power, MIPS, SPARC, ...

## Rust can *import* C-compatible libraries

Want to use `zlib`, `OpenSSL`, `SomeSpecialDriverLib`? Sure!

## Rust can *export* C-compatible libraries

* Python extension modules? [Ok!](https://pypi.org/project/polars)
* Android native libraries? [No problem.](https://cs.android.com/android/platform/superproject/main/+/main:packages/modules/DnsResolver/doh/encoding.rs)
* Replace the file parser in your Very Large C++ Application? [Can-do.](https://wiki.mozilla.org/Oxidation)

## Where did Rust come from?

## A Little Bit of History

* Rust began around 2008
* An experimental project by [Graydon Hoare](https://www.technologyreview.com/2023/02/14/1067869/rust-worlds-fastest-growing-programming-language/)
* Adopted by Mozilla
* Presented to the general public as version 0.4 in 2012

## Focus

* Rust lost many features from 2012 to 2014
  * garbage collector
  * evented runtime
  * complex error handling
  * etc
* Rust oriented itself towards being a usable systems programming language

## Development

* Always together with a larger project (e.g. Servo)
* Early adoption of regular releases
* RFC process
* Editions

## Public Release

* First 1.0 release in 2015
  * <https://blog.rust-lang.org/2015/05/15/Rust-1.0.html>
  * "This release is the official beginning of our commitment to stability"
* New release *every six weeks* since

## Who's in charge now?

## The Rust Project

<https://www.rust-lang.org/governance>

<div class="columns">
<div>

* The Leadership Council
* Compiler Team
* Dev Tools Team
* Infrastructure Team

</div>
<div>

* Language Team
* Library Team
* Moderation Team
* Launching Pad Team

</div>
</div>

## Working Groups

<div class="columns">
<div>

* Async WG
* Command-line Interface WG
* Embedded devices WG
* Game Development WG

</div>
<div>

* Rust by Example WG
* Secure Code WG
* Security Response WG
* WebAssembly (WASM) WG

</div>
</div>

## The Rust Foundation

> ... is an independent non-profit organization dedicated to stewarding the Rust
> programming language, nurturing the Rust ecosystem, and supporting the set of
> maintainers governing and developing the project.

## It has a powerful list of members

<https://foundation.rust-lang.org/members/>

## Who decides on new features?

* Discuss in chat/forums
* Open a [Request For Change (RFC)](https://github.com/rust-lang/rfcs)
* Relevant team takes a vote
* Tracking ticket is created
* Pull Request(s) to implement the change
* Stabilisation

## Summary

* Rust is a collaborative open-source project that prides itself on inclusion
* There is no "owner", nor "BDFL"
* It has strong financial backing
* It remains a work-in-progress

## Is this a community I can engage with?

## A strong Code of Conduct

The Rust Project, and pretty much the whole Community, follow a [Code of
  Conduct](https://www.rust-lang.org/policies/code-of-conduct):

> We are committed to providing a friendly, safe and welcoming environment for
> all, regardless of level of experience, gender identity and expression, sexual
> orientation, disability, personal appearance, body size, race, ethnicity, age,
> religion, nationality, or other similar characteristic.

## A strong Code of Conduct

> Likewise any spamming, trolling, flaming, baiting or other attention-stealing behavior is not welcome.

* Builds on efforts in other communities

## Why?

* Because a community is only as strong as its members

> Going beyond technical points, Rust has a vibrant, welcoming community -
> ([Stack Overflow Blog](https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/))

## Why?

* If you allow both wolves *and* sheep into your space, you won't get any sheep
* The Rust Community seems to have a higher than average representation from the
  LGBTQI+ community

## So beginners are welcome?

* Absolutely!
* Relatively speaking, we're *all* still beginners
* You even see open tickets on the rust-lang Github marked as *E-easy: Good
  first issue*.

## This extends to the compiler's interface...

* Any Rust error message which is unclear or ambiguous...
* ... is considered a bug and will be fixed ...
* ... if you open a ticket (or post @ the right people)

## Compiler Error Driven Development works!

<pre><code data-trim data-noescape><span class="er b">error[E0502]</span><b>: cannot borrow `name` as mutable because it is also borrowed as immutable</b>
<span class="eb b"> --&gt; </span>src/main.rs:4:5
<span class="eb b">  |</span>
<span class="eb b">3 |</span>     let nickname = &amp;name[..3];
<span class="eb b">  |</span>                     <span class="eb b">----</span> <span class="eb b">immutable borrow occurs here</span>
<span class="eb b">4 |</span>     name.clear();
<span class="eb b">  |</span>     <span class="er b">^^^^^^^^^^^^</span> <span class="er b">mutable borrow occurs here</span>
<span class="eb b">5 |</span>     println!(&quot;Hello there, {}!&quot;, nickname);
<span class="eb b">  |</span>                                  <span class="eb b">--------</span> <span class="eb b">immutable borrow later used here</span>
<b>Some errors have detailed explanations: E0502, E0596.</b>
<b>For more information about an error, try `rustc --explain E0502`.</b>
</code></pre>

## What does Rust run on?

## Host vs Target

* The machine you develop on
* The machine the program runs on

## Rust is a cross-compiler

* It uses LLVM to generate machine code
* *Every* Rust install is a cross-compiler
  * No rummaging for extra installers for your specific target

## Hosts

* Windows (__x86-64__, __x86__, AArch64)
* Linux (__x86-64__, __x86__, __AArch64__, AArch32, RISC-V, PowerPC, S390)
* macOS (__x86-64__, __AArch64__)
* plus FreeBSD, NetBSD and Illumos

## Targets

* All of the above, plus...
* Android
* iOS/watchOS/tvOS
* Bare-metal Embedded
* QNX, VxWorks, AIX
* WebAssembly
* UEFI
* Nintendo Switch, Sony PSP and PS Vita...
* Add your own!

## What does Rust cost?

## Rust is Open Source

* Under the MIT or Apache-2.0 licences
* You can compile `rustc` and `cargo` yourself
* <https://github.com/rust-lang/rust>

## Binaries are provided free of charge

* Available using the `rustup` tool
* [AWS](https://aws.amazon.com/blogs/opensource/aws-sponsorship-of-the-rust-project/) sponsor the project
* Nothing to sign, no USB dongle required

## Support is available

* There are lots of places you can go for help
  * Forums, Discord, Reddit
  * Professional consulting firms
  * Rust Toolchain vendors

## No-one is an expert overnight

* Budget for some training
* Budget for some time for the team to gain experience
* Budget for some support when the team have questions

## You might need a bigger computer...

> Today, compiling the __Rust compiler__ on a 4-core CPU, that is typically
> found in a standard laptop, takes up to 15 minutes with another 5-10 minutes
> for tests. However, a 96-core cloud virtual machine can complete the same
> build in less than 5 minutes with tests completing in 35 seconds.

## Compile time checks vs run-time checks

* Rust does a lot of work *up front*
* The faster your checks run, the more productive you are!
* A Raspberry Pi 4 technically works, but it takes a while...

## Can I build safety-critical systems?

## Some terminology

* a *system* is *certified* as being sufficiently safe/correct
* that *system* is often built using *qualified* tools
* *quality* is the result of an ongoing process

Note:

Some industries use the terms *certification* and *qualification*
interchangeably.

## What is a safety-critical system?

Generally built following a standard, like ISO 26262:

> ISO 26262 is intended to be applied to safety-related systems that include one
> or more electrical and/or electronic (E/E) systems and that are installed in
> series production passenger cars with a maximum gross vehicle mass up to 3500
> kg.

## What is a safety-critical system?

Generally built following a standard, like ISO 26262:

> This document describes a framework for functional safety to assist the
> development of safety-related E/E systems. This framework is intended to be
> used to integrate functional safety activities into a company-specific
> development framework.

## And for other applications:

* __DO-178C__ *Software Considerations in Airborne Systems and Equipment
  Certification*
* __IEC 61508__ *Functional Safety of Electrical/Electronic/Programmable Electronic
  Safety-related Systems*
* __IEC 62278__ *Railway applications - Specification and demonstration of
  reliability, availability, maintainability and safety*
* __IEC 62034__ *Medical device software – Software life cycle processes*
* There are many others...

## Can I use Rust?

* Well you can use C
* And C is kinda risky...
* But processes have been developed to manage that risk
* And C toolchains have been *qualified* so you can rely on them doing what they say
  they are going to do
  * If you hold them the right way

## Language Specifications

* C has *ISO/IEC 9899:2018* (C17)
* C++ has *ISO/IEC 14882:2020(E)* (C++20)
* Rust doesn't have a standard
  * The open-source compiler *is* the standard
  * The first ISO C standard (C90) came 17 years after C was invented, largely
    because there were a lot of different competing compilers

## Ferrocene

> Ferrocene is the open-source qualified Rust compiler toolchain for safety- and
> mission-critical. Qualified for automotive and industrial development.
>
> ISO26262 (ASIL D) and IEC 61508 (SIL 4) available for x86 and ARM platforms.

## Ferrocene

* To produce Ferrocene, we first wrote the *Ferrocene Language Specification*
  * See <https://spec.ferrocene.dev>
  * It's [being upstreamed](https://blog.rust-lang.org/2025/03/26/adopting-the-fls/) as the official spec
* Ferrocene is based on the open-source Rust compiler
  * Additional testing and run-time checks in the toolchain
  * Lots of documentation!
* Ferrocene itself is open-source software
  * <https://github.com/ferrocene/ferrocene>
* Pricing and support options at <https://ferrocene.dev>
* Other companies have similar offerings

---

* The 100-foot view
* Where did Rust come from?
* Who's in charge now?
* Is this a community I can engage with?
* What does Rust run on?
* What does Rust cost?
* Can I build safety-critical systems?
