# Valid Square

## Description

Given the coordinates of four points in 2D space, return whether the four points could construct a square.

The coordinate (x,y) of a point is represented by an integer array with two integers.

## Constraints

- All the input integers are in the range [-10000, 10000].
- A valid square has four equal sides with positive length and four equal angles (90-degree angles).
- Input points have no order.

## Examples

### Example 1

```
Input: p1 = [0,0], p2 = [1,1], p3 = [1,0], p4 = [0,1]
Output: True
```

### Example 2

```
Input: p1 = [1,1], p2 = [0,1], p3 = [1,2], p4 = [0,0]
Output: False
```