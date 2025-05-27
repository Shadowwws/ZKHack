zkhack-there-is-something-in-the-AIR
-------------------

The solution code required changes in ./src/lib.rs and in ./src/prover.rs. At a high level, some part of the initial state is not checked when the nullifier is computed in the trace.