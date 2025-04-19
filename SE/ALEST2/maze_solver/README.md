# From A to B is Already Quite Far

A superhero franchise film studio decided to innovate: in their next movie, there will be a **gigantic labyrinth** where heroes and villains will have to find each other and fight until… well, until the movie ends. That’s the entire script.

Now the studio is concerned about the design of the labyrinth, and since you’re one of the **three world experts in movie labyrinths**, you’ve just been hired to conduct some preliminary studies. Basically, your task begins when you receive labyrinth blueprints containing the following symbols:

```
# represents an impenetrable wall;
. represents free space;
A represents a generic hero who can move up, down, left, and right (but not diagonally);
B represents a generic villain who is standing still making his evil plans.
```

An example labyrinth is shown below. After receiving the data, you must determine the **length of the shortest path** from the hero to the villain. This is an estimate of how long it will take the hero to reach the villain and is important for balancing the chase and the deadly combat phases. In this example, we found that the hero needs **at least 49 moves** to reach the villain.

```
########################################
#..#.....#..........#........#....#....#
#..#.....#...................#....#....#
#........#....#.....#........#....#....#
#........#....#.....####..####.........#
#..#.....##.######.##........#....#....#
#.###.####...#......#........#.........#
#........#...#......#........#....#....#
#............#......#........#....#....#
#..#.....#...#......#........########.##
#..#.....#..........#........#..#......#
#..#................#........#..#......#
#..#.....#...#......##..#####...#......#
###############.##.##........#.......B.#
#............#......#...#....#.........#
#...A........#......#...#....#..#......#
#......#.....##..####........#..#......#
#......#.....#......#...#....#..#......#
###..#########......####.#####.#########
#............#......#..........#.......#
#....#.......#......#......#...........#
#....#.........#.####......#...#.......#
#....#.......#.............#.....###.###
#....#..............#..........#...#...#
#....#.......#......#.###.##...#...#...#
#....#.......#.##.###......#...#.......#
#....#.......#......#......#...#.......#
#............#......#......#...#...#...#
#....#.......#......#......#...#...#...#
########################################
```

Of course, life in Hollywood is never that simple, and after being hired, you discover that the studio is planning **mazes up to 20,000 × 20,000 in size**. So now, there's no turning back.

Your task is to receive the test cases (which the studio will upload to the course page) and solve the problem for each of them. In the end, you must present a report that describes:

* What problem is being solved;
* How the problem was modeled;
* What the solution process is like, presenting examples and algorithms;
* The results of the test cases;
* Conclusions.
