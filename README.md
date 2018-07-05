# simfd

Easiest way to find location of any app that is installed on your Xcode simulators, and location of any simulator device.

## Usage

```
USAGE:
    simfd [FLAGS] [QUERY]...

FLAGS:
    -d, --device     Search among devices instead of apps.
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <QUERY>...    Queries to find app location or simulator device location.

```

### Example of finding location of an app

INPUT:

```bash
$ simfd reminders 'iphone x'
```

RESULT:

```
+---------------------+----------+----------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------+
| com.apple.reminders | iPhone X | iOS 11.1 | /Users/shane/Library/Developer/CoreSimulator/Devices/335FE518-D964-4F26-83F7-3E0E00DE2A3D/data/Containers/Data/Application/3A3AA459-95D3-43EC-8903-81074C44401C |
+---------------------+----------+----------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------+
| com.apple.reminders | iPhone X | iOS 11.2 | /Users/shane/Library/Developer/CoreSimulator/Devices/4F6D8A1B-690D-46C4-8AC0-B15906415C35/data/Containers/Data/Application/027A6613-3DB1-4F71-8C29-3F455AC1A4B5 |
+---------------------+----------+----------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------+
| com.apple.reminders | iPhone X | iOS 11.4 | /Users/shane/Library/Developer/CoreSimulator/Devices/1F5AEA2F-D166-40B7-B800-54CF98B9D524/data/Containers/Data/Application/D68CDFE3-1B28-4DE4-9221-DBF4DB5BFFAB |
+---------------------+----------+----------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------+
| com.apple.reminders | iPhone X | iOS 11.0 | /Users/shane/Library/Developer/CoreSimulator/Devices/4FA52A95-9D7B-4A78-86FA-C6AE0840E6D3/data/Containers/Data/Application/F3CAD791-ED43-40C3-BA97-9BC8327A4337 |
+---------------------+----------+----------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------+
| com.apple.reminders | iPhone X | iOS 11.3 | /Users/shane/Library/Developer/CoreSimulator/Devices/CE29517F-783F-497E-83EF-615BC70FFFE1/data/Containers/Data/Application/415A17EB-D835-4B14-AECA-6BF30D94331B |
+---------------------+----------+----------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------+
```

### Example of finding location of an simulator device

INPUT:

```bash
$ simfd -d 'ipad pro' 11.4
```

RESULT:

```
+---------------------------------------+----------+-------------------------------------------------------------------------------------------+
| iPad Pro (10.5-inch)                  | iOS 11.4 | /Users/shane/Library/Developer/CoreSimulator/Devices/265ACA4D-5BA4-4036-A8F8-D4BBABF1F37A |
+---------------------------------------+----------+-------------------------------------------------------------------------------------------+
| iPad Pro (12.9-inch)                  | iOS 11.4 | /Users/shane/Library/Developer/CoreSimulator/Devices/85944624-94C7-4D7C-BFB5-B8DE9CA061BC |
+---------------------------------------+----------+-------------------------------------------------------------------------------------------+
| iPad Pro (12.9-inch) (2nd generation) | iOS 11.4 | /Users/shane/Library/Developer/CoreSimulator/Devices/4F139B13-02A5-4C7C-B8BB-8ECB2DF1913B |
+---------------------------------------+----------+-------------------------------------------------------------------------------------------+
| iPad Pro (9.7-inch)                   | iOS 11.4 | /Users/shane/Library/Developer/CoreSimulator/Devices/4EF57962-4D02-45AB-9168-403B8DE6EF8A |
+---------------------------------------+----------+-------------------------------------------------------------------------------------------+
```

## Installation

If you don't have rust/cargo installed:

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

After having rust/cargo installed, or if you already have them:

```bash
$ git clone https://github.com/shaneqi/simfd.git
$ cd simfd
$ cargo install
```

## License

Apache License 2.0

## Contribution

Pull requests are welcome!