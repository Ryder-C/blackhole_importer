A TUI for importing magnet links into your torrent blackhole.
![241107_10h27m58s_screenshot](https://github.com/user-attachments/assets/4504b4ca-1573-4d17-9552-7e192e653fcc)

```
Usage: blackhole_importer [OPTIONS] --magnet-link <MAGNET_LINK>

Options:
  -m, --magnet-link <MAGNET_LINK>  The magnet link
  -o, --output <OUTPUT>            Output file name
  -h, --help                       Print help
  -V, --version                    Print version
```

Example config file:
```toml
[[instance]]
name="sonarr"
path="/storage/symlinks/blackhole/sonarr"

[[instance]]
name="sonarr 4k"
path="/storage/symlinks/blackhole/sonarr 4k"

[[instance]]
name="sonarr anime"
path="/storage/symlinks/blackhole/sonarr anime"

[[instance]]
name="radarr"
path="/storage/symlinks/blackhole/radarr"

[[instance]]
name="radarr 4k"
path="/storage/symlinks/blackhole/radarr 4k"

[[instance]]
name="radarr anime"
path="/storage/symlinks/blackhole/radarr anime"
```
