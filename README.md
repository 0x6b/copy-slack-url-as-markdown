# `copy-slack-url-as-markdown`

Convert a Slack URL in the clipboard to Markdown format, then copy back to the clipboard.

## Setup

Expose your Slack user token as `SLACK_TOKEN` environment variable to use the utilities, or pass the token as an argument. Please refer to the [Slack documentation](https://api.slack.com/concepts/token-types) to see how to get the token.

Following permission scopes would be required. The required scopes depend on the type of channel-like object you're working with. You only need the scopes corresponding to that conversation type, found below.

- Public channels: [`channels:history`](https://api.slack.com/scopes/channels:history) and  [`channels:read`](https://api.slack.com/scopes/channels:read)
- Private channels: [`groups:history`](https://api.slack.com/scopes/groups:history) and  [`groups:read`](https://api.slack.com/scopes/groups:read)
- Direct messages: [`im:history`](https://api.slack.com/scopes/im:history) and  [`im:read`](https://api.slack.com/scopes/im:read)
- Group direct messages: [`mpim:history`](https://api.slack.com/scopes/mpim:history) and  [`mpim:read`](https://api.slack.com/scopes/mpim:read)

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
          The IANA time zone database identifiers to use for the timestamp [default: Asia/Tokyo]
      --template-text <TEMPLATE_TEXT>                        
          Path to the template file for plain text, without quote [env: TEMPLATE_TEXT=]
      --template-text-quote <TEMPLATE_TEXT_QUOTE>            
          Path to the template file for plain text, with quote [env: TEMPLATE_TEXT_QUOTE=]
      --template-rich-text <TEMPLATE_RICH_TEXT>              
          Path to the template file for rich text, without quote [env: TEMPLATE_RICH_TEXT=]
      --template-rich-text-quote <TEMPLATE_RICH_TEXT_QUOTE>  
          Path to the template file for rich text, with quote [env: TEMPLATE_RICH_TEXT_QUOTE=]
  -h, --help
          Print help
  -V, --version
          Print version
```

### Templates

You can customize the output format by providing a template file via the `--template-*` options, or respective environment variables. The template file is a plain text file that contains the format of the output message. Under the hood, this program uses the [tera](https://tera.netlify.app/) template engine, allowing you to take full advantage of its capabilities.

- `--template-text`: The template file for plain text, without quote.
- `--template-text-quote`: The template file for plain text, with quote.
- `--template-rich-text`: The template file for rich text (basically an HTML), without quote.
- `--template-rich-text-quote`: The template file for rich text, with quote.

The pre-defined variables are below:

| Variable               | `strftime` Specifier     | Example                              | Description                                                                      |
|------------------------|--------------------------|--------------------------------------|----------------------------------------------------------------------------------|
| `{{ channel_name }}`   | -                        | `general`                            | The name of the channel the message belongs to.                                  |
| `{{ url }}`            | -                        | `https://xxx.slack.com/archives/...` | The Slack URL of the message.                                                    |
| `{{ text }}`           | -                        | `message text`                       | The text of the message, which is the vector of the texts split by the new line. |
| `{{ timestamp }}`      | `%Y-%m-%d %H:%M:%S (%Z)` | `2024-08-27 11:28:53 (JST)`          | The timestamp of the message.                                                    |
| `{{ iso_date }}`       | `%F`                     | `2024-07-14`                         | Equivalent to `%Y-%m-%d`.                                                        |
| `{{ clock }}`          | `%T`                     | `23:30:59`                           | Equivalent to `%H:%M:%S`.                                                        |
| `{{ year }}`           | `%Y`                     | `2024`                               | A full year, including century. Zero padded to 4 digits.                         |
| `{{ year_2digit }}`    | `%y`                     | `24`                                 | A two-digit year. Represents only 1969-2068. Zero padded.                        |
| `{{ month }}`          | `%B`                     | `June`                               | The full month name.                                                             |
| `{{ month_abbrev }}`   | `%b`                     | `Jun`                                | The abbreviated month name,.                                                     |
| `{{ month_2digit }}`   | `%m`                     | `01`                                 | The month. Zero padded.                                                          |
| `{{ day }}`            | `%d`                     | `25`                                 | The day of the month. Zero-padded.                                               |
| `{{ day_space }}`      | `%e`                     | `5`                                  | The day of the month. Space padded.                                              |
| `{{ hour24 }}`         | `%H`                     | `23`                                 | The hour in a 24 hour clock. Zero padded.                                        |
| `{{ hour12 }}`         | `%I`                     | `11`                                 | The hour in a 12 hour clock. Zero padded.                                        |
| `{{ minute }}`         | `%M`                     | `04`                                 | The minute. Zero padded.                                                         |
| `{{ second }}`         | `%S`                     | `59`                                 | The second. Zero padded.                                                         |
| `{{ ampm }}`           | `%p`                     | `PM`                                 | Whether the time is in the AM or PM, uppercase.                                  |
| `{{ ampm_lower }}`     | `%P`                     | `am`                                 | Whether the time is in the AM or PM, lowercase.                                  |
| `{{ weekday }}`        | `%A`                     | `Sunday`                             | The full weekday.                                                                |
| `{{ weekday_abbrev }}` | `%a`                     | `Sun`                                | The abbreviated weekday.                                                         |
| `{{ tz_iana }}`        | `%V`                     | `Asia/Tokyo`, `+0900`                | An IANA time zone identifier, or `%z` if one doesn't exist.                      |
| `{{ tz_iana_colon }}`  | `%:V`                    | `Asia/Tokyo`, `+09:00`               | An IANA time zone identifier, or `%:z` if one doesn't exist.                     |
| `{{ tz_abbrev }}`      | `%Z`                     | `JST`                                | A time zone abbreviation. Supported when formatting only.                        |
| `{{ offset }}`         | `%z`                     | `+0900`                              | A time zone offset in the format `[+-]HHMM[SS]`.                                 |
| `{{ offset_colon }}`   | `%:z`                    | `+09:00`                             | A time zone offset in the format `[+-]HH:MM[:SS]`.                               |

See [`templates`](templates) for the default templates.

## License

MIT. See [LICENSE](LICENSE) for details.

## References

- [Web API methods | Slack](https://api.slack.com/methods)
- [Token types | Slack](https://api.slack.com/concepts/token-types)
- [Permission scopes | Slack](https://api.slack.com/scopes)
- [Tera](https://keats.github.io/tera/)

## Privacy

This utility does not share any fetched Slack conversations with third parties.
