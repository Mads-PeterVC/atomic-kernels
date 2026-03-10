from atomic_kernels import sum_as_string

def test1():
    output = sum_as_string(1, 2)
    assert output == "3"

if __name__ == "__main__":
    test1()
    print("All tests passed.")


