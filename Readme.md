## Description

[Link to spec](https://hackmd.io/@r3XngjBBSumx2rU-hKU7Qg/BkbHS80cv)

To start one instance run 

```
RUST_LOG=info cargo run -- 5
```

* The first command line argument (after `--`) is ping interval in seconds 
* Peers are started on random ports and discovered via mdns (i.e. it autodetects each peer on the LAN)
* The protocol is to send `ping` message and receive `pong` in return

## Example logs

```
[2020-12-04T21:44:50Z INFO  p2p_test::network] Starting p2p network
[2020-12-04T21:44:50Z INFO  p2p_test::network] Network started
[2020-12-04T21:44:50Z INFO  p2p_test::network::core_behaviour] Sent Pong to 16Uiu2HAmCiYmZcVkFjfYXrYNJQXApR1MKnFdDwzsetwA34wdX3Xo
[2020-12-04T21:44:50Z INFO  p2p_test::network::core_behaviour] Ping returned from 16Uiu2HAmCiYmZcVkFjfYXrYNJQXApR1MKnFdDwzsetwA34wdX3Xo with rtt 647.866µs
[2020-12-04T21:44:53Z INFO  p2p_test::network::core_behaviour] Sent Pong to 16Uiu2HAmCiYmZcVkFjfYXrYNJQXApR1MKnFdDwzsetwA34wdX3Xo
[2020-12-04T21:44:55Z INFO  p2p_test::network::core_behaviour] Ping returned from 16Uiu2HAmCiYmZcVkFjfYXrYNJQXApR1MKnFdDwzsetwA34wdX3Xo with rtt 841.998µs
[2020-12-04T21:44:56Z INFO  p2p_test::network::core_behaviour] Sent Pong to 16Uiu2HAmCiYmZcVkFjfYXrYNJQXApR1MKnFdDwzsetwA34wdX3Xo
[2020-12-04T21:44:59Z INFO  p2p_test::network::core_behaviour] Sent Pong to 16Uiu2HAmCiYmZcVkFjfYXrYNJQXApR1MKnFdDwzsetwA34wdX3Xo
[2020-12-04T21:45:00Z INFO  p2p_test::network::core_behaviour] Ping returned from 16Uiu2HAmCiYmZcVkFjfYXrYNJQXApR1MKnFdDwzsetwA34wdX3Xo with rtt 2.341905ms
```
