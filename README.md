`twitch_online` checks if twitch users are currently streaming. It has minimal TUI. It's also possible to play the
stream. For this [streamlink](https://github.com/streamlink/streamlink) should be installed on the system.

The list of streamers will be taken from `~/.config/twitch_online/users` which is a simple file each
line of which should hold streamer's name from twitch page url. For example if streamer's page is
`https://www.twitch.tv/foobar` then you should put `foobar` in `~/.config/twitch_online/users`:

```
foobar
foobar2
```
