# Intro

This repository contains code to simulate the effects of changing the minimum order age parameter.

It runs on historical data extracted from the database and makes the following simplifying assumptions:
- only consider orders that got settled
- orders become settlable at their creation time and stay settlable forever
- driver run loop takes a constant amount of time when not settling (5s) and settling (45s)
- all solvable orders can be settled in the same solution (perfect solving / merging)

Based on this this we emulate the behavior of the real driver. It starts at the time of the first order and performs run loops. When a run loop finds at least one order with the minimum age then all orders (even the younger ones) are settled. Depending on the minimum order age these settlements contain on average more or less orders.

# Running

Get from the database all orders that were settled in the last 7 days and save them into a local file:

```
psql <parameters from 1password> -c "
SELECT uid, EXTRACT(EPOCH FROM creation_timestamp)
FROM orders o
WHERE
    (o.creation_timestamp BETWEEN now()::timestamp - (interval '7d') AND now()::timestamp) AND
    (SELECT COUNT(*) FROM trades t WHERE t.order_uid = o.uid) > 0
ORDER BY creation_timestamp asc
" -t -A -F"," > out.csv
```

Compile the rust binary:

```
rustc analyze.rs
```

Run the binary on the data:

```
cat out.csv | ./analyze
```
