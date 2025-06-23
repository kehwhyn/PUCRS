# Systems Security/Cryptography – Simulating HTTPS
**Prof. Avelino Zorzo**

**Polytechnic School – PUCRS**

### Objective
This assignment aims to simulate part of the HTTPS protocol's functionality.

### Given Parameters (RFC5114)
**Prime `p` (hexadecimal):**
```
B10B8F96 A080E01D DE92DE5E AE5D54EC 52C99FBC FB06A3C6
9A6A9DCA 52D23B61 6073E286 75A23D18 9838EF1E 2EE652C0
13ECB4AE A9061123 24975C3C D49B83BF ACCBDD7D 90C4BD70
98488E9C 219A7372 4EFFD6FA E5644738 FAA31A4F F55BCCC0
A151AF5F 0DC8B4BD 45BF37DF 365C1A65 E68CFDA7 6D4DA708
DF1FB2BC 2E4A4371
```
**Generator `g` (hexadecimal):**
```
A4D1CBD5 C3FD3412 6765A442 EFB99905 F8104DD2 58AC507F
D6406CFF 14266D31 266FEA1E 5C41564B 777E690F 5504F213
160217B4 B01B886A 5E91547F 9E2749F4 D7FBD7D3 B9A92EE1
909D0D22 63F80A76 A6A24C08 7A091F53 1DBF0A01 69B6A28A
D662A4D1 8E73AFA3 2D779D59 18D08BC8 858F4DCE F97C2A24
855E6EEB 22B3B2E5
```

### Assignment Structure
#### Phase 1: Key Generation Using Diffie-Hellman
1. **Step 1:**
   - Generate a random value `a` (at least ~30 digits, smaller than `p`).
   - Compute `A = g^a mod p`.
   - Send `A` (in hexadecimal) to the professor.

2. **Step 2:**
   - Receive a value `B` (hexadecimal) from the professor.
   - Compute `V = B^a mod p`.

3. **Step 3:**
   - Compute `S = SHA256(V)`.
   - Use the **first 128 bits of `S`** as the shared secret key for communication.

#### Phase 2: Secure Message Exchange
- **Receive a message** from the professor (hexadecimal), encrypted with **AES-CBC + padding**.
  - Format: `[128-bit IV][ciphertext]` (concatenated, in hex).
- **Decrypt the message**, then send it back **encrypted and reversed** (e.g., if received `"ola"`, return `"alo"`).
  - Format: `[128-bit random IV][reversed ciphertext]` (in hex).

### Program Requirements
- For **Phase 1**, the program must:
  - Generate `a` and `A`, printing both (in hex).
  - Accept an input `B` (hex) and the previously generated `a`, then output the first **128 bits of `S` (hex)**.
- For **Phase 2**, the program must:
  - Accept the encrypted message and the key (`S`’s first 128 bits in hex).
  - Print the **decrypted received message (plaintext)** and the **response (hexadecimal, encrypted and reversed)**.

### Submission
- A **well-commented program** with all generated/exchanged values.
- Messages may (and should) be sent in advance.
