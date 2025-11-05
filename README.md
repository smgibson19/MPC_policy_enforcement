# MPC_policy_enforcement
593 semester project: creating a distributed MPC system with data policy enforcement

# Secret Sharing in Rust

The secret share function splits a secret integer into multiple "shares" so that the sum of all shares is a reconstructed version of the original secret. This is a basic illustration of how secret splitting works for secure multi-party computation (MPC).

## How it works

- Split any integer secret into `n` shares.
- Each share (except the last) is a random integer, possibly positive or negative.
- The sum of all shares equals the original secret.
- Reconstruct the original secret by summing the shares.

