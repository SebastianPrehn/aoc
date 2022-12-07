""" Advent of Code 2022. Day 4: Camp Cleanup """

import re

with open("sample.txt") as sample:
    example = sample.read().splitlines()

with open("input.txt") as input:
    pair_list = input.read().splitlines()

## part 1 and part 2 ##
def day04(data):
    full_contains = 0
    overlaps = 0
    for line in data:
        elf1_start, elf1_end, elf2_start, elf2_end = [int(sections) for sections in re.findall(r'(\d+)-(\d+),(\d+)-(\d+)',line)[0]]
        if (elf1_start - elf2_start) * (elf1_end - elf2_end) <= 0:
            full_contains += 1
        if (elf2_end - elf1_start) * (elf2_start - elf1_end) <= 0:
            overlaps += 1
    print(full_contains)
    print(overlaps)
    
day04(example)
day04(pair_list)
