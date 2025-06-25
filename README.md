# Jetsa

Convert FAA MVA and MIA charts to GeoJSON for use on networks like VATSIM.

The source XML files can be downloaded from the FAA's website at https://www.faa.gov/air_traffic/flight_info/aeronav/digital_products/mva_mia/

**Do not use for real world flight navigation.**

## Install

The `jetsa` application is a Rust-based executable. This means it can run on most Windows, MacOS, and Linux machines without the need for installing runtimes or other prerequisites.

### Installation

Visit the [Releases page](https://github.com/brianknight10/jetsa/releases). Download and unzip the package that is appropriate for your operating system and architecture. Place the `jetsa` executable (`jetsa.exe` on Windows) wherever you'd like on your file system.

## Usage

1. Download the XML file from the FAA's website (link above).

2. The application can be called using `jetsa` (or `jetsa.exe` on Windows). You can see the configuration options by running `jetsa --help`.

```
Convert FAA MVA XML to GeoJSON

Usage: jetsa [OPTIONS] --name <NAME> --source <SOURCE>

Options:
  -m, --magnetic-variation <MAGNETIC_VARIATION>
          Magnetic variation in degrees, positive values indicate east [default: 0]
  -n, --name <NAME>
          Name of the output map
  -s, --source <SOURCE>
          Path of the FAA XML file
  -h, --help
          Print help
  -V, --version
          Print version
```

3. Convert your FAA MVA XML file.

```sh
$ jetsa -n L30 -s L30_MVA_FUS3_2024.xml -m 10
```

3. The tool will parse the XML file and save the results in a `.geojson` file in the same folder as `jetsa`.

## License
This project is licensed under the [GPLv3 License](LICENSE).