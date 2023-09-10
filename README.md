Performance issue with async-graphql dataloader

To reproduce:

- Clone repository
- cargo run (--release)

Benchmark against the **non**-dataloaded query with:
`{books{id,name,published,author{id,name}}}`

Autocannon command: `autocannon http://127.0.0.1:4000/graphql -m POST -H "Content-Type: application/json" -b "{\"query\": \"{books{id,name,published,author{id,name}}}\"}" -c 5 --warmup [ -c 1 -d 3 ]`

Dataloaded query: `{books{id,name,published,authorDataloaded{id,name}}}`

Autocannon command: `autocannon http://127.0.0.1:4000/graphql -m POST -H "Content-Type: application/json" -b "{\"query\": \"{books{id,name,published,authorDataloaded{id,name}}}\"}" -c 5 --warmup [ -c 1 -d 3 ]`

Results from my M1 Macbook Pro:

**Non**-dataloaded:

```
┌─────────┬──────┬──────┬───────┬──────┬─────────┬─────────┬──────┐
│ Stat    │ 2.5% │ 50%  │ 97.5% │ 99%  │ Avg     │ Stdev   │ Max  │
├─────────┼──────┼──────┼───────┼──────┼─────────┼─────────┼──────┤
│ Latency │ 0 ms │ 0 ms │ 0 ms  │ 0 ms │ 0.01 ms │ 0.01 ms │ 2 ms │
└─────────┴──────┴──────┴───────┴──────┴─────────┴─────────┴──────┘
┌───────────┬─────────┬─────────┬─────────┬───────┬──────────┬─────────┬─────────┐
│ Stat      │ 1%      │ 2.5%    │ 50%     │ 97.5% │ Avg      │ Stdev   │ Min     │
├───────────┼─────────┼─────────┼─────────┼───────┼──────────┼─────────┼─────────┤
│ Req/Sec   │ 40607   │ 40607   │ 52479   │ 52767 │ 50802.91 │ 3660.42 │ 40584   │
├───────────┼─────────┼─────────┼─────────┼───────┼──────────┼─────────┼─────────┤
│ Bytes/Sec │ 25.4 MB │ 25.4 MB │ 32.8 MB │ 33 MB │ 31.8 MB  │ 2.29 MB │ 25.4 MB │
└───────────┴─────────┴─────────┴─────────┴───────┴──────────┴─────────┴─────────┘

Req/Bytes counts sampled once per second.
# of samples: 11

559k requests in 11s, 349 MB read
```

With dataloader:

```
┌─────────┬──────┬──────┬───────┬──────┬─────────┬───────┬───────┐
│ Stat    │ 2.5% │ 50%  │ 97.5% │ 99%  │ Avg     │ Stdev │ Max   │
├─────────┼──────┼──────┼───────┼──────┼─────────┼───────┼───────┤
│ Latency │ 1 ms │ 1 ms │ 4 ms  │ 5 ms │ 1.52 ms │ 1 ms  │ 32 ms │
└─────────┴──────┴──────┴───────┴──────┴─────────┴───────┴───────┘
┌───────────┬─────────┬─────────┬─────────┬─────────┬─────────┬────────┬─────────┐
│ Stat      │ 1%      │ 2.5%    │ 50%     │ 97.5%   │ Avg     │ Stdev  │ Min     │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼────────┼─────────┤
│ Req/Sec   │ 1952    │ 1952    │ 2467    │ 2563    │ 2384.1  │ 197.2  │ 1952    │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼────────┼─────────┤
│ Bytes/Sec │ 1.32 MB │ 1.32 MB │ 1.67 MB │ 1.73 MB │ 1.61 MB │ 133 kB │ 1.32 MB │
└───────────┴─────────┴─────────┴─────────┴─────────┴─────────┴────────┴─────────┘

Req/Bytes counts sampled once per second.
# of samples: 10

24k requests in 10.01s, 16.1 MB read
```

With adding a 0 delay:
https://docs.rs/async-graphql/latest/async_graphql/dataloader/struct.DataLoader.html#method.delay

```
autocannon http://127.0.0.1:4000/graphql -m POST -H "Content-Type: application/json" -b "{\"query\": \"{books{id,name,published,authorDataloaded{id,name}}}\"}" -c 5 --warmup [ -c 1 -d 3 ]
Running 3s warmup @ http://127.0.0.1:4000/graphql
1 connections

Running 10s test @ http://127.0.0.1:4000/graphql
5 connections


┌─────────┬──────┬──────┬───────┬──────┬─────────┬─────────┬──────┐
│ Stat    │ 2.5% │ 50%  │ 97.5% │ 99%  │ Avg     │ Stdev   │ Max  │
├─────────┼──────┼──────┼───────┼──────┼─────────┼─────────┼──────┤
│ Latency │ 0 ms │ 0 ms │ 0 ms  │ 0 ms │ 0.01 ms │ 0.01 ms │ 3 ms │
└─────────┴──────┴──────┴───────┴──────┴─────────┴─────────┴──────┘
┌───────────┬─────────┬─────────┬─────────┬─────────┬──────────┬────────┬─────────┐
│ Stat      │ 1%      │ 2.5%    │ 50%     │ 97.5%   │ Avg      │ Stdev  │ Min     │
├───────────┼─────────┼─────────┼─────────┼─────────┼──────────┼────────┼─────────┤
│ Req/Sec   │ 33215   │ 33215   │ 35999   │ 36319   │ 35547.64 │ 949.31 │ 33188   │
├───────────┼─────────┼─────────┼─────────┼─────────┼──────────┼────────┼─────────┤
│ Bytes/Sec │ 22.4 MB │ 22.4 MB │ 24.3 MB │ 24.5 MB │ 24 MB    │ 644 kB │ 22.4 MB │
└───────────┴─────────┴─────────┴─────────┴─────────┴──────────┴────────┴─────────┘

Req/Bytes counts sampled once per second.
# of samples: 11

391k requests in 11s, 264 MB read
```
