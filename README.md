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

Usage: copy-slack-url-as-markdown [OPTIONS] --token <TOKEN>

Options:
  -t, --token <TOKEN>    Slack API token [env: SLACK_TOKEN=xoxp-...]
  -q, --quote            Include the message body as a quote
      --prefix <PREFIX>  Prefix to the title [default: Slack#]
      --style <STYLE>    Style of the quoted message in rich text [default:
                         "color: rgb(96, 96, 96);"]
  -h, --help             Print help
  -V, --version          Print version
```

## License

MIT. See [LICENSE](LICENSE) for details.

## References

- [Web API methods | Slack](https://api.slack.com/methods)
- [Token types | Slack](https://api.slack.com/concepts/token-types)
- [Permission scopes | Slack](https://api.slack.com/scopes)

## Privacy

This utility does not share any fetched Slack conversations with third parties.