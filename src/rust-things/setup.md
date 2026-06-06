# Competitive programming with Rust

## Setup
### Install Fira font
See [offical document](https://github.com/tonsky/FiraCode)

### Sublimetext
Sublimetext's Menu -> `Preference` -> `Settings`

```json
{
    "font_size": 14,
    "color_scheme": "Solarized (light).sublime-color-scheme",
    "index_files": true,
    "save_on_focus_lost": true,
    "font_face": "Fira Code Medium",
    "enable_font_ligatures": true,
    "font_options": ["subpixel_antialias"],
}
```

### Rust build system
Sublimetext's Menu -> `Tools` -> `Build system` -> `New Build system ...`

```json showLineNumbers
{
   // Add "cargo fmt" if LSP "lsp_format_on_save": true, is not set
  "cmd" : "cargo clippy --fix --allow-dirty --allow-no-vcs && cargo run", // cargo might needs to be a full path
  "selector" : "*.rs",
  "shell": true,
  "working_dir" : "$file_path",
  "file_regex": "^(..[^:]*):([0-9]+):?([0-9]+)?:? (.*)$",
  "quiet": false
}
```

`Ctrl + S` to save file as `rust.sublime-build`

Select Rust as build system `Tools` -> `Build system` -> `rust`. Now, just run **F1** to **format + lint + run**

**Rust enhanced** also has a built-in build system but don't use it.

### Key bindings
`Menu` -> `Preferences` -> `Key Bindings`

```json showLineNumbers
[
    { "keys": ["f1"], "command": "build" },
    { "keys": ["f2"], "command": "cancel_build" },
    {
        "keys": ["f3"],
        "command": "lsp_show_diagnostics_panel"
    }
]
```

Now, run 
* `F1` on `main.rs` to run.
* `F2` to cancel build + run
* `F3` to show Rust analyzer logs

### Setup LSP (language server protocol)
LSP is used to check code immediately without compiling then running it.

**Some links**:
* [Sublimetext LSP](https://github.com/sublimelsp/LSP)
* [LSP-rust-analyzer](https://github.com/sublimelsp/LSP-rust-analyzer)

`Ctrl + Shift + P` -> `Preferences: LSP Settings` then fill
```json showLineNumbers
// Settings in here override those in "LSP/LSP.sublime-settings"
{
	"lsp_format_on_save": true,
}
```
Then `Ctrl + Shift + P` -> `LSP: Toggle Diagnostics Panel`, it shows realtime error, warning on typing.

**Show inlay hints**: `Menu` -> `View` -> `LSP: Show inlay hints`. It shows type of variables, ...

### Sublimetext Plugins
`Ctrl + Shift + P` -> `Install Package` -> input `Rust Enhanced`

* [Rust Enhanced](https://rust-lang.github.io/rust-enhanced/install.html) -> right mouse -> Rust -> Open settings
    ```json showLineNumbers
    {
	     "folders": [
	        { "path": "." }
	    ],
        "settings": {
            // Changes the default on-save checking behavior to use Clippy.
            "rust_syntax_checking_method": "clippy",
        },
    }
    ```
    To enable it, `right mouse` -> `Rust` -> `On-save checking`, now everytime, we press `Ctrl + S` or sublime text saves file, it will run `clippy` (pretty annoying if we want speed like in a contest)

## Other
### Kill sublimetext
In case sublimetext hang too long
```shell
killall sublime_text
```
