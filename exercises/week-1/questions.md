# Week 1 Questions

### Ch 1:
> [from the CE text](https://drive.google.com/file/d/15csrrLC72dgFdCTxQ4FQBhHtRFDbLzkU/view?usp=share_link)

- Q10. Describe a concrete example where improving the security of a system against one type of attack can increase the likelihood of other attacks.

### Ch 2:

- Q3. Consider a group of 30 people who wish to establish pair-wise secure communications using symmetric-key cryptography. How many keys need to be exchanged in total.

- Q4. Suppose Bob receives a message signed using a digital signature scheme with Alice’s secret signing key. Does it prove that Alice saw the message and chose to sign.

- Q6. Suppose a chosen-ciphertext attacker cannot recover the secret decryption key for an encryption scheme. Does this mean the encryption scheme is secure?

- Q7. Consider a symmetric-key cryptosystem in which cryptographic keys are randomly selected from the set of all n-bit strings. Approximately what should n be in order to provide 128 bits of security against a birthday attack.

### General:

1. Suppose you read about RSA encryption and wanted to find it’s standard specification. Where would you look?

2. Find two libraries for each of RSA, TLS/SSL, and AEAD. Evaluate the maturity each library, and skim the code. What about the library structure makes sense? How is their documentation? These links may help:
https://cryptography.rs/
https://lib.rs/ (librs is equivalent to crates.io, with a different interface)
Benchmark the speed of an algorithm in the two different implementations with Criterion.

3. (depricated) You’re implementing a Tweakable Encryption scheme. You need to know what standard API users will expect. Find a reference for the standard API and write the function signatures for encryption and decryption.
You want to understand a paper on a new polynomial commitment scheme, but you’ve been trying for more than an hour, and the math is over your head. What do you do?

4. Implement the Vignère cipher in 100 lines or less.

5. (a) What is a side channel attack? 

5. (b) Is your cipher implementation constant time?

### Extras

- Read [New Directions in Cryptography](https://ieeexplore.ieee.org/document/1055638).
- Consider ways to contribute what you learned this week to the [Uncloak](https://uncloak.org) knowledge graph.
