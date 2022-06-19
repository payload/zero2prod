# How to handle

* run a postgres, maybe from docker UI
* use `cargo test` to check if its compiling and running
* use `cargo run | bunyan -o short` to actually run it and have readable logs
* run requests with curl:
  ```bash
  while sleep 3; do curl -X POST 'http://127.0.0.1:3000/subscriptions' -d 'name=DenverCoder9&email=funny%40valen.tine' --verbose; done
  ```
