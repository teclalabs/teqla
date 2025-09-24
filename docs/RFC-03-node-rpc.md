# RFC-03 — Node RPC (TEQLA)

Status: DRAFT  
Author: teqlalabs  
Date: 2025-09-24

## Goal
Specify minimal RPC endpoints for SDK and dApps.

## HTTP/JSON (MVP)
- `GET /status` → { version, height, tips, peers, epoch, latestCheckpoints[] }  
- `GET /tips` → [txid, ...]  
- `POST /tx` → { rawTx } → { txid }  
- `GET /tx/{txid}` → { txid, status: pending|finalized|rejected, includedInCp?: cpId }  
- `GET /checkpoints?from=<epoch>` → [{ cpId, epoch, root, merkleRoot, proposer, vrfProof }]  
- `GET /account/{addr}` → { nonce, balance }  
- `POST /estimate-pouw` → { device: 'mobile'|'desktop' } → { targetMs }

## WebSocket (optional)
- `subscribe: newTx`, `subscribe: checkpoint`, `subscribe: finalized`

## Notes
- `rawTx` includes PQC signature + `blob_id`, `nonce_work`, `pouw`.  
- Authentication: none (public). Anti-abuse: rate limiting at the edge.
