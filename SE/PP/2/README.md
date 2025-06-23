# Assignment 2: Polynomial Evaluation (MPI)

The objective of this assignment is to implement, using the **MPI (Message Passing Interface)** standard, a parallel version of a program that evaluates a set of values for a high-degree polynomial, comparing the execution of this parallel version with that of the sequential version.

The sequential version, written in C (using MPI with a single process), evaluates the polynomial for a set of values, where the polynomial has a degree of **400 (`GRAU`)**. The code is provided at the end of this description. The following constants define the number of values to be evaluated:
- `TAM_INI` (initial set size),
- `TAM_INC` (increment to the next set size),
- `TAM_MAX` (final set size).

The sequential version uses MPI calls to measure execution time and OpenMP primitives only to speed up initialization and result verification. Thus, the sequential version must be compiled with:

`mpicc Sequencial.c -o Sequencial -fopenmp`

The parallel MPI version must follow the **master-slave** pattern. The master holds the set of values to be evaluated and the polynomial coefficients. Initially, it broadcasts the coefficients to the slaves and then distributes subsets of values to be evaluated. The slaves continuously receive sets of values, evaluate them for the polynomial, and return the results to the master until they receive a termination signal.

After parallelizing the sequential version with MPI, the program must be executed on the **`grad` cluster** under the following conditions:
- **1 node**: Run with **8 and 16 MPI processes**.
- **2 nodes**: Run with **16 and 32 MPI processes**, evenly distributed between the two nodes.

In all executions, one process will act as the master, and the others will be slaves. Each configuration must be run **at least 3 times**, with the **minimum execution times** recorded for each array size to minimize interference from other cluster processes.

Each student or group (maximum of 2 members) must submit a **PDF report** containing:
- Description of the tests performed,
- Description of the processing nodes,
- Results obtained (graphs showing execution times, speed-up, and efficiency),
- Analysis of the results,
- Conclusions.

### **Evaluation Criteria**
- Notification to the professor of any new group formation within one week of the assignment’s release.
- Execution of the provided sequential version on the `grad` cluster (to enable speed-up and efficiency calculations).
- Implementation of the parallel version in C using MPI (and optionally OpenMP).
- Execution of the parallel version as specified (using 1 and 2 nodes), with at least 3 repetitions per run, considering the **minimum time** for each array size.
- Calculation of **speed-up** and **efficiency** for the parallel executions.
- Presentation of **graphs** for execution time, speed-up, and efficiency, varying the number of evaluated values for the degree-400 polynomial, with curves for each configuration.
- Analysis of **load balancing** in the parallel execution.
- **Code clarity** (use of comments and appropriate variable names).
- **Report in PDF format** (3–4 pages) using the provided Moodle templates, including performance graphs.
- **ZIP file submission** containing:
  - PDF report,
  - Source code,
  - Additional files required for compilation and execution (parallel version source code must be included for verification).
- Submission via **Moodle** by the specified deadline.
- Analysis of the **machine hours** used by the group on the LAD cluster during development.

### **Presentation Requirements**
On the submission day, all group members must attend class and be prepared to:
- Present their work orally, answering questions about its development,
- Complete a written questionnaire about the work,
- Perform a live execution on the `grad` cluster to verify that the results match those in the report.

### **Assignments that will NOT be evaluated:**
- Those submitted by a group **not registered within the deadline**,
- Those **not submitted via Moodle** by the deadline,
- Those **not properly presented**,
- Those **missing the required PDF report**,
- Those **without execution results on the `grad` cluster**,
- Those **without performance graphs**,
- Those **missing the source code** (must compile without errors).
