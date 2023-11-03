# iMessage Query

Allows easily getting iMessage messages as raw text, which can be useful for having an LLM respond and impersonate you.

I recommend using [playground.openai.com](https://platform.openai.com/playground?mode=complete) for this
and selecting `Mode: Complete (Legacy)`.
Legacy mode is important because it acts on raw text blocks rather than the new API which acts on chat messages.
Because legacy mode is not fine-tuned to produce corporate-style chat messages, completion mode is much more likely to
emulate your style of writing.

When using `imessage-query {handle_id}`,
the output will automatically be copied to your clipboard, so that it can more easily be pasted into the OpenAI
playground.

```text
Usage: imessage-query [OPTIONS] [HANDLE_ID]

Arguments:
  [HANDLE_ID]  The handle id of the person you're chatting with

Options:
  -t, --to <TO>  The name of the person you're chatting with [default: Bob]
  -m, --me <ME>  The name of yourself [default: Andrew]
  -h, --help     Print help
  -V, --version  Print version
```
