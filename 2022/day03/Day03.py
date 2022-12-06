
alphabet = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'

def reorganize(input_file):
    with open(input_file) as input:
        char_list = []
        for line in input:
            a = line[:len(line)//2]
            b = line[len(line)//2:]
            for element in a:
                if element in b:
                    char_list.append(element)
                    break
        return print(sum([alphabet.find (elm) + 1 for elm in char_list]))


## Part 2 ##
# Here, I looked at https://github.com/hildenost/advent-of-code/blob/main/2022/03/rucksack.py
# for inspiration, as I was unaware Python was able to do *rest and discovered a few other commands
# like intersection and pop, which I will hopefully find useful in the coming days.
# Check Hilde's solutions out, as they're miles better than my own!
def find_badge(input_file):
    with open(input_file) as input:
        input = input.read().splitlines()
        badge_list = []
        for elm in range(0, len(input), 3):
            first, *rest = [set(s) for s in input[elm:elm+3]]
            badge_list.append(first.intersection(*rest).pop())
        return print(sum([alphabet.find(badge) + 1 for badge in badge_list]))

reorganize('inputSample.txt')
reorganize('input.txt')
find_badge('input.txt')
