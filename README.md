# ChatGPT By Voice

### Description

Simple application for chatting with ChatGPT using voice, implemented in Rust language.

![ChatGPT APP](https://raw.githubusercontent.com/torys877/chatgpt_by_voice/main/docs/chatgpt_voice.png)

### Features

- record you voice
- convert audio file in text, using OpenAI Api `v1/audio/transcriptions`
- send converted text to ChatGPT model (model and prompt tokens can be changed in main.rs), using OpenAI Api `v1/completions`
- send usual text to ChatGPT

### How It Works

#### Speech To Text

- enter you OpenAI Api Key (you need have some money on balance to use API)
- press `Start Record` to begin record your voice
- press `Stop Record` to stop record
- press `Speech To Text` to send request to OpenAi to convert speech to text

#### Usual Chat

- enter you OpenAI Api Key (you need have some money on balance to use API)
- write message and press `Enter`, or `Send`

### Ihor Oleksiienko

* [Github](https://github.com/torys877)
* [Linkedin](https://www.linkedin.com/in/igor-alekseyenko-77613726/)
* [Facebook](https://www.facebook.com/torysua/)