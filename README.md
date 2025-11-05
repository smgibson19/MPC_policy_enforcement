# MPC_policy_enforcement
593 semester project: creating a distributed MPC system with data policy enforcement

# Secret Sharing in Rust

This project demonstrates a simple **secret sharing** scheme in Rust. The program splits a secret integer into multiple "shares" such that the sum of all shares reconstructs the original secret. This is a basic illustration of how secret splitting works, often used in cryptography and secure multi-party computation.

---

## Features

- Split any integer secret into `n` shares.
- Each share (except the last) is a random integer, possibly positive or negative.
- The sum of all shares equals the original secret.
- Reconstruct the original secret by summing the shares.

---

## How It Works

1. **Secret Sharing**:  
   The `share` function generates `n-1` random integers with random signs. The last share is calculated as:  

