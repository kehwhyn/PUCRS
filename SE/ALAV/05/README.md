# Introductory Exercise Prompt on Dynamic Programming

1. Given the three Fibonacci implementations below:

   * implement the algorithms;
   * test all for integers 4, 8, 16, 32;
   * also test the last two for integers 128, 1000, and 10,000.

   ```java
   FIBO-REC (n)
      if n ≤ 1
      then return n
      else a ← FIBO-REC (n − 1)
           b ← FIBO-REC (n − 2)
           return a + b
   ```

   ```java
   FIBO (n)
        f [0] ← 0
        f [1] ← 1
        for i ← 2 to n do
           f[i] ← f[i-1]+f[i-2]
        return f [n]
   ```

   ```java
   MEMOIZED-FIBO (f, n)
        for i ← 0 to n do
            f [i] ← −1
        return LOOKUP-FIBO (f, n)

   LOOKUP-FIBO (f, n)
        if f [n] ≥ 0
        then return f [n]
        if n ≤ 1
        then f [n] ← n
        else f [n] ← LOOKUP-FIBO(f, n − 1) + LOOKUP-FIBO(f, n − 2)
        return f [n]
   ```

2. Create a table with the execution results above. The rows are the implemented algorithms, and the columns are the tested and measured values.

3. Solve the knapsack problem as explained in class.
   1. Find a solution that tests all possible combinations and selects the best, applying divide-and-conquer or not;
   1. Count the number of iterations;
   1. Implement and test your solution for the case presented in class and other similar-size cases (;-).

4. Implement and test the knapsack algorithm presented in class. Count the number of iterations and compare it with your previous solution.

5. Solve the knapsack problem using the dynamic programming algorithm shown in class, test it and count the number of iterations.

6. Create a table with the results and number of iterations for both implementations, for the test cases available on Moodle.

```java
Integer backPackPD(Integer N, Integer C, Tuple<Integer, Integer> items)

   N = number of products;
   C = actual capacity of the backpack

   items[N +1]; (Index 0 holds null)

   maxTab[N+1][C+1];
   Initialize row 0 and column 0 with 0;
   for i = 1 to N
   for j = 1 to C

   if item items[i].weight <= j // if the item fits in the current backpack
      maxTab[i][j] = Max(maxTab[i-1][j],
      items[i].value +
      maxTab[i-1][j – items[i].weight]);
   else
      maxTab[i][j] = maxTab[i-1][j];

   return maxTab[N][C] // maximum value for a backpack of capacity C
// that can contain items from item 1 to item N.
```

# Edit Distance Class

```
ED(S, T, i, j): int
// S: Initial string, T: Target string, i: [1..m], j:[1..n]
// returns the minimum number of edits when comparing
// S[i] with T[j]. m is the size of S, n the size of T
//
Base Case:
When we run out of characters to compare in S or T. If in both, the
result is 0. If in only one, return the remaining characters of the one
that is not empty;
Recursive Cases:
If S[i] == T[i], call recursively ED(S, T, i-1, j-1) (it's a match, nothing
needs to be done at this position, cost is zero).
If not, three recursive calls are necessary:
• Substitution: ED(S, T, i-1, j-1) + 1
• Insertion: ED(S, T, i, j-1) + 1
• Deletion: ED(S, T, i-1, j) + 1
• Return the one that results in the lowest cost
```

```java
// Assuming the Costs: Deletion=D, Insertion=I , Substitution=S and Match=M=0;

int distEdProgDina(string A, string B)
    m = length(A);
    n = length(B);
    matrix[0][0] = 0;
    for i = 1 to m
       matrix[i][0] = matrix[i-1][0] + 1  // add an I;
    for j = 1 to n
       matrix[0][j] = matrix[0][j-1] + 1  // add a D;
    for i = 1 to m
       for j = 1 to n
          if A[i] == B[j]
             extraCost = 0 // Match;
          else
             extraCost = 1 // Substitution;
          matrix[i][j] = Min(matrix[i-1][j] +1, matrix[i][j-1] +1,
                             matrix[i-1][j-1] + extraCost);
    return matrix[m][n];
```
