# Takeaways

* Python code can technically work from Rust without modification
* Using Rust together with Python can be a good way to improve performance compared to pure python
* Using multiple processes, each owning their own GIL, is the way to go if you want performance. This might require a message broker.
* I want to look at more real-world use-cases of this type of system and interaction more.


## Pure Python
Each batch of 8 jobs runs for little over 5 seconds.

```bash
$> python .\test_python_code.py
<SNIP>
PYTH b34d2fc3-46e1-4302-8327-c76ed38ea7ed found 2750000 so far in 23571428
7cfad1c1-58b7-4dd6-b02a-d4d00bf667f2 finished in 5.118744611740112 seconds
dd99f7da-4f8b-4b7d-899f-2101cb7026e6 finished in 5.113743305206299 seconds
54f89266-7ad9-43fd-96d4-764d160f1988 finished in 5.131748199462891 seconds
82046bb1-02d8-40e8-bbcb-9024feafc98d finished in 5.131748676300049 seconds
f82c7799-05b5-4157-937c-5d8c8a0f7747 finished in 5.179760217666626 seconds
386736e9-0b83-40f0-8a5c-c27c1427dc17 finished in 5.19476580619812 seconds
4147e76e-3ee6-4bd5-831f-31f54b9591cd finished in 5.2277748584747314 seconds
b34d2fc3-46e1-4302-8327-c76ed38ea7ed finished in 5.26878547668457 seconds
Spawned: 32a10631-f6a3-4f08-a5d5-c56d0f86cb9b
<SNIP>
PYTH 5e1cff70-32bb-41fc-9fb4-b32227112e3d found 2750000 so far in 23571426
PYTH 5e1cff70-32bb-41fc-9fb4-b32227112e3d found 2750000 so far in 23571428
90ffd3e8-d1ff-4cac-a186-6c7d4c536514 finished in 5.069124937057495 seconds
5ce45f60-f75a-45c9-9781-85899b87c71a finished in 5.0681257247924805 seconds
10c2b6e9-07d1-4fa0-af36-4bb98e0ee590 finished in 5.105136394500732 seconds
1b4c6b30-54f3-409f-8fcd-886ecc6a1dea finished in 5.138144254684448 seconds
f718409e-ebb9-45c8-b1b6-511c2dcc34fc finished in 5.155148506164551 seconds
862d5f92-0d40-4d41-8e2f-d2385fc462ed finished in 5.164150953292847 seconds
32a10631-f6a3-4f08-a5d5-c56d0f86cb9b finished in 5.208162546157837 seconds
5e1cff70-32bb-41fc-9fb4-b32227112e3d finished in 5.218165874481201 seconds
Finished in: 10.736017942428589
```

## Rust Python hybrid
```bash
$> .\target\release\rustpython-mp-testing.exe broker
<SNIP>
RUST aacfbf8d-78d5-42f8-8955-b78b09a849aa found 2500000 so far in 21428576
Finished in: 0.75246800
Finished in: 0.72855200
Worker result: 2916667
Worker result: 2916667
RUST aacfbf8d-78d5-42f8-8955-b78b09a849aa found 2750000 so far in 23571425
RUST aacfbf8d-78d5-42f8-8955-b78b09a849aa found 2750000 so far in 23571426
RUST aacfbf8d-78d5-42f8-8955-b78b09a849aa found 2750000 so far in 23571428
Finished in: 0.82742900
Worker result: 2916667
<SNIP>
PYTH d2eb5abe-f2fc-4933-9e94-393212ead38a found 2750000 so far in 23571428
aede500a-f5eb-43d8-b761-cee1ff1b1577 finished in 5.192108392715454 seconds
Worker result: 2916667
Finished with status: exit code: 0
96dc5820-6924-45a4-8bef-e66691497d38 finished in 5.2351202964782715 seconds
Worker result: 2916667
Finished with status: exit code: 0
190b6246-45f2-4056-b407-13e850f4cdc0 finished in 5.239121675491333 seconds
Worker result: 2916667
Finished with status: exit code: 0
bb68d29e-c5b2-45f4-a698-8c9a90c37924 finished in 5.260127782821655 seconds
Worker result: 2916667
Finished with status: exit code: 0
d695785b-78f2-47e6-a85d-3a692fa79664 finished in 5.300137758255005 seconds
Worker result: 2916667
Finished with status: exit code: 0
fc66c61a-100c-4067-b95b-f788241a9b93 finished in 5.3061394691467285 seconds
Worker result: 2916667
958e0082-4deb-41ed-bbc0-986362ece537 finished in 5.285135269165039 seconds
Finished with status: exit code: 0
Worker result: 2916667
Finished with status: exit code: 0
d2eb5abe-f2fc-4933-9e94-393212ead38a finished in 5.323143720626831 seconds
Worker result: 2916667
Finished with status: exit code: 0
```

## Notes

The Rust version ran the Python code ~ 0.1 - 0.2 seconds slower; however, the same code written in Rust ran in ~0.75 seconds, which is over 5 times faster than the Python code. This means that the speed increase from Rust more than compensates for the extra overhead when runnig Python from Rust. Additonal research is needed to determine the performance considerations in other use cases such as IO-bounded tasks.