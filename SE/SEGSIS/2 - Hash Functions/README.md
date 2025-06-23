# Assignment 2 - Cryptography - SHA-256

**Prof. Avelino Zorzo** - PPGCC/Facin/PUCRS
*Exercise from Introduction to Cryptography, Stanford University, Dan Boneh*

## Problem Statement

Suppose a website hosts a large video file F that anyone can download. Browsers need to verify the file's authenticity before displaying its content. The conventional approach is to:
1. Have the website compute a collision-resistant hash `h = H(F)`
2. Distribute `h` to users via an authenticated channel
3. Browsers download the entire file, verify `H(F) == h`, then display the content

**Problem:** This requires waiting for the entire download to complete before playback can begin.

## Proposed Solution

### Block-based Authentication System
1. Split file F into 1KB blocks (1024 bytes)
2. Compute hashes recursively from last to first block:
   - Hash last block (Bn-1) → hn
   - Append hn to Bn-2 → hash → hn-1
   - Continue until first block (B0) → final hash h0
3. Distribute only h0 via authenticated channel

### Verification Process
As each block (Bi ∥ hi+1) arrives:
1. Compute H(Bi ∥ hi+1)
2. Verify it matches the expected hi
3. If valid, display block immediately

## Security Guarantee
If H is collision-resistant, attackers cannot modify any block without detection.

## Implementation Requirements
- Use **SHA-256** as hash function
- Use existing crypto libraries (PyCrypto, Crypto++, etc.)
- Append hashes as **32 raw bytes (256 bits)**
- Handle files where size ≠ multiple of 1KB (last block may be shorter)

## Tasks
1. Write code to:
   - Compute h0 for a given file F
   - Verify blocks as they arrive at client
2. Test with video file: "FuncoesResumo - Hash Functions.mp4"
3. Write report (1-2 pages) describing:
   - Implementation details
   - Computed h0 for the test file

## Verification Check
For file "FuncoesResumo - SHA1.mp4":
- Expected h0 (hex): `302256b74111bcba1c04282a1e31da7e547d4a7098cdaec8330d48bd87569516`
- Last block hash (hex): `37d88ff100aaf4c63bb828ff1a89f99af2123e143bd758d0eb1573a044e74c84`

## Deliverables
- Implementation code
- 1-2 page report
