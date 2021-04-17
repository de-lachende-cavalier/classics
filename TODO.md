[x] Refactor the whole thing with better encapsulation (using traits and structs and impls, etc.)
[x] Add decryption tests to all the test_known_pairs() in cipher/ (check out scytale.rs to see what i mean)
[] Implement the Solitaire cipher (by Schneier)
[] Refactor once again (the methods of the various structs should take selfs as parameters (some of them, at least))
[] Manage the various available cipher in lib.rs through some config file (YAML, JSON or some other format) => updating manually is a pain
[] Further inspect scytale.rs (still some work to do on that)
[] Add benchmark for scytale
[] Add proper documentation
[] Optimize the implementation
