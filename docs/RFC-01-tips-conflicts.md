# RFC-01 — Tip Selection and Conflict Resolution (TEQLA)

Status: DRAFT  
Author: teqlalabs  
Date: 2025-09-24

## Goal
Define:
1. How to select tips (which recent transactions a new tx should reference)  
2. How to resolve conflicts (double-spends, nonces, forks)

## Terms
- **Tip**: transaction with no references yet (a leaf of the DAG).  
- **Cumulative Weight (CW)**: number of direct + indirect references a tx has.  
- **Active Window**: set of recent txs (e.g., last N seconds).

## Tip Selection
- `k ∈ [2,4]` (MVP: 2)  
- Sampling:
  - Build `ActiveTips` = tips within the active window (e.g., last 30s).  
  - Prefer tips with lower CW (balances the DAG).  
  - Anti-bias: 20% chance to pick a random tip from the window to avoid centralization.

**Algorithm (MVP)**
1. `Candidates = sort(ActiveTips, by: CW asc, timestamp asc)`  
2. Choose `k` first candidates, with 20% probability replacing one with a random candidate.

## Conflicts
- **Account Nonce Monotonicity**: `nonce(tx) == last_nonce(account) + 1`  
- If two txs share the **same nonce**:
  - prefer the one with **higher CW**;  
  - if still tied, pick the smaller `txid` (lexicographic).  
- **Reorg Bound**: VRF checkpoints (RFC-02) limit rollback depth.

## Validity of References
- A tx is only valid if all `prevs` are known (or asynchronously fetched) and valid.  
- Txs referencing invalid parents become invalid.

## DoS / Spam (related)
- PoUW (RFC-01.1) regulates admission: target ~150–300ms on mobile.  
- Peer scoring: peers propagating invalid txs get their bandwidth reduced.

## Parameters (MVP)
- `k=2`, `active_window=30s`.  
- `cumulative_weight` = simple count (future: lazy-DP optimization).  
- anti-bias randomness: `p=0.2`.

## Tests
- Vectors: nonce collisions, tx flood with conflicts, latency spikes.
