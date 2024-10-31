# Benchmark results

## Format:

*commit* - *"custom name"* - *times*

## Results:
*2c401bd* - **"Sand and Water - movement per particle update"** - [442.14 ms 449.21 ms 456.66 ms]
*08c7b1e* - **"Sand and Water - movement with randomness"** - [464.11 ms 469.47 ms 475.24 ms]
*329a346* - **"Sand and Water - all movement done by particles"** - [526.29 ms 540.01 ms 554.38 ms] [423.45 ms 424.64 ms 425.86 ms]
*b069087* - **"All particles with movements done by particle"** - [495.09 ms 498.12 ms 501.32 ms]
*75571d0* - **"Removed unnecesery checks in simulation"** - [459.52 ms 462.61 ms 465.86 ms]
*e36437c* - **"Minimize use of the rand crate"** - [416.24 ms 418.35 ms 420.73 ms] [433.62 ms 441.95 ms 451.43 ms] [424.03 ms 428.95 ms 434.15 ms]
