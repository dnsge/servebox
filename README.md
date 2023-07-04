# servebox

A simple, lightly-configurable file server box.

## Serving Files

Create a `config.toml` file (or specify the path to the config file in `SERVEBOX_CONFIG_FILE`):

```toml
# Configure HTTP server
bind = "0.0.0.0:8080"
workers = 2

[[serve]]
web_path = "/"                  # Web path prefix
file_path = "./web_files/site1" # Path to files to serve
index_file = "index.html"       # Optional, file name to show on index requests
show_index = false              # Optional, display index of files in directories
host = "site1.example.com"      # Optional, restrict to Host header

# Specify multiple handlers
[[serve]]
web_path = "/"
file_path = "./web_files/site2"
index_file = "index.html"
show_index = false
host = "site2.example.com"
```

## Docker 

```bash
$ docker run \
    -v ./config:/config:ro \
    -v ./files:/files:ro \
    -p 8080:8080 \
    ghcr.io/dnsge/servebox:master
```
