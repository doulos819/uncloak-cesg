# Week 1 Answers 

### Ch 1: (from the CE text)

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

General:

1. Suppose you read about RSA encryption and wanted to find it’s standard specification. Where would you look?
Find two libraries for each of RSA, TLS/SSL, and AEAD. Evaluate the maturity each library, and skim the code. What about the library structure makes sense? How is their documentation? These links may help:
https://cryptography.rs/
https://lib.rs/ (librs is equivalent to crates.io, with a different interface)
Benchmark the speed of an algorithm in the two different implementations with Criterion.

2. (depricated) You’re implementing a Tweakable Encryption scheme. You need to know what standard API users will expect. Find a reference for the standard API and write the function signatures for encryption and decryption.
You want to understand a paper on a new polynomial commitment scheme, but you’ve been trying for more than an hour, and the math is over your head. What do you do?

3. Implement the Vignère cipher in 100 lines or less.

4. (a) What is a side channel attack?

4. (b) Is your cipher implementation constant time?
Extra: Read New Directions in Cryptography.
Extra: Consider ways to contribute what you learned this week to the Uncloak knowledge graph.

