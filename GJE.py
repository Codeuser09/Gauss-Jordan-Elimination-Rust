#Python equivalent the Rust version was transcoded from by me

import numpy as np

M = np.array([[8, -8, -9, -1], [-10, 15, -9, -25], [-9, -1, 7, 3]], float)

if M[0][0] == 0:
    M[0] += 1

for j in range(len(M)):  # For each column
    if M[j][j] != 1:  # If the jth element of that column is not 1
        M[j] = M[j] / M[j][j]  # Then divide the row by that element

    for i in range(len(M)):  # For each row
        if i != j:  # If we are not at the row we want to have the pivot 1
            M[i] -= M[j] * M[i][j]

np.set_printoptions(precision=20)
display(M[:, -1])
