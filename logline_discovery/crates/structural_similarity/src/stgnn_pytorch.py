#!/usr/bin/env python3
"""Placeholder ST-GNN inference script.

Reads JSON from stdin in the format:
{
  "span_a": {...},
  "span_b": {...}
}

Outputs JSON { "similarity": <float> } where the score is currently a dummy
value (0.42). Replace the body of `compute_similarity` with real PyTorch Geometric
logic when available.
"""
import json
import sys

def compute_similarity(span_a, span_b):
    # TODO: integrate PyTorch Geometric ST-GNN inference here.
    return 0.42


def main():
    raw = sys.stdin.read()
    payload = json.loads(raw)
    span_a = payload.get("span_a", {})
    span_b = payload.get("span_b", {})
    sim = compute_similarity(span_a, span_b)
    json.dump({"similarity": sim}, sys.stdout)

if __name__ == "__main__":
    main()
