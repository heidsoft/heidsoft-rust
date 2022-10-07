# 更新依赖
```
rocket_app git:(master) ✗ cargo run
    Updating crates.io index
warning: spurious network error (2 tries remaining): failed to connect to github.com: Operation timed out; class=Os (2)
warning: spurious network error (1 tries remaining): failed to connect to github.com: Operation timed out; class=Os (2)
  Downloaded multer v2.0.4
  Downloaded want v0.3.0
  Downloaded pear v0.2.3
  Downloaded ghash v0.5.0
  Downloaded fastrand v1.8.0
  Downloaded tower-service v0.3.2
  Downloaded aes v0.8.1
  Downloaded toml v0.5.9
  Downloaded devise v0.3.1
  Downloaded opaque-debug v0.3.0
  Downloaded atomic v0.5.1
  Downloaded ubyte v0.10.3
  Downloaded aead v0.5.1
  Downloaded hmac v0.12.1
  Downloaded aes-gcm v0.10.1
  Downloaded hyper v0.14.20
  Downloaded state v0.5.3
  Downloaded yansi v0.5.1
  Downloaded tracing-attributes v0.1.23
  Downloaded stable-pattern v0.1.0
  Downloaded ref-cast v1.0.9
  Downloaded spin v0.9.4
  Downloaded pear_codegen v0.2.3
  Downloaded proc-macro2-diagnostics v0.9.1
  Downloaded rocket_http v0.5.0-rc.2
  Downloaded tempfile v3.3.0
  Downloaded try-lock v0.2.3
  Downloaded rocket v0.5.0-rc.2
  Downloaded ctr v0.9.2
  Downloaded rocket_codegen v0.5.0-rc.2
  Downloaded glob v0.3.0
  Downloaded futures-channel v0.3.24
  Downloaded subtle v2.4.1
  Downloaded futures v0.3.24
  Downloaded futures-io v0.3.24
  Downloaded cipher v0.4.3
  Downloaded unicode-xid v0.2.4
  Downloaded sha2 v0.10.6
  Downloaded universal-hash v0.5.0
  Downloaded inlinable_string v0.1.15
  Downloaded http-body v0.4.5
  Downloaded uncased v0.9.7
  Downloaded polyval v0.6.0
  Downloaded serde_derive v1.0.145
  Downloaded async-trait v0.1.57
  Downloaded inout v0.1.3
  Downloaded binascii v0.1.4
  Downloaded devise_core v0.3.1
  Downloaded remove_dir_all v0.5.3
  Downloaded ref-cast-impl v1.0.9
  Downloaded hkdf v0.12.3
  Downloaded figment v0.10.8
  Downloaded devise_codegen v0.3.1
  Downloaded 53 crates (2.0 MB) in 1.25s
   Compiling version_check v0.9.4
   Compiling libc v0.2.134
   Compiling cfg-if v1.0.0
   Compiling autocfg v1.1.0
   Compiling proc-macro2 v1.0.46
   Compiling quote v1.0.21
   Compiling unicode-ident v1.0.4
   Compiling syn v1.0.102
   Compiling typenum v1.15.0
   Compiling pin-project-lite v0.2.9
   Compiling memchr v2.5.0
   Compiling futures-channel v0.3.24
   Compiling bytes v1.2.1
   Compiling subtle v2.4.1
   Compiling futures-core v0.3.24
   Compiling pin-utils v0.1.0
   Compiling itoa v1.0.4
   Compiling log v0.4.17
   Compiling serde_derive v1.0.145
   Compiling yansi v0.5.1
   Compiling once_cell v1.15.0
   Compiling serde v1.0.145
   Compiling cpufeatures v0.2.5
   Compiling fnv v1.0.7
   Compiling futures-task v0.3.24
   Compiling httparse v1.8.0
   Compiling futures-sink v0.3.24
   Compiling hashbrown v0.12.3
   Compiling futures-util v0.3.24
   Compiling try-lock v0.2.3
   Compiling opaque-debug v0.3.0
   Compiling futures-io v0.3.24
   Compiling ppv-lite86 v0.2.16
   Compiling ref-cast v1.0.9
   Compiling inlinable_string v0.1.15
   Compiling time-macros v0.2.4
   Compiling httpdate v1.0.2
   Compiling smallvec v1.10.0
   Compiling tower-service v0.3.2
   Compiling base64 v0.13.0
   Compiling bitflags v1.3.2
   Compiling percent-encoding v2.2.0
   Compiling state v0.5.3
   Compiling parking_lot_core v0.9.3
   Compiling either v1.8.0
   Compiling encoding_rs v0.8.31
   Compiling scopeguard v1.1.0
   Compiling async-trait v0.1.57
   Compiling glob v0.3.0
   Compiling mime v0.3.16
   Compiling remove_dir_all v0.5.3
   Compiling unicode-xid v0.2.4
   Compiling fastrand v1.8.0
   Compiling spin v0.9.4
   Compiling binascii v0.1.4
   Compiling tracing-core v0.1.30
   Compiling generic-array v0.14.6
   Compiling proc-macro2-diagnostics v0.9.1
   Compiling cookie v0.16.1
   Compiling uncased v0.9.7
   Compiling multer v2.0.4
   Compiling figment v0.10.8
   Compiling rocket v0.5.0-rc.2
   Compiling slab v0.4.7
   Compiling tokio v1.21.2
   Compiling indexmap v1.9.1
   Compiling lock_api v0.4.9
   Compiling atomic v0.5.1
   Compiling http v0.2.8
   Compiling want v0.3.0
   Compiling tracing v0.1.37
   Compiling stable-pattern v0.1.0
   Compiling getrandom v0.2.7
   Compiling socket2 v0.4.7
   Compiling signal-hook-registry v1.4.0
   Compiling num_cpus v1.13.1
   Compiling mio v0.8.4
   Compiling num_threads v0.1.6
   Compiling tempfile v3.3.0
   Compiling atty v0.2.14
   Compiling rand_core v0.6.4
   Compiling parking_lot v0.12.1
   Compiling rand_chacha v0.3.1
   Compiling http-body v0.4.5
   Compiling time v0.3.15
   Compiling rand v0.8.5
   Compiling crypto-common v0.1.6
   Compiling inout v0.1.3
   Compiling block-buffer v0.10.3
   Compiling universal-hash v0.5.0
   Compiling aead v0.5.1
   Compiling cipher v0.4.3
   Compiling digest v0.10.5
   Compiling polyval v0.6.0
   Compiling ghash v0.5.0
   Compiling hmac v0.12.1
   Compiling sha2 v0.10.6
   Compiling aes v0.8.1
   Compiling ctr v0.9.2
   Compiling hkdf v0.12.3
   Compiling futures v0.3.24
   Compiling aes-gcm v0.10.1
   Compiling hyper v0.14.20
   Compiling devise_core v0.3.1
   Compiling tokio-macros v1.8.0
   Compiling pear_codegen v0.2.3
   Compiling ref-cast-impl v1.0.9
   Compiling devise_codegen v0.3.1
   Compiling async-stream-impl v0.3.3
   Compiling async-stream v0.3.3
   Compiling devise v0.3.1
   Compiling pear v0.2.3
   Compiling rocket_http v0.5.0-rc.2
   Compiling rocket_codegen v0.5.0-rc.2
   Compiling tokio-util v0.7.4
   Compiling tokio-stream v0.1.10
   Compiling toml v0.5.9
   Compiling ubyte v0.10.3
   Compiling h2 v0.3.14
   Compiling rocket_app v0.1.0 (/Users/heidsoft/Downloads/research/rust/heidsoft-rust/rocket_app)
    Finished dev [unoptimized + debuginfo] target(s) in 1m 10s
     Running `target/debug/rocket_app`
🔧 Configured for debug.
   >> address: 127.0.0.1
   >> port: 8000
   >> workers: 16
   >> ident: Rocket
   >> limits: bytes = 8KiB, data-form = 2MiB, file = 1MiB, form = 32KiB, json = 1MiB, msgpack = 1MiB, string = 8KiB
   >> temp dir: /var/folders/p6/vf6vrtts00n22t8qlszy99xm0000gn/T/
   >> http/2: true
   >> keep-alive: 5s
   >> tls: disabled
   >> shutdown: ctrlc = true, force = true, signals = [SIGTERM], grace = 2s, mercy = 3s
   >> log level: normal
   >> cli colors: true
📬 Routes:
   >> (index) GET /
📡 Fairings:
   >> Shield (liftoff, response, singleton)
🛡️ Shield:
   >> Permissions-Policy: interest-cohort=()
   >> X-Frame-Options: SAMEORIGIN
   >> X-Content-Type-Options: nosniff
🚀 Rocket has launched from http://127.0.0.1:8000
GET / text/html:
   >> Matched: (index) GET /
   >> Outcome: Success
   >> Response succeeded.
GET /favicon.ico image/avif:
   >> No matching routes for GET /favicon.ico image/avif.
   >> No 404 catcher registered. Using Rocket default.
   >> Response succeeded.
GET / text/html:
   >> Matched: (index) GET /
   >> Outcome: Success
   >> Response succeeded.
```