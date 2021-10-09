from multiprocessing import Pool
import uuid
import time


def test_function() -> int:
    """
    Some random number crunching to make the CPU do work, not an IO test
    """
    start_time = time.time()
    exec_uuid = uuid.uuid4()
    print(f"Spawned: {exec_uuid}")
    num_found = 0
    for num in range(0, 25_000_000):
        if num % 2 == 0 or num % 3 == 0 or num % 5 == 0:
            bitthing = num ^ 0x42
            isthree = bitthing & 3
            if isthree == 3:
                num_found += 1
            if num_found % 250_000 == 0:
                print(f"PYTH {exec_uuid} found {num_found} so far in {num}")
    end_time = time.time()
    delta = end_time - start_time
    print(f"{exec_uuid} finished in {delta} seconds")
    return num_found


if __name__ == '__main__':
    num_workers = 16
    start_time = time.time()

    # run test twice from the top, both in python
    pool = Pool(num_workers)
    for _ in range(num_workers):
        pool.apply_async(test_function, ())
    pool.close()
    pool.join()

    pool = Pool(num_workers)
    for _ in range(num_workers):
        pool.apply_async(test_function, ())
    pool.close()
    pool.join()

    end_time = time.time()
    print(f"Finished in: {end_time - start_time}")