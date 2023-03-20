weather
------------
**weather** is a command-line tool that shows weather.
It supports multiple weather providers. You can get weather by address and date.
Uses the APIs of providers to get data about weather. Supports both historical and forecast data.
Date ranges, address, response and errors depend on a provider. See the API documentation for each supported provider.

The configuration file are stored in the expected place for your system. For Linux it's `$XDG_CONFIG_HOME` or `$HOME/.config`,
Windows it's `{FOLDERID_RoamingAppData}` and MacOS it's `$HOME/Library/Application Support`. See the [`directories`] crates for more information.

# Usage
## Options
```shell
Usage: weather <COMMAND>

Commands:
  configure  Configure credentials for a provider
  get        Show weather for an address
  set        Set a current provider
  provider   Return current provider
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
## Examples
### Configuring
Firstly, you need to list supported providers and choose one of them:
```shell
$ weather configure --help
Configure credentials for a provider

Usage: weather configure <PROVIDER>

Arguments:
  <PROVIDER>  [possible values: open-weather, weather-api]

Options:
  -h, --help  Print help
```
Configure credentials for a provider:
```shell
$ weather configure open-weather
type_your_api_key_here
Credentials are configured successfully
```
Set the current provider:
```shell
$ weather set open-weather
```
### Get current weather in London
```shell
$ weather get London
{"lat":51.5073,"lon":-0.1276,"timezone":"Europe/London","timezone_offset":0,"data":[{"dt":1678976786,"sunrise":1678947201,"sunset":1678989917,"temp":286.81,"feels_like":286.06,"pressure":1002,"humidity":70,"dew_point":281.45,"uvi":1.06,"clouds":100,"visibility":10000,"wind_speed":6.17,"wind_deg":190,"weather":[{"id":804,"main":"Clouds","description":"overcast clouds","icon":"04d"}]}]}
```
### Get weather in Paris for the specified date
```shell
$ weather get Paris 2023-03-21
{"lat":48.8589,"lon":2.32,"timezone":"Europe/Paris","timezone_offset":3600,"data":[{"dt":1679356800,"sunrise":1679377947,"sunset":1679421813,"temp":286.13,"feels_like":285.31,"pressure":1020,"humidity":70,"dew_point":280.79,"uvi":3.53,"clouds":82,"visibility":10000,"wind_speed":1.46,"wind_deg":151,"wind_gust":1.5,"weather":[{"id":803,"main":"Clouds","description":"broken clouds","icon":"04n"}]}]}
```

[`directories`]: https://crates.io/crates/directories