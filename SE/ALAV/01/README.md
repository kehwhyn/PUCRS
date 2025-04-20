# Reheating Exercise – Recursion

Let’s do some programming exercises to refresh our memory on recursive algorithms. This is also an opportunity to work with our development environment (Git-GitHub-GitHub Classroom).

---

### Tasks

1. **Develop recursive algorithms** for the following problems and implement them in **Java**:

   1. **Multiplication** of two natural numbers using repeated addition.
      *(Example: 6 × 4 = 4 + 4 + 4 + 4 + 4 + 4)*

   2. **Addition** of two natural numbers using repeated increments.
      *(Example: 3 + 2 = + + (+ + + 1))*

   3. Compute the series:
      **1 + 1/2 + 1/3 + 1/4 + ... + 1/N**

   4. **String reversal**.

   5. Generator for the sequence defined by:
      - F(1) = 1
      - F(2) = 2
      - F(n) = 2 × F(n − 1) + 3 × F(n − 2)

   6. **Ackermann sequence generator**:
      - A(m, n) = n + 1, if m = 0
      - A(m, n) = A(m − 1, 1), if m ≠ 0 and n = 0
      - A(m, n) = A(m − 1, A(m, n − 1)), if m ≠ 0 and n ≠ 0

   7. Given an array of integers, **calculate the sum and product** of the elements.

   8. Check whether a given word is a **palindrome**.
      *(Examples: aba, abcba, xyzzyx)*

   9. Given a number **n**, generate **all possible combinations** of the first **n letters** of the alphabet.
      *(Example: n = 3 → ABC, ACB, BAC, BCA, CAB, CBA)*

   10. Define a **generalized Fibonacci sequence** from `f0` to `f1` as:
       - fibg(f0, f1, 0) = f0
       - fibg(f0, f1, 1) = f1
       - fibg(f0, f1, n) = fibg(f0, f1, n − 1) + fibg(f0, f1, n − 2), if n > 1
