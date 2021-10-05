## Rust implementation of Quick find/Quick union algorithm
The binary starts a REPL which takes 2 unsigned integers as inputs and performs a quick union
over them. The result is stored in an array indexed `0..N-1`. Weighted quick union provides a way
to avoid very tall trees, and thus if we are performing a find operation, then we can expect better than
O(N) time complexity. The code is self-explanatory.