# `copy-slack-url-as-markdown`

Convert a Slack URL in the clipboard to Markdown format, then copy back to the clipboard.

## Setup

Expose your Slack user token as `SLACK_TOKEN` environment variable to use the utilities, or pass the token as an argument. Please refer to the [Slack documentation](https://api.slack.com/concepts/token-types) to see how to get the token.

The following permission scopes are required to resolve the user and user group names:

- [`users:read`](https://api.slack.com/scopes/users:read)
- [`usergroups:read`](https://api.slack.com/scopes/usergroups:read)

Following permission scopes would also be required to get the message to copy, depend on the type of channel-like object (conversation type) you're working with.

- Public channels: [`channels:history`](https://api.slack.com/scopes/channels:history), [`channels:read`](https://api.slack.com/scopes/channels:read)
- Private channels: [`groups:history`](https://api.slack.com/scopes/groups:history), [`groups:read`](https://api.slack.com/scopes/groups:read)
- Direct messages: [`im:history`](https://api.slack.com/scopes/im:history), [`im:read`](https://api.slack.com/scopes/im:read)
- Group direct messages: [`mpim:history`](https://api.slack.com/scopes/mpim:history), [`mpim:read`](https://api.slack.com/scopes/mpim:read)

## Usage

```console
$ s2m --help
Copy Slack URL as Markdown

Usage: s2m [OPTIONS] --token <TOKEN>

Options:
      --token <TOKEN>
          Slack API token [env: SLACK_TOKEN=xoxp-...]
  -q, --quote
          Include the message body as a quote
  -t, --timezone <TIMEZONE>
          The IANA time zone database identifiers to use for the timestamp
          [default: Asia/Tokyo]
      --text <TEXT>
          Path to the template file or a string for plain text (without
          quote). Leave empty to use the default [env: TEMPLATE_TEXT=]
      --text-quote <TEXT_QUOTE>
          Path to the template file or a string for plain text (with quote).
          Leave empty to use the default [env: TEMPLATE_TEXT_QUOTE=]
      --rich-text <RICH_TEXT>
          Path to the template file or a string for rich text (without quote).
          Leave empty to use the default [env: TEMPLATE_RICH_TEXT=]
      --rich-text-quote <RICH_TEXT_QUOTE>
          Path to the template file or a string for rich text (with quote).
          Leave empty to use the template [env: TEMPLATE_RICH_TEXT_QUOTE=]
  -h, --help
          Print help
  -V, --version
          Print version
```

### Templates

You can customize the output format by providing a path to the template file or a string via the options, or respective environment variables. The template file is a plain text file that contains the format of the output message. Leave empty to use the default. Under the hood, this program uses the [Tera](https://keats.github.io/tera/) template engine, allowing you to take full advantage of its capabilities.

| Option              | Environment Variable       | For                       |
|---------------------|----------------------------|---------------------------|
| `--text`            | `TEMPLATE_TEXT`            | plain text, without quote |
| `--text-quote`      | `TEMPLATE_TEXT_QUOTE`      | plain text, with quote    |
| `--rich-text`       | `TEMPLATE_RICH_TEXT`       | rich text, without quote  |
| `--rich-text-quote` | `TEMPLATE_RICH_TEXT_QUOTE` | rich text, with quote     |

The pre-defined variables, which you can use in the template i.e., `{{ variable }}`, are below:

| Variable         | [`jiff:fmt::strftime`](https://docs.rs/jiff/latest/jiff/fmt/strtime/) Specifier | Example                              | Description                                                                      |
|------------------|---------------------------------------------------------------------------------|--------------------------------------|----------------------------------------------------------------------------------|
| `channel_name`   | (not available)                                                                 | `general`                            | The name of the channel the message belongs to.                                  |
| `user_name`      | (not available)                                                                 | `jake`                               | The name of the user who posted the message.                                     |
| `url`            | (not available)                                                                 | `https://xxx.slack.com/archives/...` | The Slack URL of the message.                                                    |
| `text`           | (not available)                                                                 | `Hello, world!`                      | The text of the message, which is the vector of the texts split by the new line. |
| `html`           | (not available)                                                                 | `<p>Hello, world!</p>`               | The HTML version of the message                                                  |
| `timestamp`      | `%Y-%m-%d %H:%M:%S (%Z)`                                                        | `2024-08-22 02:39:12 (JST)`          | The timestamp of the message.                                                    |
| `iso_date`       | `%F`                                                                            | `2024-08-22`                         | Equivalent to `%Y-%m-%d`.                                                        |
| `clock`          | `%T`                                                                            | `02:39:12`                           | Equivalent to `%H:%M:%S`.                                                        |
| `year`           | `%Y`                                                                            | `2024`                               | A full year, including century. Zero padded to 4 digits.                         |
| `year_2digit`    | `%y`                                                                            | `24`                                 | A two-digit year. Represents only 1969-2068. Zero padded.                        |
| `month`          | `%B`                                                                            | `August`                             | The full month name.                                                             |
| `month_abbrev`   | `%b`                                                                            | `Aug`                                | The abbreviated month name,.                                                     |
| `month_2digit`   | `%m`                                                                            | `08`                                 | The month. Zero padded.                                                          |
| `day`            | `%d`                                                                            | `22`                                 | The day of the month. Zero-padded.                                               |
| `day_space`      | `%e`                                                                            | `22`                                 | The day of the month. Space padded.                                              |
| `hour24`         | `%H`                                                                            | `02`                                 | The hour in a 24 hour clock. Zero padded.                                        |
| `hour12`         | `%I`                                                                            | `02`                                 | The hour in a 12 hour clock. Zero padded.                                        |
| `minute`         | `%M`                                                                            | `39`                                 | The minute. Zero padded.                                                         |
| `second`         | `%S`                                                                            | `12`                                 | The second. Zero padded.                                                         |
| `ampm`           | `%p`                                                                            | `AM`                                 | Whether the time is in the AM or PM, uppercase.                                  |
| `ampm_lower`     | `%P`                                                                            | `am`                                 | Whether the time is in the AM or PM, lowercase.                                  |
| `weekday`        | `%A`                                                                            | `Thursday`                           | The full weekday.                                                                |
| `weekday_abbrev` | `%a`                                                                            | `Thu`                                | The abbreviated weekday.                                                         |
| `tz_iana`        | `%V`                                                                            | `Asia/Tokyo`                         | An IANA time zone identifier, or `%z` if one doesn't exist.                      |
| `tz_abbrev`      | `%Z`                                                                            | `JST`                                | A time zone abbreviation. Supported when formatting only.                        |
| `offset`         | `%z`                                                                            | `+0900`                              | A time zone offset in the format `[+-]HHMM[SS]`.                                 |
| `offset_colon`   | `%:z`                                                                           | `+09:00`                             | A time zone offset in the format `[+-]HH:MM[:SS]`.                               |

See [`assets/templates`](assets/templates) for the default templates.

## Limitations

For quoting, only [Rich text block](https://api.slack.com/reference/block-kit/blocks#rich_text) is supported. Other types will be just ignored.

## License

MIT. See [LICENSE](LICENSE) for details.

## References

- [Web API methods | Slack](https://api.slack.com/methods)
- [Token types | Slack](https://api.slack.com/concepts/token-types)
- [Permission scopes | Slack](https://api.slack.com/scopes)
- [Reference: blocks | Slack](https://api.slack.com/reference/block-kit/blocks#user-element-type)
- [Tera](https://keats.github.io/tera/)

## Privacy

This utility does not share any fetched Slack conversations with third parties.
