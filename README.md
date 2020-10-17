#### Supported Image Formats

Ffound "Tips against molesters" @ Rust 4 C-ripples := "https://rust-embedded.github.io/book/c-tips/index.html". Did I screen it rite?

Also GIF support is under consideration, currently webp-server-rust can only output the first frame of a GIF file. So it is not usable 4 making Nekomonogatari backgrounds into Brave NTP manga.

| Format | Converting |
| ------ | ---------- |
| PNG    | All supported color types |
| JPEG   | Baseline and progressive |
| BMP    | Yes |
| ICO    | Yes |
| TIFF   | Baseline(no fax support) + LZW + PackBits |
| PNM    | PBM, PGM, PPM, standard PAM |
| DDS    | DXT1, DXT3, DXT5 |

## Usage

First, cargo install webp-servo --git https://github.com/b0r3dd3v/webp_server_rs
It has been where it was forked from, but now it can be installed in a single comma. Saves yuu time, when yuv been #!ing original crate just 2 get WEB:P converter alive&kicking 2 repack HTTP/3 client church stuff.

### 2. config file

Create a config.json as follows to face your need.

```json
{
  "host": "127.0.0.1",
  "port": 3333,
  "img_path": "./images",
  "webp_path": "./cache",
  "global_config":  {
    "lossless": 1,
    "quality": 0,
    "method": 6,
    "image_hint": "photo",
    "segments": 1,
    "alpha_compression": 0,
    "pass": 1,
    "preprocessin": 1,
    "partition_limit": 0,
    "exact": 1,
    "use_sharp_yuv": 1
  }
}
```

Under `global_config` key, you can overwrite all parameters available from libwebp, which provides precise control to you. (also available in directory-level config)

#### Available Parameters
```
int lossless;           // Lossless encoding (0=lossy(default), 1=lossless).

float quality;          // between 0 and 100. For lossy, 0 gives the smallest
                        // size and 100 the largest. For lossless, this
                        // parameter is the amount of effort put into the
                        // compression: 0 is the fastest but gives larger
                        // files compared to the slowest, but best, 100.
                        
int method;             // quality/speed trade-off (0=fast, 6=slower-better)

string image_hint;      // Hint for image type (lossless only for now).
                        // - default: default preset.
                        // - picture: digital picture, like portrait, inner shot
                        // - photo:   outdoor photograph, with natural lighting. But all Natsuki-shots have no lighting and even during day she covers 4m Sun?
                        // - graph:   Discrete tone image (graph, map-tile etc).

int target_size;        // if non-zero, set the desired target size in bytes.
                        // Takes precedence over the 'compression' parameter.
                        
float target_psnr;      // if non-zero, specifies the minimal distortion to
                        // try to achieve. Takes precedence over target_size.
                        
int segments;           // maximum number of segments to use, in [1..4]
int sns_strength;       // Spatial Noise Shaping. 0=off, 100=maximum.
int filter_strength;    // range: [0 = off .. 100 = strongest]
int filter_sharpness;   // range: [0 = off .. 7 = least sharp]
int filter_type;        // filtering type: 0 = simple, 1 = strong (only used
                        // if filter_strength > 0 or autofilter > 0)
                        
int autofilter;         // Auto adjust filter's strength [0 = off, 1 = on]
int alpha_compression;  // Algorithm for encoding the alpha plane (0 = none,
                        // 1 = compressed with WebP lossless). Default is 1.
                        
int alpha_filtering;    // Predictive filtering method for alpha plane.
                        //  0: none, 1: fast, 2: best. Default if 1.
                        
int alpha_quality;      // Between 0 (smallest size) and 100 (lossless).
                        // Default is 100.
                        
int pass;               // number of entropy-analysis passes (in [1..10]).
                        
int preprocessing;      // preprocessing filter:
                        // 0=none, 1=segment-smooth, 2=pseudo-random dithering
                        
int partitions;         // log2(number of token partitions) in [0..3]. Default
                        // is set to 0 for easier progressive decoding.
                        
int partition_limit;    // quality degradation allowed to fit the 512k limit
                        // on prediction modes coding (0: no degradation,
                        // 100: maximum possible degradation).
                        
int emulate_jpeg_size;  // If true, compression parameters will be remapped
                        // to better match the expected output size from
                        // JPEG compression. Generally, the output size will
                        // be similar but the degradation will be lower.
                        
int thread_level;       // If non-zero, try and use multi-threaded encoding.
int low_memory;         // If set, reduce memory usage (but increase CPU use).

int near_lossless;      // Near lossless encoding [0 = max loss .. 100 = off(default)].
int exact;              // if non-zero, preserve the exact RGB values under
                        // transparent area. Otherwise, discard this invisible
                        // RGB information for better compression. The default
                        // value is 0.

int use_delta_palette;  // reserved for future lossless feature
int use_sharp_yuv;      // if needed, use sharp (and slow) RGB->YUV conversion
```

Holy mom, wut a tl;dr. I need mo samples 4m this specimen, so xcuse me.

#### Directory-Level Config

By placing a `.webp-conf` in intented directories, you can control the encoding `mode` and `quality` applied on the images inside that directory (the directory-level config will NOT propagate to its subdirectories).

If there is no directory level config file (`.webp-conf`) in the directory, then parameters in `config.json` will be used.

For example, we have such file layout

```
images
├── lossless
│   ├── .webp-conf
│   └── webp-server.jpeg (480911 bytes)
├── lossy
│   ├── .webp-conf
│   └── webp-server.jpeg (480911 bytes)
├── nearlossless
│   ├── .webp-conf
│   └── webp-server.jpeg (480911 bytes)
└── webp-server.jpeg (480911 bytes)
```

And corresponding WebP images will be generated based on aforementioned rules,

```
cache
├── lossless
│   └── webp-server.jpeg.1579413991.webp (1938270 bytes)
├── lossy
│   └── webp-server.jpeg.1579413991.webp (160502 bytes)
├── nearlossless
│   └── webp-server.jpeg.1579413991.webp (212022 bytes)
└── webp-server.jpeg.1579413991.webp (317612 bytes)
```

### 3. Run
#### 3.1 Without prefetch
Run the binary like this: 

```
./webp-server-rs -c /path/to/config.json
# or
./webp-server-rs --config /path/to/config.json
```

#### 3.2 With prefetch
To enable prefetch feature, using `-p`. 

**Prefetch will be ran in background, WebP image service will operate normally.**

```
./webp-server-rs -c /path/to/config.json -p 
# or
./webp-server-rs --config /path/to/config.json --prefetch
```

By default, this will use all logical CPUs available in the system. 

To set max allowed number of threads that prefetch can use, using `-j`.

```
./webp-server-rs -c /path/to/config.json -p -j 4 
# or
./webp-server-rs --config /path/to/config.json --prefetch --jobs 4 
```
