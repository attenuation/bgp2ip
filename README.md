# bgp2ip
bgp data to ip list filter by isp

```
bash prepare
cargo build --release
./target/release/bgp2ip asnames.txt rib.txt
```