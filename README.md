`twitch-online` checks if twitch streamers are currently online.
It has minimal TUI. It's also possible to play the stream via [streamlink](https://github.com/streamlink/streamlink).

![image](https://user-images.githubusercontent.com/6440135/200137559-79530e6e-66e6-4d4a-9f22-abb01e503ecb.png)

## Requirements

- `curl`
- (optional) [streamlink](https://github.com/streamlink/streamlink).

## Config file

Configuration should be placed in `~/.config/twitch-online/config.toml`.

Example:

```toml
# The list of twitch streamers. Each element represents streamer's id from twitch
# page url. For example if streamer's page is `https://www.twitch.tv/streamer_1`
# then you should add `streamer_1` to the list
streamers = [
  "streamer_1",
  "streamer_2",
]

# How many threads will be used at the same time to fetch streamers status
# in parallel.
# If not specified the number of threads will be equal to the number of
# streamers (Default).
threads_num = 5
```
