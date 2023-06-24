import random
import rust_py
from time import *

N = 256

A = [[random.randint(1, 100) for _ in range(N)] for _ in range(N)]
B = [[random.randint(1, 100) for _ in range(N)] for _ in range(N)]

start = time()
rust_py.matmul(A, B)
end = time()

print(f"Time taken: {end - start}")