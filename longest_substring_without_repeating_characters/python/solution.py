def length_of_longest_substring_sliding_window_optimized(s: str) -> int:
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
