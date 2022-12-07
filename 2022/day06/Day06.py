""" Advent of Code 2022. Day 6: Tuning Trouble """

with open("input.txt") as input:
    datastream = input.read()

def find_signal(sequence):
    for char in range(len(datastream)):
        marker = datastream[char:char+sequence]
        if len(set(marker)) == sequence:
            return char + sequence

print(find_signal(4))
print(find_signal(14))
