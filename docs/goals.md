[x] Make prime_math.rs library:
    [x] Make floored_sqrt function (using Newton-Raphson method for fun)
    [x] Add a pub func to check for primes, with `upper_bound = floored_sqrt(input)`.

[x] Make prime_table.rs module:
    [x] Make PrimeTable struct
    [x] Add PrimeTable::new() func
    [x] Use the serde to add Serializse & Deserialize traits to PrimeTable struct.
    [x] Add save and load functions using toml and fs libraries.
    [x] Add func to generate primes, and push newly discovered primes to `self.stored_primes`.
    [x] Add func to return a Vec of primes up to an input u32.

[x] Make REPL in main.rs:
    [x] Use reedline to create prompt and read stdin.
    [x] Loop through inputs, generating and printing out a vec of primes up to the input value.
    [x] If input is not an unsigned integer, print a try-again message, and continue the loop.
    [x] Force break when `CTRL-C` or `CTRL-D` are pressed, or if readline error occurs.