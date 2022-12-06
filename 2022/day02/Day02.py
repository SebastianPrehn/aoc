import numpy as np

## Part 1 ##
# Create an array with all calculated values
# elfV you> rock  | paper | scissors  #
# rock    ( 1+3   , 2 + 6 , 3 + 0   ) #
# paper   ( 1 + 0 , 2 + 3 , 3 + 6   ) #
# scissors( 1 + 6 , 2 + 0 , 3 + 3   ) #
rps = np.array([[4,8,3], [1,5,9], [7,2,6]])
elf_table = {'A': 0, 'B': 1, 'C': 2}
player_table = {'X': 0, 'Y': 1, 'Z': 2}

def strategy(input_file):
    player = 0
    with open(input_file) as input:
        for line in input:
            move = line.split() 
            player += rps[elf_table[move[0]],player_table[move[1]]]
        return player

print(f"Input sample result (part 1): {strategy('inputSample.txt')}")
print(f"Input result (part 1): {strategy('input.txt')}")

## Part 2 ##
# Create an array with all calculated values
# elfV cond> lose | draw  | win       #
# rock    ( 3     , 1 + 3 , 2 + 6   ) #
# paper   ( 1     , 2 + 3 , 3 + 6   ) #
# scissors( 2 + 0 , 3 + 3 , 1 + 6   ) #
rps_cond = np.array([[3,4,8], [1,5,9], [2,6,7]])

def strategy_cond(input_file):
    player = 0
    with open(input_file) as input:
        for line in input:
            move = line.split()
            player += rps_cond[elf_table[move[0]],player_table[move[1]]]
            # print(f"Move: {move} and score: {rps_cond[elf_table[move[0]],player_table[move[1]]]}")
        return player

print(f"Input sample result (part 2): {strategy_cond('inputSample.txt')}")
print(f"Input result (part 2): {strategy_cond('input.txt')}")
