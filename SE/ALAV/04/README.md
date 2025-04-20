# Preparatory Exercise Statement on Divide and Conquer

1. Let's begin with an already known and studied algorithm (from AEDI). Merge Sort is a sorting algorithm based on the following steps:

   * recursively sort the left half of the array
   * recursively sort the right half of the array
   * merge the two halves to get the sorted array.

   Therefore:

   * implement the algorithm below;
   * test it with arrays of integers with random content and sizes 32, 2048, and 1,048,576. In these tests, count the number of iterations the algorithm performs, and the time spent;

   ```java
   MERGE-SORT(L: List with n elements) : Ordered list with n elements
       IF (list L has one element)
           RETURN L.
       Divide the list into two halves A and B.
       A ← MERGE-SORT(A).
       B ← MERGE-SORT(B).
       L ← MERGE(A, B).
       RETURN L.
   ```

2. The algorithm below (which does not use divide and conquer) finds the maximum value in an array.

   Again:

   * implement the algorithm below;
   * test it with arrays of integers with random content and sizes 32, 2048, and 1,048,576. In these tests, count the number of iterations the algorithm performs, and the time spent;

   ```java
   int maxVal1(int A[], int n) {
       int max = A[0];
       for (int i = 1; i < n; i++) {
           if( A[i] > max )
              max = A[i];
       }
       return max;
   }
   ```

3. The algorithm below (which uses divide and conquer) finds the maximum value in an array.

   Again:

   * implement the algorithm below;
   * test it with arrays of integers with random content and sizes 32, 2048, and 1,048,576. In these tests, count the number of iterations the algorithm performs, and the time spent;

   ```java
   int maxVal2(int A[], int init, int end) {
       if (end - init <= 1)
           return max(A[init], A[end]);
       else {
             int m = (init + end)/2;
             int v1 = maxVal2(A,init,m);
             int v2 = maxVal2(A,m+1,end);
             return max(v1,v2);
            }
   }
   ```

4. Integer Multiplication of n-bits receives two integers `x` and `y` of n-bits and returns the result of x * y.

   Again:

   * implement the algorithm below;
   * test it with 3 cases of integer values: 4-bit, 16-bit, and 64-bit. In these tests, count the number of iterations the algorithm performs, and the time spent.

   The algorithm is given below:

   ```java
   MULTIPLY(x, y, n)
      IF (n = 1)
         RETURN x * y.
      ELSE
         m ← ⎡ n / 2 ⎤.
         a ← ⎣ x / 2^m ⎦; b ← x mod 2^m.
         c ← ⎣ y / 2^m ⎦; d ← y mod 2^m.
         e ← MULTIPLY(a, c, m).
         f ← MULTIPLY(b, d, m).
         g ← MULTIPLY(b, c, m).
         h ← MULTIPLY(a, d, m).
         RETURN 2^(2m)*e + 2^m*(g + h) + f.
   ```

   Adjust the signature of your implementation to use the `long` integer type in Java.

5. Build a table with the results of the executions above. The rows of the table are the implemented algorithms, the columns are the sizes of the vectors used for testing and measurement.
