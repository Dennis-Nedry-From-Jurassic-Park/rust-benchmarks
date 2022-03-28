benchmark flat structs results:

opt-level = 0
codegen-units = 2

1) 50 iterations over 1 mln inserts in each

| # |   task   | iterations | total duration | mean duration |
|---|----------|------------|----------------|---------------|
| 1 | smallvec | 50         | 3.246719856s   | 64.934397ms   |
| 2 | tinyvec  | 50         | 3.348513772s   | 66.970275ms   |
| 3 | arrayvec | 50         | 2.031665945s   | 40.633318ms   |

2) 50 iterations over 100 mln inserts in each

| # |   task   | iterations | total duration | mean duration |
|---|----------|------------|----------------|---------------|
| 1 | smallvec | 50         | 552.109450292s | 11.042189005s |
| 2 | tinyvec  | 50         | 401.431720705s | 8.028634414s  |
| 3 | arrayvec | 50         | 249.249487455s | 4.984989749s  |