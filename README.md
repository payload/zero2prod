# How to handle

* run a postgres, maybe from docker UI
* use `cargo test` to check if its compiling and running
* use `cargo run | bunyan -o short` to actually run it and have readable logs
* run requests with curl:
  ```bash
  while sleep 3; do curl -X POST 'http://127.0.0.1:3000/subscriptions' -d 'name=DenverCoder9&email=funny%40valen.tine' --verbose; done
  ```

# Ideas for future

## UUIDs are difficult to read

* I found out about [ulid](https://github.com/ulid/spec) as an UUID alternative and want to use that probably.
  But I may want to read about IDs in Postgresql too.
  There are also [new UUID versions](https://uuid6.github.io/uuid6-ietf-draft/) floating around.
