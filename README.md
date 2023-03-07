# Volcano 
## What is Volcano?
Volcano is a complete set of software to run a markdown server, including a server to index the files, a parser to convert the files to Dioxus syntax, and frontend web UI to view the notes.

---

## Project Breakdown
`server/` - A crate containing a server listening on localhost.

`volcano_parser/` - A markdown parser to convert Markdown text into simple tokens.

`webui/` - A Dioxus frontend (targeting web) to fetch notes from the server, tokenize them, and display them as HTML.

---

## Goals/Features
Volcano's parser is incredibly minimalistic. While I aim to support every feature of GFM in the future, currently the parser is very bare-bones and only supports what I need for my notes. However, it *does* support LaTeX rendering (through KaTeX), as the main inspiration behind this project was a lack of LaTeX support from other open source Markdown servers.

*\* = Planned in the short term*
| Feature | Supported |
|-------- | --------- |
| Bold    | No*       |
| Italic  | No*       |
| LaTeX   | No*       |
| Headers | No*       |
| Lists   | No        |
| Code    | No        |
| Inline Code | No*   |
