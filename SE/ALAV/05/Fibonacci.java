import java.math.BigInteger;

public class Fibonacci {
    public static int iterations;

    public static void main(String[] args) {
        System.out.println("+------------------------------------+");
        System.out.println("|               fibo_rec             |");
        System.out.println("+------------------------------------+");
        System.out.println("| nth number | time(ms) | iterations |");
        System.out.println("+------------------------------------+");

        var start = System.currentTimeMillis();
        fibo_rec(4);
        var end = System.currentTimeMillis() - start;
        System.out.println(String.format("|%12d|%10d|%12d|", 4, end, iterations));

        iterations = 0;
        start = System.currentTimeMillis();
        fibo_rec(8);
        end = System.currentTimeMillis() - start;
        System.out.println(String.format("|%12d|%10d|%12d|", 8, end, iterations));

        iterations = 0;
        start = System.currentTimeMillis();
        fibo_rec(16);
        end = System.currentTimeMillis() - start;
        System.out.println(String.format("|%12d|%10d|%12d|", 16, end, iterations));

        iterations = 0;
        start = System.currentTimeMillis();
        fibo_rec(32);
        end = System.currentTimeMillis() - start;
        System.out.println(String.format("|%12d|%10d|%12d|", 32, end, iterations));
        System.out.println("+------------------------------------+\n");

        System.out.println("+------------------------------------+");
        System.out.println("|                 fibo               |");
        System.out.println("+------------------------------------+");
        System.out.println("| nth number | time(ms) | iterations |");
        System.out.println("+------------------------------------+");

        iterations = 0;
        start = System.currentTimeMillis();
        fibo(4);
        end = System.currentTimeMillis() - start;
        System.out.println(String.format("|%12d|%10d|%12d|", 4, end, iterations));

        iterations = 0;
        start = System.currentTimeMillis();
        fibo(8);
        end = System.currentTimeMillis() - start;
        System.out.println(String.format("|%12d|%10d|%12d|", 8, end, iterations));

        iterations = 0;
        start = System.currentTimeMillis();
        fibo(16);
        end = System.currentTimeMillis() - start;
        System.out.println(String.format("|%12d|%10d|%12d|", 16, end, iterations));

        iterations = 0;
        start = System.currentTimeMillis();
        fibo(32);
        end = System.currentTimeMillis() - start;
        System.out.println(String.format("|%12d|%10d|%12d|", 32, end, iterations));

        iterations = 0;
        start = System.currentTimeMillis();
        fibo(128);
        end = System.currentTimeMillis() - start;
        System.out.println(String.format("|%12d|%10d|%12d|", 128, end, iterations));

        iterations = 0;
        start = System.currentTimeMillis();
        fibo(1000);
        end = System.currentTimeMillis() - start;
        System.out.println(String.format("|%12d|%10d|%12d|", 1000, end, iterations));

        iterations = 0;
        start = System.currentTimeMillis();
        fibo(10000);
        end = System.currentTimeMillis() - start;
        System.out.println(String.format("|%12d|%10d|%12d|", 10000, end, iterations));
        System.out.println("+------------------------------------+\n");

        System.out.println("+------------------------------------+");
        System.out.println("|            memoized_fibo           |");
        System.out.println("+------------------------------------+");
        System.out.println("| nth number | time(ms) | iterations |");
        System.out.println("+------------------------------------+");

        BigInteger[] vec = new BigInteger[5];
        iterations = 0;
        start = System.currentTimeMillis();
        memoized_fibo(vec, 4);
        end = System.currentTimeMillis() - start;
        System.out.println(String.format("|%12d|%10d|%12d|", 4, end, iterations));

        vec = new BigInteger[9];
        iterations = 0;
        start = System.currentTimeMillis();
        memoized_fibo(vec, 8);
        end = System.currentTimeMillis() - start;
        System.out.println(String.format("|%12d|%10d|%12d|", 8, end, iterations));

        vec = new BigInteger[17];
        iterations = 0;
        start = System.currentTimeMillis();
        memoized_fibo(vec, 16);
        end = System.currentTimeMillis() - start;
        System.out.println(String.format("|%12d|%10d|%12d|", 16, end, iterations));

        vec = new BigInteger[33];
        iterations = 0;
        start = System.currentTimeMillis();
        memoized_fibo(vec, 32);
        end = System.currentTimeMillis() - start;
        System.out.println(String.format("|%12d|%10d|%12d|", 32, end, iterations));

        vec = new BigInteger[129];
        iterations = 0;
        start = System.currentTimeMillis();
        memoized_fibo(vec, 128);
        end = System.currentTimeMillis() - start;
        System.out.println(String.format("|%12d|%10d|%12d|", 128, end, iterations));

        vec = new BigInteger[1001];
        iterations = 0;
        start = System.currentTimeMillis();
        memoized_fibo(vec, 1000);
        end = System.currentTimeMillis() - start;
        System.out.println(String.format("|%12d|%10d|%12d|", 1000, end, iterations));

        vec = new BigInteger[10001];
        iterations = 0;
        start = System.currentTimeMillis();
        memoized_fibo(vec, 10000);
        end = System.currentTimeMillis() - start;
        System.out.println(String.format("|%12d|%10d|%12d|", 10000, end, iterations));
        System.out.println("+------------------------------------+");
    }

    public static int fibo_rec(int n) {
        iterations++;

        if (n <= 1)
            return n;

        return fibo_rec(n - 1) + fibo_rec(n - 2);
    }

    public static int fibo(int n) {
        int[] f = new int[n + 1];

        f[0] = 0;
        f[1] = 1;

        for (int i = 2; i <= n; i++) {
            iterations++;
            f[i] = f[i - 1] + f[i - 2];
        }

        return f[n];
    }

    public static BigInteger memoized_fibo(BigInteger[] f, int n) {
        for (int i = 0; i <= n; i++)
            f[i] = BigInteger.valueOf(-1);

        return lookup_fibo(f, n);
    }

    // Memory goes boom because there's too many recursive calls
    // Up to 8000 is ok, maybe if I had more memory I could go higher
    public static BigInteger lookup_fibo(BigInteger[] f, int n) {
        iterations++;

        if (f[n].compareTo(BigInteger.ZERO) >= 0)
            return f[n];

        if (n <= 1)
            f[n] = BigInteger.valueOf(n);
        else
            f[n] = lookup_fibo(f, n - 1).add(lookup_fibo(f, n - 2));

        return f[n];
    }
}
