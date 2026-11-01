zLog Package Central
====

![badge](https://github.com/nextzlog/pkg.zlog.org/actions/workflows/build.yaml/badge.svg)

The official package repository (plugin marketplace) for [zLog 令和 edition](https://use.zlog.org) and [zLog Max](https://max.zlog.org).
zLog is a powerful amateur radio logging software for contests and has been loved by millions for 30 years.

## Publish

First, create a TOML file and include the download URL and md5 checksum in the dll table.
You can include the checksum directly or include the URL of the md5 file.

```toml
# dll.<name>
[dll.sample]
url = "https://example.com/releases/sample.dll"
sum = "https://example.com/releases/sample.dll.md5"
```

You can also publish other files along with the DLL, such as CFG and DAT files.

```toml
# cfg.<name>
[cfg.sample]
url = "https://example.com/releases/sample.cfg"

# dat.<name>
[dat.sample]
url = "https://example.com/releases/sample.dat"
```

Finally, write the package information, which will be displayed in the plugin manager.

```toml
# pkg.<name>
[pkg.sample]
tag = "title"
msg = "description"
web = "https://example.com/sample/index.html"
doc = "https://example.com/sample/details.md"
use = ["cfg.sample", "dat.sample", "dll.sample"] # dependency
exp = "unstable" # or "stable"
```

Publish the TOML file to a Git repository, and make an issue at [nextzlog/todo](https://github.com/nextzlog/todo) to request crawling.

## Crawling

The crawled TOML files are listed in [market.list](src/market.list), checked against [schema.yaml](src/schema.yaml) and merged into `market.json` once a day.

## Contribution

Feel free to make issues at [nextzlog/todo](https://github.com/nextzlog/todo).
Follow [@nextzlog](https://twitter.com/nextzlog) on Twitter.

## License

### Author

[無線部開発班](https://nextzlog.dev)

- JS2GGD
- JR8PPG
- JG1VPP
- JS2FVO

### Clauses

[MIT License](LICENSE)
