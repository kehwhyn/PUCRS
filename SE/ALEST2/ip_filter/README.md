# The IP Filter Filter

You have just been found by a security company that develops software to clean up IP filters in large corporations. When a firewall is installed, it receives a list of IP addresses that must be blocked because they have been identified as harmful. Over time, new addresses are added or changed, and from time to time, a cleanup is necessary. The list is simple—it contains pairs of addresses (between 0 and 2^31), and **all** addresses between them must be blocked, including the addresses that were added. The first value is the **lowest** prohibited IP number, followed by the **highest** prohibited IP number. The two addresses are separated by a dash ("-").

```
793359858-872845434
163828202-386942572
860031788-1045135809
220012220-481960112
107202696-258581056
77865186-199064861
207437354-311919151
125701728-213021371
851097643-935519522
613234685-874907996
432569835-566753675
45389652-232145031
896106680-1011116869
272672436-491730771
354054836-415525648
436958726-539840832
861203282-877295410
837159809-839258919
273032866-505234870
135087090-305043137
```

So the question that must be answered, given the list of blocked addresses, is:
> What is the length of the **smallest possible list** that blocks the **same addresses**?

Solving the problem using the table above would result in the following list:

```
45389652-566753675
613234685-1045135809
```

which has a length of 2. You must write a program to read lists of addresses and determine the length of the smallest possible list, then test it with the files provided on the course page and submit a report including:

* What problem is being solved;
* How the problem was modeled;
* What the solution process is like, with examples and algorithms;
* The results of the test cases;
* Conclusions.
