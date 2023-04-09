# Rust back-end for brute-forcing hashcode
A Rust project that will handle brute-forcing of hashcodes. It will run as a web service that accepts a number of endpoints, like:
- Status: Are there any threads running, how many permutations does it have to check, etc.
- Supported hash functions to check: MD5, SHA-1, etc.
- Hardware available: CPU, video cards, etc.
- Hashcode to brute force. Also include length to check, expected characters, etc.

The web services can run from a different server if needed.
