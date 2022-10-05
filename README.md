## Quickjs function execution

This project is a test project to integrate functions in a quickjs runtime in Rust and execute them based on payloads.

Mian objective : Be able to inject functions and ran thoses functions on arbitrary payload while keeping good performances.


## Bench

Current benchmark is only run from main a fixed number of execution using the same function_id and a dynamix string as pyaload a fixed number of time and compute the time spent.
This should improved in the future if I rework on this.

| Number of elements | Time |
| --- | --- |
| 1 |  138μs|
| 10 | 840μs |
| 100 | 7349μs |
| 1000 | 63473μs |
| 10000 | 602671μs |