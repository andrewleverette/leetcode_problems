# Task Scheduler

## Description

You are given a char array representing tasks CPU need to do. It contains capital letters A to Z where each letter represents a different task. Tasks could be done without the original order of the array. Each task is done in one unit of time. For each unit of time, the CPU could complete either one task or just be idle.

However, there is a non-negative integer n that represents the cooldown period between two same tasks (the same letter in the array), that is that there must be at least n units of time between any two same tasks.

You need to return the least number of units of times that the CPU will take to finish all the given tasks.

Constraints:

The number of tasks is in the range [1, 10000].
The integer n is in the range [0, 100].

## Examples

### Example 1

```
Input: tasks = ["A","A","A","B","B","B"], n = 2
Output: 8
Explanation: 
A -> B -> idle -> A -> B -> idle -> A -> B
There is at least 2 units of time between any two same tasks.
```

### Example 2

```
Input: tasks = ["A","A","A","B","B","B"], n = 0
Output: 6
Explanation: On this case any permutation of size 6 would work since n = 0.
["A","A","A","B","B","B"]
["A","B","A","B","A","B"]
["B","B","B","A","A","A"]
...
And so on.
```

### Example 3

```
Input: tasks = ["A","A","A","A","A","A","B","C","D","E","F","G"], n = 2
Output: 16
Explanation: 
One possible solution is
A -> B -> C -> A -> D -> E -> A -> F -> G -> A -> idle -> idle -> A -> idle -> idle -> A
```

## Notes

Input: 
    - tasks = ["A","A","A","B","B","B"]
    - n = 2

Steps

- find the max frequency and the number of idle spaces
    * => A _ _ A _ _ A
    * => max frequency = highest freq - 1 = 2
    * => idle time needed = max frequency * n = 2 * 2 = 4
- fill in spaces with the remaining frequencies
    * B would take up 2 of the idle spaces
    * => idle time - 2 = 2
- interval is the remaining idle time + the number of tasks in list if idle time is greater than zero, otherwise return the number of tasks