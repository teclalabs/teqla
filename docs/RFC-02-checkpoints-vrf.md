# RFC-02 — VRF-Based Checkpoints (TEQLA)

Status: DRAFT  
Author: teqlalabs  
Date: 2025-09-24

## Goal
Define probabilistic checkpoints to achieve finality in ~3–8s without fixed committees.

## Idea
Every epoch `T_epoch` (MVP: 8s):
- Compute `seed_epoch = H(root_dag_hash || epoch_idx)`  
- Each node runs `VRF(seed_epoch)` (PQC-ready; e.g., Dilithium + public verif)  
- If `VRF_out < threshold`, the node may publish a **checkpoint**:
  - `CP = (epoch_idx, vrf_proof, root_dag_hash, merkle_tx_root, time, sig_pqc)`

The network aggregates multiple CPs per epoch. A tx is **finalized** when:
- It is proven to be included in **k** checkpoints within the last `W` epochs, OR  
- There are **≥ m** checkpoints with the same `root_dag_hash`.

## Parameters (MVP)
- `T_epoch = 8s`  
- `threshold`: tuned so ~√N nodes eligible (3–10 per epoch).  
- `k = 2`, `W = 3`, `m = 3` (subject to testnet tuning).

## Proofs
- `vrf_proof` publicly verifiable.  
- Inclusion proof: `merkle_proof(txid)` against `merkle_tx_root`.  
- PQC signatures required on all CPs.

## Security
- No fixed leaders → DDoS harder.  
- Grinding attacks mitigated by unpredictable seed (depends on current DAG).  
- Rollbacks become statistically negligible after ≥1 epoch with multiple CPs.

## SDK Interaction
- `awaitFinality(txid)` checks recent CPs.  
- Expose `GET /checkpoints?from=...` in node RPC (see RFC-03).

## Telemetry
- Publish CPs per epoch, avg latency, proposer distribution.
