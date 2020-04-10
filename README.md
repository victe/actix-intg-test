# Example setup for integration testing of actix-web servers 

Run with

```shell script
cargo test -- --nocapture
```

Needed improvements:

- Force Kill spawned process before exit.
- Check and assign only free ports.

