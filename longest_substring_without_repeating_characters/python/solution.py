def length_of_longest_substring_sliding_window_optimized(s: str) -> int:
    """
    Returns the length of the longest substring without repeating characters.
    
    Arguments:
    
    * `s` - The input string
    
    Approach:
    
    This solution uses a sliding window approach which keeps track of the
    starting and ending positions of the current substring. For each iteration,
    the current character is checked against the characters seen so far.
    If that character has already been seen, then slide the starting position
    forward to the ending position of the current substring. This solution works
    if you can make safe assumptions about the characters set. In this case I assume
    the character set is standard ASCII.
    """
    char_set = [0] * 128
    length = 0

    start = 0
    for end, char in enumerate(s):
        start = max(char_set[ord(char)], start)
        length = max(length, end - start + 1)
        char_set[ord(char)] = end + 1

    return length


def test_example_1():
    assert length_of_longest_substring_sliding_window_optimized("abcabcbb") == 3


def test_example_2():
    assert length_of_longest_substring_sliding_window_optimized("bbbbb") == 1


def test_example_3():
    assert length_of_longest_substring_sliding_window_optimized("pwwkew") == 3
