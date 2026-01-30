const left 0b00000000
const out 0b10000110
const move 0b00000001
const input_to_reg_3 0b10110011
const if_wall 0b11000111
const if_always_move 0b11000100
const reg_0_to_reg_3 =  0b10000011
const right = 0b00000010

const start_move = 0b00000000
const start_left = 7 # 0b00000111
const start_right = 16 # 0b00011011
 

# --------------
# start_move 0




# init counter
0b00000001 # reg_0=1
0b10000010 # reg_2=1

# add 1
0b00000001 # reg_0=1
0b10000001 # reg_1=1
0b01000100 # reg_3 = reg_1 + reg_2
0b10011010 # copy reg_3 to reg_2








