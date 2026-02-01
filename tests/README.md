# ML-KEM-RS Test Suite

Comprehensive test suite for the ML-KEM (FIPS 203) implementation in Rust.

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Specific Test Modules
```bash
# Test auxiliary functions (bit manipulation, zeta tables)
cargo test --test test_auxiliary

# Test cryptographic primitives (hash functions, PRF)
cargo test --test test_cryptographic

# Test polynomial operations (addition, multiplication, modulo)
cargo test --test test_polynomial

# Test algebraic operations (NTT, sampling)
cargo test --test test_algebraic

# Test key generation (KPKE)
cargo test --test test_kpke

# Test ByteVec enum
cargo test --test test_bytevec

# Integration tests
cargo test --test integration_tests
```

### Run Specific Tests
```bash
cargo test test_ntt_roundtrip
cargo test test_modulus_is_prime
```

### Run Tests with Output
```bash
cargo test -- --nocapture
cargo test -- --show-output
```

## Test Coverage

### Auxiliary Functions (`test_auxiliary.rs`)
- ✅ Bit reversal lookup table generation
- ✅ Bit reversal function
- ✅ Bytes to bits conversion
- ✅ Bits to bytes conversion (has known issues)
- ✅ Zeta value computation and lookup
- ✅ Regression test for alternating zeros bug

### Cryptographic Functions (`test_cryptographic.rs`)
- ✅ PRF (Pseudo-Random Function) for eta=2 and eta=3
- ✅ Hash function H (SHA3-256)
- ✅ Hash function J (SHAKE256)
- ✅ Function G (SHA3-512 split)
- ✅ Determinism tests
- ✅ Different inputs produce different outputs

### Polynomial Operations (`test_polynomial.rs`)
- ✅ Polynomial creation and indexing
- ✅ Addition (including overflow)
- ✅ Scalar multiplication
- ✅ Modulo reduction
- ✅ Commutativity and associativity
- ✅ Copy semantics

### Algebraic Operations (`test_algebraic.rs`)
- ✅ NTT (Number Theoretic Transform)
- ✅ NTT inverse
- ✅ NTT roundtrip (NTT → NTT⁻¹ → original)
- ✅ Sample NTT (rejection sampling)
- ✅ Sample Poly CBD (Centered Binomial Distribution)
- ✅ Output range validation (all coefficients < q)
- ✅ Regression test for alternating zeros bug

### Key Generation (`test_kpke.rs`)
- ✅ Key generation completes without panic
- ✅ Deterministic behavior
- ✅ Different seeds produce different keys
- ✅ Edge cases (zero seed, max seed)
- ✅ Integration with getrandom

### Integration Tests (`integration_tests.rs`)
- ✅ FIPS 203 parameter sets validation
- ✅ Security levels (NIST Level 1, 3, 5)
- ✅ Modulus primality check
- ✅ Complete workflow tests
- ⚠️ NTT operations (has overflow issue)

## Known Issues

### Critical
1. **NTT Inverse Overflow**: Integer multiplication overflow in `ntt_inv` function
   - Location: `src/algebraic.rs:49`
   - Impact: Prevents NTT roundtrip in release mode

### Minor
2. **bits_to_bytes Index Out of Bounds**: Loop condition error
   - Location: `src/auxiliary.rs:86`
   - Impact: Fails on certain input sizes
   
3. **bytes_to_bits Bit Order**: Possible bit extraction order issue
   - Tests expect different bit ordering
   - May need verification against FIPS 203 spec

## Test Quality Metrics

- **Total Test Functions**: 100+
- **Code Coverage**: High coverage of public APIs
- **Edge Cases**: Zero inputs, max values, boundary conditions
- **Regression Tests**: Tests for previously found bugs
- **Integration Tests**: End-to-end workflow validation

## FIPS 203 Compliance Testing

The test suite validates:
- ✅ Parameter sets (ML-KEM-512, 768, 1024)
- ✅ Modulus q = 3329 (prime)
- ✅ Polynomial ring dimension n = 256
- ✅ Coefficient range [0, q)
- ⚠️ NTT correctness (pending overflow fix)
- ✅ Sampling functions (rejection sampling, CBD)
- ✅ Hash functions (SHA3-256, SHA3-512, SHAKE256)

## Contributing Tests

When adding new features:
1. Add unit tests in the corresponding `test_*.rs` file
2. Add integration tests in `integration_tests.rs` if needed
3. Include edge cases and error conditions
4. Add regression tests for any bugs found
5. Update this README with test coverage

## Test Naming Convention

- `test_<function>_<scenario>`: Basic functionality tests
- `test_<function>_<property>`: Property-based tests
- `test_<module>_<integration>`: Integration tests
- Prefix with `#[ignore]` for slow tests

## Debugging Failed Tests

```bash
# Run with backtrace
RUST_BACKTRACE=1 cargo test

# Run single test with output
cargo test test_name -- --nocapture --exact

# Run in debug mode (slower but catches overflows)
cargo test

# Run in release mode
cargo test --release
```

## Performance Testing

```bash
# Run benchmarks (if implemented)
cargo bench

# Profile test execution
cargo test --release -- --nocapture
```
