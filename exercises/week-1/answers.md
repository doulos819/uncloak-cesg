# Week 1 Answers 

### Ch 1: 

- Q10. Describe a concrete example where improving the security of a system against one type of attack can increase the likelihood of other attacks.
	- This is more of an engineering question than a cryptograhy question per se; security versus performance, and security versus feature complexity are common recurring issue when designing a system. But to give a recent example, the SIKE key-encapsulation protocol was a finalist in the NIST Post Quantum cryptographic algorithm competition. The protocol made it through 5 years of analysis and 4 rounds of highly-public review, before finally revealing weaknesses in August 2022. New cryptographic algorithms may enable new use-cases, but sometimes come with poorly understood security assumptions, which can take years to discover. An engineer attempting to anticipate post-quantum security with a new algorithm increases their risk to not yet well-understood security risks.
	- The lesson: new algorithms, even with security proofs, are not necessarily secure! Time helps matters somewhat: an algorithm that hasn’t been broken in 10 years is more likely to remain unbroken than an algorithm proposed last month.

### Ch 2:
- Q3. Consider a group of 30 people who wish to establish pair-wise secure communications using symmetric-key cryptography. How many keys need to be exchanged in total.
    - - 435 keys would need to be exchanged.($N(N-1)/2$)) where $N$ = 30. If you aren't familiar with this formula, try deriving it yourself, and proving it with induction. Look up the Handshake Lemma or [Triangular numbers](https://en.wikipedia.org/wiki/Triangular_number) for more details.

- Q4. Suppose Bob receives a messages signed using a digital signature scheme with Alice’s secret signing key. Does it prove that Alice saw the message and chose to sign.
    - No,  $K_a$ is long and difficult to remember, much less compute with. If a malicious party takes over Alice's PC, they might be able to forge Alice's signature. 

- Q6. Suppose a chosen-ciphertext attacker cannot recover the secret decryption key for an encryption scheme. Does this mean the encryption scheme is secure?
    - No. The attacker may still recover information about the message (broken diffusion), even if they cannot recover information about the key (successful confusion). See [Confusion and diffusion](https://en.wikipedia.org/wiki/Confusion_and_diffusion). For example, AES in ECB mode does not leak information about the key, but is still obviously broken; you can [see the penguin](https://words.filippo.io/the-ecb-penguin/).

- Q7. Consider a symmetric-key cryptosystem in which cryptographic keys are randomly selected from the set of all n-bit strings. Approximately what should n be in order to provide 128 bits of security against a birthday attack.
    - $n = 256$ in order to provide 128 bits of security against a birthday attack. ($2^{n/2}$ where n = 128).

### General:

1. Suppose you read about RSA encryption and wanted to find it’s standard specification. Where would you look?
	- [IETF Datatracker](https://datatracker.ietf.org/doc/rfc3447/)

2. Find two libraries for each of RSA, TLS/SSL, and AEAD. Evaluate the maturity each library, and skim the code. What about the library structure makes sense? How is their documentation? These links may help:
- https://cryptography.rs/
- https://lib.rs/ (librs is equivalent to crates.io, with a different interface)
	- Repos:
		- The [RustCrypto](https://github.com/RustCrypto), [Ring](https://github.com/briansmith/ring), [RusTLS](https://github.com/rustls/rustls), and [Orion](https://github.com/orion-rs/orion) libraries are reliable references for many of the standard algorithms in the course, also see [Arkworks](https://github.com/arkworks-rs) for ZK specific algorithms. These libraries are typically thoroughly optimized and sometimes audited.
		- Some situations justify reaching for bindings to an implementation in another language, for instance, bindings to the standard [OpenSSL](https://github.com/sfackler/rust-openssl) implementation--though in this case, the RusTLS implementation is known to be [more secure](https://www.zdnet.com/article/a-rust-based-tls-library-outperformed-openssl-in-almost-every-category/) than OpenSSL, *despite* not being standard 🍄. While this course specifically covers Rust implementations of cryptographic algorithms, take a moment to look at the [HACL*](https://hacl-star.github.io/index.html) library of algorithms written in F*. The F* type system lends itself to native formal verification. The Project Everest/HACL* implementations and their [bindings (not yet in production)](https://github.com/franziskuskiefer/evercrypt-rust/tree/main/evercrypt-rs) are formally verified. F* cryptography implementations are especially valuable for widely used software; they have, for example, been deployed in the Linux Kernel and Firefox. Still, these F* lacks the performance and ease-of-implementation of Rust.
	- RSA:
		- https://github.com/RustCrypto/RSA - From the Rust Crypto developers. The example gives a clear example of how to use the library. The repo includes benches and integration tests, and includes a note about the auditing status. The repo is actively maintained. https://lib.rs/crates/ring - Most cryptography libraries will include RSA. Reach for any standard cryptography library.
	- TLS: See discussion above.
	- AEAD:
		- Many Rust Cryptography libraries will rely on standard trait marker libraries like [the RustCrypto AE with Additional Data Traits](https://github.com/RustCrypto/traits/tree/master/aead). The Rust cryptography community uses these traits to recommend general APIs for standard algorithms. AEAD is not an algorithm itself but a class of algorithms with a common API. Standard cryptography implementations typically contain most of their functionality within `impl TRAIT_NAME for MY_IMPLEMENTATION` blocks. See [AEADs/aes-gcm at master · RustCrypto/AEADs · GitHub](https://github.com/RustCrypto/AEADs/tree/master/aes-gcm) examples.
2. (a) Benchmark the speed of an algorithm in the two different implementations with [Criterion](https://lib.rs/crates/criterion). 
    - Video intro: https://youtu.be/eIB3Pd5LBkc
    - User guide: https://bheisler.github.io/criterion.rs/book/index.html

3. (depricated) You’re implementing a Tweakable Encryption scheme. You need to know what standard API users will expect. Find a reference for the standard API and write the function signatures for encryption and decryption.
You want to understand a paper on a new polynomial commitment scheme, but you’ve been trying for more than an hour, and the math is over your head. What do you do?

4. Implement the Vignère cipher in 100 lines or less.

5. (a) What is a side channel attack?

5. (b) Is your cipher implementation constant time?
Extra: Read New Directions in Cryptography.
Extra: Consider ways to contribute what you learned this week to the Uncloak knowledge graph.

### Extras

- Read [New Directions in Cryptography](https://ieeexplore.ieee.org/document/1055638).
- Consider ways to contribute what you learned this week to the [Uncloak](https://uncloak.org) knowledge graph.
