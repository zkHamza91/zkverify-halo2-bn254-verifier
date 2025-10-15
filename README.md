# 🧩 zkVerify Phase 2 — Halo2 BN254 Verifier Pallet

**Author:** zkHamza91  
**Challenge:** Phase 2 — Build a Verifier  
**Reward:** 50,000 VFY  
**Verifier Type:** Halo2 BN254  
**Status:** ✅ Fully Compiling (no_std / wasm32-unknown-unknown)

---

## 📘 Overview

This repository implements a minimal **Substrate `no_std` pallet** that integrates a **Halo2 BN254 proof verifier**.  
It satisfies all requirements from [docs.zkverify.io → Phase 2 Challenge](https://docs.zkverify.io/incentivizedtestnet/challenges/phase2_challenges/build_a_verifier):

- ✅ `no_std` compatible  
- ✅ Compiles for `wasm32-unknown-unknown` target  
- ✅ Uses only crates.io and local path dependencies (no SDK clone)  
- ✅ Includes a custom verifier crate (`halo2_bn254_verifier`)  
- ✅ FRAME v14 pallet structure with `construct_runtime!` integration example  

---

## 🧠 Workspace Layout

zkVerify_solo/
├── halo2_bn254_verifier
│ ├── src
│ │ └── lib.rs
│ └── Cargo.toml
├── pallets
│ └── halo2-bn254-verifier
│ ├── src
│ │ └── lib.rs
│ └── Cargo.toml
├── target
│ ├── debug
│ │ ├── build
│ │ ├── deps
│ │ ├── examples
│ │ └── incremental
│ ├── wasm32-unknown-unknown
│ │ ├── debug
│ │ └── CACHEDIR.TAG
│ └── CACHEDIR.TAG
├── Cargo.lock
├── Cargo.toml
└── README.md


### 🏗 Crates
| Crate | Description |
|-------|--------------|
| `halo2_bn254_verifier` | A minimal no_std Halo2 verifier library using `halo2_proofs`, `ff`, and `group`. |
| `pallet-halo2-bn254-verifier` | A Substrate pallet exposing a `verify()` extrinsic that calls the verifier. |

---

## 🚀 How to Use the Build

### 🧱 Building the Pallet

Verify the build environment and confirm `no_std` compatibility:

```bash

# Clean workspace
cargo clean

# Build for no_std (WASM target)
cargo check -p pallet-halo2-bn254-verifier --target wasm32-unknown-unknown --no-default-features

