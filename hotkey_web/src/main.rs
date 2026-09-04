use dioxus::prelude::*;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

#[derive(Clone, PartialEq)]
struct Hotkey {
    software: &'static str,
    keys: &'static str,
    description: &'static str,
}

lazy_static::lazy_static! {
    static ref ALL_HOTKEYS: Vec<Hotkey> = vec![
        // VS Code
        Hotkey { software: "vscode", keys: "Ctrl+Shift+P", description: "Show Command Palette" },
        Hotkey { software: "vscode", keys: "Ctrl+P", description: "Quick Open, Go to File..." },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+N", description: "New window/instance" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+W", description: "Close window/instance" },
        Hotkey { software: "vscode", keys: "Ctrl+,", description: "User Settings" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+S", description: "Keyboard Shortcuts" },
        Hotkey { software: "vscode", keys: "Ctrl+X", description: "Cut line (empty selection)" },
        Hotkey { software: "vscode", keys: "Ctrl+C", description: "Copy line (empty selection)" },
        Hotkey { software: "vscode", keys: "Alt+Up/Down", description: "Move line up/down" },
        Hotkey { software: "vscode", keys: "Shift+Alt+Up/Down", description: "Copy line up/down" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+K", description: "Delete line" },
        Hotkey { software: "vscode", keys: "Ctrl+Enter", description: "Insert line below" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+Enter", description: "Insert line above" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+\\", description: "Jump to matching bracket" },
        Hotkey { software: "vscode", keys: "Ctrl+] / [", description: "Indent/outdent line" },
        Hotkey { software: "vscode", keys: "Home / End", description: "Go to beginning/end of line" },
        Hotkey { software: "vscode", keys: "Ctrl+Home", description: "Go to beginning of file" },
        Hotkey { software: "vscode", keys: "Ctrl+End", description: "Go to end of file" },
        Hotkey { software: "vscode", keys: "Ctrl+Up/Down", description: "Scroll line up/down" },
        Hotkey { software: "vscode", keys: "Alt+PgUp/PgDn", description: "Scroll page up/down" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+[", description: "Fold (collapse) region" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+]", description: "Unfold (uncollapse) region" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+[", description: "Fold (collapse) all subregions" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+]", description: "Unfold (uncollapse) all subregions" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+0", description: "Fold (collapse) all regions" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+J", description: "Unfold (uncollapse) all regions" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+C", description: "Add line comment" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+U", description: "Remove line comment" },
        Hotkey { software: "vscode", keys: "Ctrl+/", description: "Toggle line comment" },
        Hotkey { software: "vscode", keys: "Shift+Alt+A", description: "Toggle block comment" },
        Hotkey { software: "vscode", keys: "Alt+Z", description: "Toggle word wrap" },
        Hotkey { software: "vscode", keys: "Ctrl+T", description: "Show all Symbols" },
        Hotkey { software: "vscode", keys: "Ctrl+G", description: "Go to Line..." },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+O", description: "Go to Symbol..." },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+M", description: "Show Problems panel" },
        Hotkey { software: "vscode", keys: "F8", description: "Go to next error or warning" },
        Hotkey { software: "vscode", keys: "Shift+F8", description: "Go to previous error or warning" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+Tab", description: "Navigate editor group history" },
        Hotkey { software: "vscode", keys: "Alt+Left / Right", description: "Go back / forward" },
        Hotkey { software: "vscode", keys: "Ctrl+M", description: "Toggle Tab key moves focus" },
        Hotkey { software: "vscode", keys: "Ctrl+F", description: "Find" },
        Hotkey { software: "vscode", keys: "Ctrl+H", description: "Replace" },
        Hotkey { software: "vscode", keys: "F3 / Shift+F3", description: "Find next/previous" },
        Hotkey { software: "vscode", keys: "Alt+Enter", description: "Select all occurences of Find match" },
        Hotkey { software: "vscode", keys: "Ctrl+D", description: "Add selection to next Find match" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+D", description: "Move last selection to next Find match" },
        Hotkey { software: "vscode", keys: "Alt+Click", description: "Insert cursor" },
        Hotkey { software: "vscode", keys: "Ctrl+Alt+Up / Down", description: "Insert cursor above / below" },
        Hotkey { software: "vscode", keys: "Ctrl+U", description: "Undo last cursor operation" },
        Hotkey { software: "vscode", keys: "Shift+Alt+I", description: "Insert cursor at end of each line selected" },
        Hotkey { software: "vscode", keys: "Ctrl+I", description: "Select current line" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+L", description: "Select all occurrences of current selection" },
        Hotkey { software: "vscode", keys: "Ctrl+F2", description: "Select all occurrences of current word" },
        Hotkey { software: "vscode", keys: "Shift+Alt+Right", description: "Expand selection" },
        Hotkey { software: "vscode", keys: "Shift+Alt+Left", description: "Shrink selection" },
        Hotkey { software: "vscode", keys: "Shift+Alt+Drag", description: "Column (box) selection" },
        Hotkey { software: "vscode", keys: "Ctrl+Space", description: "Trigger suggestion" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+Space", description: "Trigger parameter hints" },
        Hotkey { software: "vscode", keys: "Shift+Alt+F", description: "Format document" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+F", description: "Format selection" },
        Hotkey { software: "vscode", keys: "F12", description: "Go to Definition" },
        Hotkey { software: "vscode", keys: "Alt+F12", description: "Peek Definition" },
        Hotkey { software: "vscode", keys: "Ctrl+K F12", description: "Open Definition to the side" },
        Hotkey { software: "vscode", keys: "Ctrl+.", description: "Quick Fix" },
        Hotkey { software: "vscode", keys: "Shift+F12", description: "Show References" },
        Hotkey { software: "vscode", keys: "F2", description: "Rename Symbol" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+X", description: "Trim trailing whitespace" },
        Hotkey { software: "vscode", keys: "Ctrl+K M", description: "Change file language" },
        Hotkey { software: "vscode", keys: "Ctrl+F4 / Ctrl+W", description: "Close editor" },
        Hotkey { software: "vscode", keys: "Ctrl+K F", description: "Close folder" },
        Hotkey { software: "vscode", keys: "Ctrl+\\", description: "Split editor" },
        Hotkey { software: "vscode", keys: "Ctrl+1 / 2 / 3", description: "Focus into 1st, 2nd or 3rd editor group" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+Left/Right", description: "Focus into previous/next editor group" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+PgUp/PgDn", description: "Move editor left/right" },
        Hotkey { software: "vscode", keys: "Ctrl+K Left / Right", description: "Move active editor group" },
        Hotkey { software: "vscode", keys: "Ctrl+N", description: "New File" },
        Hotkey { software: "vscode", keys: "Ctrl+O", description: "Open File..." },
        Hotkey { software: "vscode", keys: "Ctrl+S", description: "Save" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+S", description: "Save As..." },
        Hotkey { software: "vscode", keys: "Ctrl+K S", description: "Save All" },
        Hotkey { software: "vscode", keys: "Ctrl+F4", description: "Close" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+W", description: "Close All" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+T", description: "Reopen closed editor" },
        Hotkey { software: "vscode", keys: "Ctrl+K Enter", description: "Keep preview mode editor open" },
        Hotkey { software: "vscode", keys: "Ctrl+Tab", description: "Open next" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+Tab", description: "Open previous" },
        Hotkey { software: "vscode", keys: "Ctrl+K P", description: "Copy path of active file" },
        Hotkey { software: "vscode", keys: "Ctrl+K R", description: "Reveal active file in Explorer" },
        Hotkey { software: "vscode", keys: "Ctrl+K O", description: "Show active file in new window/instance" },
        Hotkey { software: "vscode", keys: "F11", description: "Toggle full screen" },
        Hotkey { software: "vscode", keys: "Shift+Alt+0", description: "Toggle editor layout (horizontal/vertical)" },
        Hotkey { software: "vscode", keys: "Ctrl+= / -", description: "Zoom in/out" },
        Hotkey { software: "vscode", keys: "Ctrl+B", description: "Toggle Primary Side Bar visibility" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+E", description: "Show Explorer / Toggle Focus" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+F", description: "Show Search" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+G", description: "Show Source Control" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+D", description: "Show Run and Debug" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+X", description: "Show Extensions" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+H", description: "Replace in files" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+J", description: "Toggle Search details" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+U", description: "Show Output panel" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+V", description: "Open Markdown preview" },
        Hotkey { software: "vscode", keys: "Ctrl+K V", description: "Open Markdown preview to the side" },
        Hotkey { software: "vscode", keys: "Ctrl+K Z", description: "Zen Mode (Esc Esc to exit)" },
        Hotkey { software: "vscode", keys: "F9", description: "Toggle breakpoint" },
        Hotkey { software: "vscode", keys: "F5", description: "Start / Continue" },
        Hotkey { software: "vscode", keys: "Shift+F5", description: "Stop" },
        Hotkey { software: "vscode", keys: "F11 / Shift+F11", description: "Step into / out" },
        Hotkey { software: "vscode", keys: "F10", description: "Step over" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+I", description: "Show hover" },
        Hotkey { software: "vscode", keys: "Ctrl+`", description: "Show integrated terminal" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+`", description: "Create new terminal" },
        Hotkey { software: "vscode", keys: "Ctrl+C", description: "Copy selection" },
        Hotkey { software: "vscode", keys: "Ctrl+V", description: "Paste into active terminal" },
        Hotkey { software: "vscode", keys: "Ctrl+Up / Down", description: "Scroll up / down" },
        Hotkey { software: "vscode", keys: "Shift+PgUp / PgDn", description: "Scroll page up / down" },
        Hotkey { software: "vscode", keys: "Ctrl+Home / End", description: "Scroll to top / bottom" },

        // =======================
        // Vim & Neovim (Motions & Patterns)
        // =======================
        // Basic Motions
        Hotkey { software: "vim", keys: "h / j / k / l", description: "Move left / down / up / right" },
        Hotkey { software: "vim", keys: "w / W", description: "Move forward to next word / WORD (WORD includes punctuation)" },
        Hotkey { software: "vim", keys: "b / B", description: "Move backward to start of word / WORD" },
        Hotkey { software: "vim", keys: "e / E", description: "Move forward to end of word / WORD" },
        Hotkey { software: "vim", keys: "ge / gE", description: "Move backward to end of word / WORD" },
        Hotkey { software: "vim", keys: "0", description: "Move to start of line" },
        Hotkey { software: "vim", keys: "^", description: "Move to first non-blank character of line" },
        Hotkey { software: "vim", keys: "_", description: "Move to first non-blank character (count lines down)" },
        Hotkey { software: "vim", keys: "$", description: "Move to end of line" },
        Hotkey { software: "vim", keys: "g_", description: "Move to last non-blank character of line" },
        Hotkey { software: "vim", keys: "gg", description: "Move to first line of file" },
        Hotkey { software: "vim", keys: "G", description: "Move to last line of file (or [count]G to line)" },
        Hotkey { software: "vim", keys: ":{n} / {n}G", description: "Go to line number {n}" },
        Hotkey { software: "vim", keys: "{ / }", description: "Move to previous / next paragraph or blank line" },
        Hotkey { software: "vim", keys: "( / )", description: "Move to previous / next sentence" },
        Hotkey { software: "vim", keys: "]] / [[", description: "Move to next / previous section or function start" },
        Hotkey { software: "vim", keys: "][ / []", description: "Move to next / previous section or function end" },
        Hotkey { software: "vim", keys: "%", description: "Jump to matching bracket/brace/paren" },
        Hotkey { software: "vim", keys: "[{ / ]}", description: "Jump to enclosing open / close curly brace" },
        Hotkey { software: "vim", keys: "[( / ])", description: "Jump to enclosing open / close parenthesis" },

        // Character Search Motions
        Hotkey { software: "vim", keys: "f{char}", description: "Find character inline forward (jump on char)" },
        Hotkey { software: "vim", keys: "F{char}", description: "Find character inline backward (jump on char)" },
        Hotkey { software: "vim", keys: "t{char}", description: "Till character inline forward (jump right before char)" },
        Hotkey { software: "vim", keys: "T{char}", description: "Till character inline backward (jump right after char)" },
        Hotkey { software: "vim", keys: ";", description: "Repeat last inline character search forward" },
        Hotkey { software: "vim", keys: ",", description: "Repeat last inline character search backward" },

        // Screen & Viewport Positioning
        Hotkey { software: "vim", keys: "H", description: "Move cursor to High (top line of screen)" },
        Hotkey { software: "vim", keys: "M", description: "Move cursor to Middle line of screen" },
        Hotkey { software: "vim", keys: "L", description: "Move cursor to Low (bottom line of screen)" },
        Hotkey { software: "vim", keys: "zz", description: "Center viewport on cursor line" },
        Hotkey { software: "vim", keys: "zt", description: "Scroll viewport so cursor line is at top" },
        Hotkey { software: "vim", keys: "zb", description: "Scroll viewport so cursor line is at bottom" },
        Hotkey { software: "vim", keys: "Ctrl+e", description: "Scroll viewport down one line (cursor stays)" },
        Hotkey { software: "vim", keys: "Ctrl+y", description: "Scroll viewport up one line (cursor stays)" },
        Hotkey { software: "vim", keys: "Ctrl+d", description: "Scroll viewport half page down" },
        Hotkey { software: "vim", keys: "Ctrl+u", description: "Scroll viewport half page up" },
        Hotkey { software: "vim", keys: "Ctrl+f", description: "Scroll viewport full page down" },
        Hotkey { software: "vim", keys: "Ctrl+b", description: "Scroll viewport full page up" },

        // Text Objects (The Vim Patterns)
        Hotkey { software: "vim", keys: "ciw / caw", description: "Change inside / around word (delete and insert)" },
        Hotkey { software: "vim", keys: "diw / daw", description: "Delete inside / around word" },
        Hotkey { software: "vim", keys: "yiw / yaw", description: "Yank inside / around word" },
        Hotkey { software: "vim", keys: "viw / vaw", description: "Visual select inside / around word" },
        Hotkey { software: "vim", keys: "ci\" / ca\"", description: "Change inside / around double quotes" },
        Hotkey { software: "vim", keys: "di\" / da\"", description: "Delete inside / around double quotes" },
        Hotkey { software: "vim", keys: "yi\" / ya\"", description: "Yank inside / around double quotes" },
        Hotkey { software: "vim", keys: "vi\" / va\"", description: "Visual select inside / around double quotes" },
        Hotkey { software: "vim", keys: "ci' / ca'", description: "Change inside / around single quotes" },
        Hotkey { software: "vim", keys: "di' / da'", description: "Delete inside / around single quotes" },
        Hotkey { software: "vim", keys: "ci` / ca`", description: "Change inside / around backticks" },
        Hotkey { software: "vim", keys: "di` / da`", description: "Delete inside / around backticks" },
        Hotkey { software: "vim", keys: "ci( / ca(", description: "Change inside / around parentheses (or cib / cab)" },
        Hotkey { software: "vim", keys: "di( / da(", description: "Delete inside / around parentheses (or dib / dab)" },
        Hotkey { software: "vim", keys: "yi( / ya(", description: "Yank inside / around parentheses" },
        Hotkey { software: "vim", keys: "vi( / va(", description: "Visual select inside / around parentheses" },
        Hotkey { software: "vim", keys: "ci{ / ca{", description: "Change inside / around curly braces (or ciB / caB)" },
        Hotkey { software: "vim", keys: "di{ / da{", description: "Delete inside / around curly braces (or diB / daB)" },
        Hotkey { software: "vim", keys: "yi{ / ya{", description: "Yank inside / around curly braces" },
        Hotkey { software: "vim", keys: "vi{ / va{", description: "Visual select inside / around curly braces" },
        Hotkey { software: "vim", keys: "ci[ / ca[", description: "Change inside / around square brackets" },
        Hotkey { software: "vim", keys: "di[ / da[", description: "Delete inside / around square brackets" },
        Hotkey { software: "vim", keys: "ci< / ca<", description: "Change inside / around angle brackets" },
        Hotkey { software: "vim", keys: "cit / cat", description: "Change inside / around HTML or XML tags" },
        Hotkey { software: "vim", keys: "dit / dat", description: "Delete inside / around HTML or XML tags" },
        Hotkey { software: "vim", keys: "cip / dap", description: "Change / delete inside paragraph" },
        Hotkey { software: "vim", keys: "cis / das", description: "Change / delete inside sentence" },

        // Editing Operators & Patterns
        Hotkey { software: "vim", keys: "i / I", description: "Insert before cursor / at line start" },
        Hotkey { software: "vim", keys: "a / A", description: "Append after cursor / at line end" },
        Hotkey { software: "vim", keys: "o / O", description: "Open new line below / above and enter insert mode" },
        Hotkey { software: "vim", keys: "x / X", description: "Delete character under / before cursor" },
        Hotkey { software: "vim", keys: "r{char} / R", description: "Replace character under cursor / Enter Replace mode" },
        Hotkey { software: "vim", keys: "s / S", description: "Substitute character / Substitute whole line" },
        Hotkey { software: "vim", keys: "c{motion}", description: "Change text over motion (e.g. cw, c$, c2w)" },
        Hotkey { software: "vim", keys: "C / cc", description: "Change to line end / Change whole line" },
        Hotkey { software: "vim", keys: "d{motion}", description: "Delete text over motion (e.g. dw, d$, dG)" },
        Hotkey { software: "vim", keys: "D / dd", description: "Delete to line end / Delete whole line" },
        Hotkey { software: "vim", keys: "y{motion}", description: "Yank text over motion (e.g. yw, y$, yG)" },
        Hotkey { software: "vim", keys: "Y / yy", description: "Yank to line end / Yank whole line" },
        Hotkey { software: "vim", keys: "p / P", description: "Paste after / before cursor (or below / above line)" },
        Hotkey { software: "vim", keys: "gp / gP", description: "Paste and leave cursor after pasted text" },
        Hotkey { software: "vim", keys: "J / gJ", description: "Join current line with next (with space / without space)" },
        Hotkey { software: "vim", keys: "~", description: "Toggle case of character under cursor" },
        Hotkey { software: "vim", keys: "g~{motion}", description: "Toggle case of text over motion (e.g. g~w)" },
        Hotkey { software: "vim", keys: "gu{motion} / gU{motion}", description: "Lowercase / uppercase text over motion (e.g. guw, gUiw)" },
        Hotkey { software: "vim", keys: "guu / gUU", description: "Lowercase / uppercase current line" },
        Hotkey { software: "vim", keys: ">> / <<", description: "Indent / outdent current line" },
        Hotkey { software: "vim", keys: ">{motion} / <{motion}", description: "Indent / outdent text over motion (e.g. >i{, >ap)" },
        Hotkey { software: "vim", keys: "== / ={motion}", description: "Auto-indent current line / over motion (e.g. =ap)" },
        Hotkey { software: "vim", keys: "gg=G", description: "Auto-indent entire file" },
        Hotkey { software: "vim", keys: "Ctrl+a / Ctrl+x", description: "Increment / decrement number under cursor" },
        Hotkey { software: "vim", keys: "xp", description: "Transpose (swap) character under cursor with next" },
        Hotkey { software: "vim", keys: "ddp", description: "Transpose (swap) current line with line below" },
        Hotkey { software: "vim", keys: ".", description: "Repeat last change/edit command" },
        Hotkey { software: "vim", keys: "u / Ctrl+r", description: "Undo / redo last change" },

        // Search & Substitute Patterns
        Hotkey { software: "vim", keys: "/{pattern}", description: "Search forward for pattern" },
        Hotkey { software: "vim", keys: "?{pattern}", description: "Search backward for pattern" },
        Hotkey { software: "vim", keys: "n / N", description: "Repeat search in same / opposite direction" },
        Hotkey { software: "vim", keys: "* / #", description: "Search forward / backward for exact word under cursor" },
        Hotkey { software: "vim", keys: "g* / g#", description: "Search forward / backward for partial word under cursor" },
        Hotkey { software: "vim", keys: ":s/old/new/g", description: "Replace all occurrences of 'old' with 'new' in current line" },
        Hotkey { software: "vim", keys: ":%s/old/new/g", description: "Replace all occurrences in entire file" },
        Hotkey { software: "vim", keys: ":%s/old/new/gc", description: "Replace in entire file with confirmation prompt (y/n/a/q)" },
        Hotkey { software: "vim", keys: ":noh", description: "Clear search highlighting (:nohlsearch)" },
        Hotkey { software: "vim", keys: "& / :&&", description: "Repeat last substitute command on current line" },

        // Marks, Jumps & History
        Hotkey { software: "vim", keys: "Ctrl+o", description: "Jump backward to older position in jump list" },
        Hotkey { software: "vim", keys: "Ctrl+i", description: "Jump forward to newer position in jump list" },
        Hotkey { software: "vim", keys: "'' / ``", description: "Jump back to position before last jump (line / exact pos)" },
        Hotkey { software: "vim", keys: "g; / g,", description: "Jump backward / forward to position in change list" },
        Hotkey { software: "vim", keys: "gi", description: "Jump to last insert edit position and enter Insert mode" },
        Hotkey { software: "vim", keys: "gv", description: "Reselect previous visual selection" },
        Hotkey { software: "vim", keys: "gf", description: "Go to file path under cursor" },
        Hotkey { software: "vim", keys: "gx", description: "Open file or URL under cursor in system default app" },
        Hotkey { software: "vim", keys: "Ctrl+^", description: "Toggle between current and alternate (last) buffer" },
        Hotkey { software: "vim", keys: "m{a-z}", description: "Set buffer-local mark {a-z} at cursor" },
        Hotkey { software: "vim", keys: "'{a-z} / `{a-z}", description: "Jump to line / exact pos of local mark {a-z}" },
        Hotkey { software: "vim", keys: "m{A-Z}", description: "Set global (cross-file) mark {A-Z}" },
        Hotkey { software: "vim", keys: "'{A-Z}", description: "Jump to global mark {A-Z} across files" },
        Hotkey { software: "vim", keys: ":marks", description: "List all active marks" },

        // Macros & Registers
        Hotkey { software: "vim", keys: "q{a-z}", description: "Record macro into register {a-z}" },
        Hotkey { software: "vim", keys: "q", description: "Stop recording macro" },
        Hotkey { software: "vim", keys: "@{a-z} / @@", description: "Play macro from register {a-z} / Repeat last macro" },
        Hotkey { software: "vim", keys: "\"{reg}y / \"{reg}p", description: "Yank into / paste from named register {reg}" },
        Hotkey { software: "vim", keys: "\"0p", description: "Paste from yank register (ignores deleted text)" },
        Hotkey { software: "vim", keys: "\"+y / \"+p", description: "Yank to / paste from system OS clipboard" },
        Hotkey { software: "vim", keys: "\"*y / \"*p", description: "Yank to / paste from primary selection clipboard" },
        Hotkey { software: "vim", keys: "\"_d", description: "Delete into black hole register (preserves clipboard)" },
        Hotkey { software: "vim", keys: ":reg", description: "Display contents of all registers" },

        // Visual Mode & Block Patterns
        Hotkey { software: "vim", keys: "v / V", description: "Start character-wise / line-wise Visual mode" },
        Hotkey { software: "vim", keys: "Ctrl+v", description: "Start visual block (column) mode" },
        Hotkey { software: "vim", keys: "o / O", description: "Move cursor to other end / other corner of visual selection" },
        Hotkey { software: "vim", keys: "I (block mode)", description: "Insert text at start of each block line (Esc to apply)" },
        Hotkey { software: "vim", keys: "A (block mode)", description: "Append text at end of each block line (Esc to apply)" },
        Hotkey { software: "vim", keys: "c (block mode)", description: "Change column block on selected lines (Esc to apply)" },
        Hotkey { software: "vim", keys: "r{char} (block)", description: "Replace entire rectangular block with {char}" },
        Hotkey { software: "vim", keys: "> / <", description: "Indent / outdent selected lines (in visual mode)" },

        // Window & Tab Splits
        Hotkey { software: "vim", keys: "Ctrl+w s / :sp", description: "Split window horizontally" },
        Hotkey { software: "vim", keys: "Ctrl+w v / :vsp", description: "Split window vertically" },
        Hotkey { software: "vim", keys: "Ctrl+w h/j/k/l", description: "Move focus to window left / down / up / right" },
        Hotkey { software: "vim", keys: "Ctrl+w H/J/K/L", description: "Move window to far left / bottom / top / right" },
        Hotkey { software: "vim", keys: "Ctrl+w w", description: "Cycle focus between split windows" },
        Hotkey { software: "vim", keys: "Ctrl+w q / :q", description: "Close current split window" },
        Hotkey { software: "vim", keys: "Ctrl+w o / :only", description: "Close all windows except current active one" },
        Hotkey { software: "vim", keys: "Ctrl+w =", description: "Make all split windows equal size" },
        Hotkey { software: "vim", keys: "Ctrl+w + / -", description: "Increase / decrease window height" },
        Hotkey { software: "vim", keys: "Ctrl+w > / <", description: "Increase / decrease window width" },
        Hotkey { software: "vim", keys: "Ctrl+w r / R", description: "Rotate split windows clockwise / counter-clockwise" },
        Hotkey { software: "vim", keys: ":tabnew / :tabclose", description: "Open new tab / close current tab" },
        Hotkey { software: "vim", keys: "gt / gT", description: "Go to next / previous tab" },
        Hotkey { software: "vim", keys: ":w / :wq / :q!", description: "Save file / save and quit / quit without saving" },

        // =======================
        // LazyVim Specific
        // =======================
        // Files & Telescope
        Hotkey { software: "lazyvim", keys: "<leader>ff", description: "Find files (Telescope)" },
        Hotkey { software: "lazyvim", keys: "<leader><space>", description: "Find files (Root Dir)" },
        Hotkey { software: "lazyvim", keys: "<leader>fF", description: "Find files in cwd (Telescope)" },
        Hotkey { software: "lazyvim", keys: "<leader>fg", description: "Find git files" },
        Hotkey { software: "lazyvim", keys: "<leader>fr", description: "Recent files" },
        Hotkey { software: "lazyvim", keys: "<leader>fR", description: "Recent files (cwd)" },
        Hotkey { software: "lazyvim", keys: "<leader>fc", description: "Find config files" },
        Hotkey { software: "lazyvim", keys: "<leader>fb", description: "Find buffers" },
        Hotkey { software: "lazyvim", keys: "<leader>,", description: "Switch Buffer" },
        // Search & Grep
        Hotkey { software: "lazyvim", keys: "<leader>sg", description: "Live grep (root dir)" },
        Hotkey { software: "lazyvim", keys: "<leader>sG", description: "Live grep (cwd)" },
        Hotkey { software: "lazyvim", keys: "<leader>/", description: "Grep text (root dir)" },
        Hotkey { software: "lazyvim", keys: "<leader>sw", description: "Search word under cursor (root dir)" },
        Hotkey { software: "lazyvim", keys: "<leader>sW", description: "Search word under cursor (cwd)" },
        Hotkey { software: "lazyvim", keys: "<leader>sb", description: "Search current buffer lines" },
        Hotkey { software: "lazyvim", keys: "<leader>sh", description: "Search help pages" },
        Hotkey { software: "lazyvim", keys: "<leader>sk", description: "Search keymaps" },
        Hotkey { software: "lazyvim", keys: "<leader>sc", description: "Search command history" },
        Hotkey { software: "lazyvim", keys: "<leader>sC", description: "Search commands" },
        Hotkey { software: "lazyvim", keys: "<leader>sd", description: "Search document diagnostics" },
        Hotkey { software: "lazyvim", keys: "<leader>sD", description: "Search workspace diagnostics" },
        Hotkey { software: "lazyvim", keys: "<leader>sm", description: "Jump to mark" },
        Hotkey { software: "lazyvim", keys: "<leader>sR", description: "Resume last search" },
        // Explorer
        Hotkey { software: "lazyvim", keys: "<leader>e", description: "Toggle Neo-tree (root dir)" },
        Hotkey { software: "lazyvim", keys: "<leader>E", description: "Toggle Neo-tree (cwd)" },
        Hotkey { software: "lazyvim", keys: "<leader>fe", description: "Explorer Neo-tree (root dir)" },
        // Buffers
        Hotkey { software: "lazyvim", keys: "<leader>bb", description: "Switch to other buffer" },
        Hotkey { software: "lazyvim", keys: "<leader>bd", description: "Delete / close buffer" },
        Hotkey { software: "lazyvim", keys: "<leader>bD", description: "Delete buffer and window" },
        Hotkey { software: "lazyvim", keys: "<leader>bo", description: "Delete other buffers" },
        Hotkey { software: "lazyvim", keys: "<leader>bl", description: "Delete buffers to the left" },
        Hotkey { software: "lazyvim", keys: "<leader>br", description: "Delete buffers to the right" },
        Hotkey { software: "lazyvim", keys: "<leader>bp", description: "Toggle pin buffer" },
        Hotkey { software: "lazyvim", keys: "<leader>bP", description: "Delete non-pinned buffers" },
        Hotkey { software: "lazyvim", keys: "<S-h> / [b", description: "Previous buffer" },
        Hotkey { software: "lazyvim", keys: "<S-l> / ]b", description: "Next buffer" },
        // Windows
        Hotkey { software: "lazyvim", keys: "<leader>ww", description: "Other window" },
        Hotkey { software: "lazyvim", keys: "<leader>wd", description: "Delete window" },
        Hotkey { software: "lazyvim", keys: "<leader>w-", description: "Split window below" },
        Hotkey { software: "lazyvim", keys: "<leader>w|", description: "Split window right" },
        Hotkey { software: "lazyvim", keys: "<C-h/j/k/l>", description: "Navigate windows" },
        // LSP & Code Navigation
        Hotkey { software: "lazyvim", keys: "gd", description: "Go to definition" },
        Hotkey { software: "lazyvim", keys: "gr", description: "Go to references" },
        Hotkey { software: "lazyvim", keys: "gD", description: "Go to declaration" },
        Hotkey { software: "lazyvim", keys: "gI", description: "Go to implementation" },
        Hotkey { software: "lazyvim", keys: "gy", description: "Go to type definition" },
        Hotkey { software: "lazyvim", keys: "K", description: "Hover documentation" },
        Hotkey { software: "lazyvim", keys: "gK", description: "Signature help" },
        Hotkey { software: "lazyvim", keys: "<c-k>", description: "Signature help (insert mode)" },
        Hotkey { software: "lazyvim", keys: "<leader>ca", description: "Code action" },
        Hotkey { software: "lazyvim", keys: "<leader>cr", description: "Rename symbol" },
        Hotkey { software: "lazyvim", keys: "<leader>cf", description: "Format document" },
        Hotkey { software: "lazyvim", keys: "[d", description: "Previous diagnostic" },
        Hotkey { software: "lazyvim", keys: "]d", description: "Next diagnostic" },
        Hotkey { software: "lazyvim", keys: "gl", description: "Line diagnostics" },
        // Git & Tools
        Hotkey { software: "lazyvim", keys: "<leader>gg", description: "Lazygit (root dir)" },
        Hotkey { software: "lazyvim", keys: "<leader>gG", description: "Lazygit (cwd)" },
        Hotkey { software: "lazyvim", keys: "<leader>l", description: "Lazy plugin manager" },
        Hotkey { software: "lazyvim", keys: "<leader>cm", description: "Mason tool installer" },
        Hotkey { software: "lazyvim", keys: "<leader>qq", description: "Quit all" },
        Hotkey { software: "lazyvim", keys: "<leader>qs", description: "Restore session" },
        Hotkey { software: "lazyvim", keys: "<leader>ql", description: "Restore last session" },
        Hotkey { software: "lazyvim", keys: "<leader>qd", description: "Don't save current session" },
        Hotkey { software: "lazyvim", keys: "<leader>ft", description: "Toggle floating terminal (root dir)" },
        Hotkey { software: "lazyvim", keys: "<leader>fT", description: "Toggle floating terminal (cwd)" },
        Hotkey { software: "lazyvim", keys: "<c-/>", description: "Toggle terminal (root dir)" },
        Hotkey { software: "lazyvim", keys: "<leader>uC", description: "Select colorscheme" },
        Hotkey { software: "lazyvim", keys: "<leader>uh", description: "Toggle Inlay Hints" },
        Hotkey { software: "lazyvim", keys: "<leader>un", description: "Dismiss all notifications" },

        // =======================
        // tmux
        // =======================
        Hotkey { software: "tmux", keys: "Ctrl+b", description: "Default prefix key" },
        Hotkey { software: "tmux", keys: "Prefix + c", description: "Create new window" },
        Hotkey { software: "tmux", keys: "Prefix + d", description: "Detach session" },
        Hotkey { software: "tmux", keys: "Prefix + s", description: "List sessions interactively" },
        Hotkey { software: "tmux", keys: "Prefix + w", description: "List windows and sessions interactively" },
        Hotkey { software: "tmux", keys: "Prefix + n", description: "Next window" },
        Hotkey { software: "tmux", keys: "Prefix + p", description: "Previous window" },
        Hotkey { software: "tmux", keys: "Prefix + 0..9", description: "Select window by number" },
        Hotkey { software: "tmux", keys: "Prefix + &", description: "Kill current window" },
        Hotkey { software: "tmux", keys: "Prefix + ,", description: "Rename current window" },
        Hotkey { software: "tmux", keys: "Prefix + $", description: "Rename current session" },
        Hotkey { software: "tmux", keys: "Prefix + %", description: "Split window vertically (left/right)" },
        Hotkey { software: "tmux", keys: "Prefix + \"", description: "Split window horizontally (top/bottom)" },
        Hotkey { software: "tmux", keys: "Prefix + x", description: "Kill current pane" },
        Hotkey { software: "tmux", keys: "Prefix + z", description: "Toggle pane zoom (maximize/restore)" },
        Hotkey { software: "tmux", keys: "Prefix + <Arrow>", description: "Switch focus to pane in direction" },
        Hotkey { software: "tmux", keys: "Prefix + Ctrl+<Arrow>", description: "Resize pane in direction" },
        Hotkey { software: "tmux", keys: "Prefix + o", description: "Rotate through panes" },
        Hotkey { software: "tmux", keys: "Prefix + ;", description: "Toggle between current and previous pane" },
        Hotkey { software: "tmux", keys: "Prefix + {", description: "Swap pane left" },
        Hotkey { software: "tmux", keys: "Prefix + }", description: "Swap pane right" },
        Hotkey { software: "tmux", keys: "Prefix + q", description: "Show pane numbers" },
        Hotkey { software: "tmux", keys: "Prefix + !", description: "Break pane into new window" },
        Hotkey { software: "tmux", keys: "Prefix + [", description: "Enter copy / scrollback mode" },
        Hotkey { software: "tmux", keys: "Prefix + ]", description: "Paste from tmux buffer" },
        Hotkey { software: "tmux", keys: "Prefix + t", description: "Show large digital clock" },
        Hotkey { software: "tmux", keys: "Prefix + ?", description: "List all keybindings" },

        // =======================
        // Terminal (Bash, Zsh, Readline & Windows Terminal)
        // =======================
        // Line Navigation (Readline / Bash / Zsh)
        Hotkey { software: "terminal", keys: "Ctrl+a", description: "Move cursor to beginning of line (Home)" },
        Hotkey { software: "terminal", keys: "Ctrl+e", description: "Move cursor to end of line (End)" },
        Hotkey { software: "terminal", keys: "Alt+f / Ctrl+Right", description: "Move cursor forward one word" },
        Hotkey { software: "terminal", keys: "Alt+b / Ctrl+Left", description: "Move cursor backward one word" },
        Hotkey { software: "terminal", keys: "Ctrl+f / Ctrl+b", description: "Move cursor forward / backward one character" },
        Hotkey { software: "terminal", keys: "Ctrl+x Ctrl+x", description: "Toggle cursor between line start and current position" },
        // Text Editing & Deletion
        Hotkey { software: "terminal", keys: "Ctrl+u", description: "Cut (kill) line from cursor to beginning" },
        Hotkey { software: "terminal", keys: "Ctrl+k", description: "Cut (kill) line from cursor to end" },
        Hotkey { software: "terminal", keys: "Ctrl+w", description: "Cut (kill) word before cursor (whitespace-delimited)" },
        Hotkey { software: "terminal", keys: "Alt+d", description: "Cut (kill) word after cursor" },
        Hotkey { software: "terminal", keys: "Alt+Backspace", description: "Cut (kill) word before cursor (delimiter-aware)" },
        Hotkey { software: "terminal", keys: "Ctrl+y", description: "Paste (yank) last cut text at cursor" },
        Hotkey { software: "terminal", keys: "Alt+y", description: "Rotate through kill-ring after pasting with Ctrl+y" },
        Hotkey { software: "terminal", keys: "Ctrl+d", description: "Delete character under cursor (or exit shell if line empty)" },
        Hotkey { software: "terminal", keys: "Ctrl+h", description: "Delete character before cursor (same as Backspace)" },
        Hotkey { software: "terminal", keys: "Ctrl+t", description: "Transpose (swap) character before cursor with current" },
        Hotkey { software: "terminal", keys: "Alt+t", description: "Transpose (swap) last two words" },
        Hotkey { software: "terminal", keys: "Alt+u", description: "Uppercase word from cursor to end of word" },
        Hotkey { software: "terminal", keys: "Alt+l", description: "Lowercase word from cursor to end of word" },
        Hotkey { software: "terminal", keys: "Alt+c", description: "Capitalize word from cursor to end of word" },
        Hotkey { software: "terminal", keys: "Ctrl+_ / Ctrl+x Ctrl+u", description: "Undo last edit on command line" },
        // History & Search
        Hotkey { software: "terminal", keys: "Ctrl+r", description: "Reverse incremental search command history" },
        Hotkey { software: "terminal", keys: "Ctrl+s", description: "Forward search command history" },
        Hotkey { software: "terminal", keys: "Ctrl+g", description: "Abort history search and restore original line" },
        Hotkey { software: "terminal", keys: "Ctrl+p / Up", description: "Previous command in history" },
        Hotkey { software: "terminal", keys: "Ctrl+n / Down", description: "Next command in history" },
        Hotkey { software: "terminal", keys: "Alt+. / Esc .", description: "Insert last argument of previous command (!$) " },
        Hotkey { software: "terminal", keys: "Alt+_", description: "Insert first argument of previous command (!^)" },
        Hotkey { software: "terminal", keys: "Ctrl+o", description: "Execute current command and fetch next from history" },
        Hotkey { software: "terminal", keys: "!!", description: "Repeat previous command (e.g. sudo !!)" },
        Hotkey { software: "terminal", keys: "!$", description: "Expand to last argument of previous command" },
        Hotkey { software: "terminal", keys: "!*", description: "Expand to all arguments of previous command" },
        Hotkey { software: "terminal", keys: "!^", description: "Expand to first argument of previous command" },
        Hotkey { software: "terminal", keys: "!{prefix}", description: "Execute most recent command starting with {prefix}" },
        Hotkey { software: "terminal", keys: "!?{string}", description: "Execute most recent command containing {string}" },
        // Process Control & Signals
        Hotkey { software: "terminal", keys: "Ctrl+c", description: "Interrupt / terminate foreground process (SIGINT)" },
        Hotkey { software: "terminal", keys: "Ctrl+z", description: "Suspend foreground process to background (SIGTSTP)" },
        Hotkey { software: "terminal", keys: "Ctrl+d", description: "Send EOF (End of File) / exit shell session" },
        Hotkey { software: "terminal", keys: "Ctrl+\\", description: "Quit process with core dump (SIGQUIT)" },
        Hotkey { software: "terminal", keys: "Ctrl+s", description: "Pause (freeze) terminal display output (XOFF)" },
        Hotkey { software: "terminal", keys: "Ctrl+q", description: "Resume terminal display output (XON)" },
        Hotkey { software: "terminal", keys: "fg / bg", description: "Resume suspended job in foreground / background" },
        Hotkey { software: "terminal", keys: "jobs", description: "List all active background and suspended jobs" },
        // Screen & Shell Utilities
        Hotkey { software: "terminal", keys: "Ctrl+l", description: "Clear screen (preserves current command line)" },
        Hotkey { software: "terminal", keys: "Tab", description: "Autocomplete command, argument, or file path" },
        Hotkey { software: "terminal", keys: "Tab Tab", description: "Show all available completion candidates" },
        Hotkey { software: "terminal", keys: "Ctrl+x Ctrl+e", description: "Open current command line in $EDITOR (vim/nano)" },
        Hotkey { software: "terminal", keys: "reset", description: "Reinitialize terminal settings and fix broken display" },
        // Terminal Window & Tabs (Windows Terminal / Linux / macOS)
        Hotkey { software: "terminal", keys: "Ctrl+Shift+t", description: "Open new tab" },
        Hotkey { software: "terminal", keys: "Ctrl+Shift+w", description: "Close current tab" },
        Hotkey { software: "terminal", keys: "Ctrl+Shift+n", description: "Open new terminal window" },
        Hotkey { software: "terminal", keys: "Ctrl+Tab / Ctrl+Shift+Tab", description: "Switch to next / previous tab" },
        Hotkey { software: "terminal", keys: "Ctrl+Shift+1..9", description: "Switch directly to tab 1 through 9" },
        Hotkey { software: "terminal", keys: "Alt+Shift+d", description: "Split pane automatically (duplicate profile)" },
        Hotkey { software: "terminal", keys: "Alt+Shift++", description: "Split pane horizontally" },
        Hotkey { software: "terminal", keys: "Alt+Shift+-", description: "Split pane vertically" },
        Hotkey { software: "terminal", keys: "Alt+Arrow", description: "Move focus between split panes" },
        Hotkey { software: "terminal", keys: "Alt+Shift+Arrow", description: "Resize focused pane in direction" },
        Hotkey { software: "terminal", keys: "Ctrl+Shift+f", description: "Find in terminal output" },
        Hotkey { software: "terminal", keys: "Ctrl+Shift+c", description: "Copy selected text to clipboard" },
        Hotkey { software: "terminal", keys: "Ctrl+Shift+v", description: "Paste from clipboard into terminal" },
        Hotkey { software: "terminal", keys: "Shift+Insert", description: "Paste from clipboard into terminal" },
        Hotkey { software: "terminal", keys: "Shift+PgUp / Shift+PgDn", description: "Scroll terminal output buffer up / down" },
    ];
}

const SOFTWARES: &[&str] = &["all", "vscode", "vim", "lazyvim", "tmux", "terminal"];

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Search,
    SelectSoftware,
}

#[allow(non_snake_case)]
fn App() -> Element {
    let mut query = use_signal(|| String::new());
    let mut selected_software = use_signal(|| "all".to_string());
    let mut mode = use_signal(|| Mode::Search);
    let mut selected_index = use_signal(|| 0_usize);
    let copied_id = use_signal(|| Option::<usize>::None);
    
    let mut software_query = use_signal(|| String::new());
    let mut sw_selected_index = use_signal(|| 0_usize);

    let filtered_indices = use_memo(move || {
        let matcher = SkimMatcherV2::default();
        let q = query();
        let sw = selected_software();
        let mut matches = Vec::new();

        for (i, hotkey) in ALL_HOTKEYS.iter().enumerate() {
            if sw != "all" {
                if sw == "lazyvim" {
                    if hotkey.software != "lazyvim" && hotkey.software != "vim" {
                        continue;
                    }
                } else if hotkey.software != sw {
                    continue;
                }
            }

            if q.is_empty() {
                matches.push((i, 0));
            } else {
                let target = format!("{} {} {}", hotkey.software, hotkey.keys, hotkey.description);
                if let Some(score) = matcher.fuzzy_match(&target, &q) {
                    matches.push((i, score));
                }
            }
        }

        if !q.is_empty() {
            matches.sort_by(|a, b| b.1.cmp(&a.1));
        }
        matches
    });

    let filtered_softwares = use_memo(move || {
        let matcher = SkimMatcherV2::default();
        let sq = software_query();
        let mut matches = Vec::new();
        if sq.is_empty() {
            return SOFTWARES.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        } else {
            for sw in SOFTWARES.iter() {
                if let Some(score) = matcher.fuzzy_match(sw, &sq) {
                    matches.push((sw.to_string(), score));
                }
            }
            matches.sort_by(|a, b| b.1.cmp(&a.1));
            matches.into_iter().map(|(s, _)| s).collect::<Vec<_>>()
        }
    });

    let fi_len = filtered_indices().len();
    if fi_len > 0 && selected_index() >= fi_len {
        selected_index.set(0);
    }
    
    let fs_len = filtered_softwares().len();
    if fs_len > 0 && sw_selected_index() >= fs_len {
        sw_selected_index.set(0);
    }

    let copy_shortcut = move |idx: usize, keys: &'static str| {
        let clean_keys = keys.replace('\\', "\\\\").replace('\'', "\\'");
        let js = format!("navigator.clipboard.writeText('{}');", clean_keys);
        let _ = document::eval(&js);
        let mut cid = copied_id;
        cid.set(Some(idx));
    };

    let onkeydown = move |evt: Event<KeyboardData>| {
        let key = evt.key();
        
        match mode() {
            Mode::Search => {
                match key.to_string().as_str() {
                    "ArrowDown" => {
                        evt.prevent_default();
                        let fi_len = filtered_indices().len();
                        if fi_len > 0 {
                            let curr = selected_index();
                            selected_index.set(if curr >= fi_len - 1 { 0 } else { curr + 1 });
                        }
                    }
                    "ArrowUp" => {
                        evt.prevent_default();
                        let fi_len = filtered_indices().len();
                        if fi_len > 0 {
                            let curr = selected_index();
                            selected_index.set(if curr == 0 { fi_len - 1 } else { curr - 1 });
                        }
                    }
                    "Enter" => {
                        evt.prevent_default();
                        let fi = filtered_indices();
                        if let Some(&(i, _)) = fi.get(selected_index()) {
                            let hotkey = &ALL_HOTKEYS[i];
                            copy_shortcut(i, hotkey.keys);
                        }
                    }
                    "Escape" => {
                        if !query().is_empty() {
                            query.set(String::new());
                            selected_index.set(0);
                        }
                    }
                    ":" if query().is_empty() => {
                        evt.prevent_default();
                        mode.set(Mode::SelectSoftware);
                        software_query.set(String::new());
                        sw_selected_index.set(0);
                    }
                    _ => {}
                }
            }
            Mode::SelectSoftware => {
                match key.to_string().as_str() {
                    "Escape" => {
                        evt.prevent_default();
                        mode.set(Mode::Search);
                    }
                    "ArrowDown" => {
                        evt.prevent_default();
                        let fs_len = filtered_softwares().len();
                        if fs_len > 0 {
                            let curr = sw_selected_index();
                            sw_selected_index.set(if curr >= fs_len - 1 { 0 } else { curr + 1 });
                        }
                    }
                    "ArrowUp" => {
                        evt.prevent_default();
                        let fs_len = filtered_softwares().len();
                        if fs_len > 0 {
                            let curr = sw_selected_index();
                            sw_selected_index.set(if curr == 0 { fs_len - 1 } else { curr - 1 });
                        }
                    }
                    "Tab" => {
                        evt.prevent_default();
                        let fs = filtered_softwares();
                        let curr = sw_selected_index();
                        if let Some(sw) = fs.get(curr) {
                            software_query.set(sw.clone());
                        }
                    }
                    "Enter" => {
                        evt.prevent_default();
                        let fs = filtered_softwares();
                        let curr = sw_selected_index();
                        if let Some(sw) = fs.get(curr) {
                            selected_software.set(sw.clone());
                            mode.set(Mode::Search);
                            query.set(String::new());
                            selected_index.set(0);
                        }
                    }
                    _ => {}
                }
            }
        }
    };

    let fi = filtered_indices();
    let fs = filtered_softwares();
    let total_count = ALL_HOTKEYS.len();
    let matched_count = fi.len();

    rsx! {
        style {
            r#"
            :root {{
                --theme-bg-outer: #181818;
                --theme-bg-page: #1e1e1e;
                --theme-bg-panel: #242424;
                --theme-bg-active: #2a2a2a;
                --theme-border: #363636;
                --theme-border-subtle: #2c2c2c;

                --theme-accent: #d60645;
                --theme-accent-bright: #de1b54;
                --theme-accent-subtle: rgba(214, 6, 69, 0.14);
                --theme-accent-glow: rgba(214, 6, 69, 0.4);

                --theme-success: #a6d24b;
                --theme-success-subtle: rgba(166, 210, 75, 0.15);
                --theme-danger: #e0566f;
                --theme-danger-subtle: rgba(224, 86, 111, 0.15);
                --theme-danger-hover: #ef7288;
                --theme-warning: #d9b23a;
                --theme-warning-subtle: rgba(217, 178, 58, 0.15);
                --theme-info: #7aa2c7;
                --theme-info-subtle: rgba(122, 162, 199, 0.15);
                --theme-violet: #9b87c4;
                --theme-teal: #6fbfa4;

                --text-bright: #bdbcbc;
                --text-normal: #a8a8a8;
                --text-muted: #787878;
                --text-dim: #555555;
                --text-link: #d60645;

                --font-interface: 'Inter', -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
                --font-monospace: 'JetBrains Mono', monospace;
            }}

            * {{
                box-sizing: border-box;
                margin: 0;
                padding: 0;
            }}

            html, body {{
                width: 100%;
                height: 100%;
                background-color: var(--theme-bg-outer);
                color: var(--text-normal);
                font-family: var(--font-interface);
                font-size: 13px;
                line-height: 1.5;
                overflow: hidden;
                -webkit-font-smoothing: antialiased;
            }}

            /* Custom Sleek Scrollbar */
            ::-webkit-scrollbar {{
                width: 10px;
                height: 10px;
            }}
            ::-webkit-scrollbar-track {{
                background: transparent;
            }}
            ::-webkit-scrollbar-thumb {{
                background-color: var(--theme-border);
                border-radius: 5px;
                border: 2px solid var(--theme-bg-panel);
                background-clip: padding-box;
            }}
            ::-webkit-scrollbar-thumb:hover {{
                background-color: var(--text-muted);
            }}

            /* Material Symbols Sharp */
            .material-symbols-sharp {{
                font-family: 'Material Symbols Sharp';
                font-weight: normal;
                font-style: normal;
                font-size: 18px;
                line-height: 1;
                letter-spacing: normal;
                text-transform: none;
                display: inline-flex;
                align-items: center;
                justify-content: center;
                white-space: nowrap;
                user-select: none;
                vertical-align: middle;
                flex-shrink: 0;
            }}

            /* Outlineless Filter Buttons */
            .filter-tab-btn {{
                background: transparent;
                border: none;
                outline: none;
                color: var(--text-muted);
                font-family: var(--font-interface);
                font-size: 11px;
                font-weight: 500;
                text-transform: uppercase;
                letter-spacing: 0.3px;
                padding: 4px 10px;
                border-radius: 11px;
                cursor: pointer;
                display: inline-flex;
                align-items: center;
                gap: 5px;
                transition: color 0.15s ease, background-color 0.15s ease;
                white-space: nowrap;
                user-select: none;
            }}
            .filter-tab-btn:hover {{
                color: #ffffff;
            }}
            .filter-tab-btn.active {{
                background-color: var(--theme-accent-subtle);
                color: var(--theme-accent);
                font-weight: 600;
            }}

            /* Hotkey List Item */
            .hotkey-row {{
                display: flex;
                align-items: center;
                padding: 8px 16px;
                border-bottom: 1px solid var(--theme-border-subtle);
                transition: background-color 0.12s ease;
                cursor: pointer;
                font-size: 12px;
                position: relative;
                user-select: none;
            }}
            .hotkey-row:hover {{
                background-color: var(--theme-bg-active);
            }}
            .hotkey-row.selected {{
                background-color: var(--theme-bg-panel);
                box-shadow: inset 3px 0 0 var(--theme-accent);
            }}

            /* Keyboard chip */
            .key-chip {{
                font-family: var(--font-monospace);
                font-size: 11px;
                font-weight: 500;
                color: var(--text-bright);
                background-color: var(--theme-bg-outer);
                border: 1px solid var(--theme-border);
                border-radius: 3px;
                padding: 2px 7px;
                display: inline-flex;
                align-items: center;
                transition: border-color 0.15s ease, color 0.15s ease;
            }}
            .hotkey-row.selected .key-chip {{
                border-color: var(--theme-accent);
                color: var(--theme-accent-bright);
            }}

            /* Action icon button */
            .icon-btn {{
                background: transparent;
                border: none;
                outline: none;
                color: var(--text-muted);
                cursor: pointer;
                padding: 4px;
                border-radius: 3px;
                display: inline-flex;
                align-items: center;
                justify-content: center;
                transition: color 0.15s ease;
            }}
            .icon-btn:hover {{
                color: #ffffff;
            }}

            .cursor {{
                display: inline-block;
                width: 7px;
                height: 1.1em;
                background-color: var(--theme-accent);
                vertical-align: text-bottom;
                animation: blink 1s step-end infinite;
            }}
            @keyframes blink {{ 50% {{ opacity: 0; }} }}
            "#
        }

        div {
            tabindex: 0,
            autofocus: true,
            onkeydown: onkeydown,
            style: "width: 100%; height: 100%; display: flex; flex-direction: column; outline: none; background-color: var(--theme-bg-outer);",

            // 1. BRAND HEADER BAR (Section 2 of NullDark Spec)
            header {
                style: "height: 44px; background-color: var(--theme-bg-outer); border-bottom: 1px solid var(--theme-border); padding: 0 16px; display: flex; align-items: center; justify-content: space-between; flex-shrink: 0; user-select: none;",
                
                // Left Brand Anchor
                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    img {
                        src: "icon.svg",
                        style: "height: 26px; width: auto; object-fit: contain; display: block; border: none; background: transparent;",
                        alt: "NullDark Mark",
                    }
                    div {
                        style: "display: flex; align-items: baseline;",
                        span { style: "font-size: 13px; font-weight: 700; color: var(--text-muted); letter-spacing: 0.2px;", "NULL" }
                        span { style: "font-size: 13px; font-weight: 700; color: var(--text-bright); letter-spacing: 0.2px;", "HOTKEYS" }
                    }
                    span { style: "color: var(--theme-border); font-size: 13px; font-weight: 300; margin: 0 4px;", "|" }
                    span { style: "font-size: 11px; font-weight: 500; text-transform: uppercase; letter-spacing: 0.3px; color: var(--text-muted);", "KEYBOARD SHORTCUT WORKSPACE" }
                }

                // Right Status Indicators
                div {
                    style: "display: flex; align-items: center; gap: 14px;",
                    
                    // Boundary-Free Status Indicator
                    div {
                        style: "display: flex; align-items: center; gap: 6px; font-size: 11px; color: var(--text-normal); letter-spacing: 0.2px;",
                        span { style: "width: 7px; height: 7px; border-radius: 50%; background-color: var(--theme-success); box-shadow: 0 0 6px rgba(166, 210, 75, 0.4); display: inline-block;" }
                        span { "OPERATIONAL" }
                    }

                    // Tinted Status Pill
                    div {
                        style: "height: 22px; border-radius: 11px; padding: 0 10px; background: var(--theme-accent-subtle); color: var(--theme-accent); font-size: 11px; font-weight: 600; display: inline-flex; align-items: center; border: none;",
                        "{matched_count} SHORTCUTS"
                    }
                }
            }

            // 2. SEARCH & FILTER TOOLBAR
            div {
                style: "background-color: var(--theme-bg-outer); border-bottom: 1px solid var(--theme-border); padding: 8px 16px; display: flex; align-items: center; justify-content: space-between; gap: 16px; flex-shrink: 0;",
                
                // Search Input Box
                div {
                    style: "position: relative; flex: 1; max-width: 480px; display: flex; align-items: center;",
                    span {
                        class: "material-symbols-sharp",
                        style: "position: absolute; left: 10px; font-size: 18px; color: var(--text-muted); pointer-events: none;",
                        "search"
                    }
                    input {
                        r#type: "text",
                        autofocus: true,
                        style: "background: var(--theme-bg-panel); border: 1px solid var(--theme-border); border-radius: 4px; padding: 6px 36px 6px 34px; color: var(--text-bright); font-size: 12px; font-family: var(--font-interface); width: 100%; outline: none; transition: border-color 0.15s, box-shadow 0.15s;",
                        placeholder: "Search shortcuts (e.g. format, git, pane, buffer)...",
                        value: "{query()}",
                        oninput: move |evt| {
                            query.set(evt.value());
                            selected_index.set(0);
                        },
                    }
                    if !query().is_empty() {
                        button {
                            class: "icon-btn",
                            style: "position: absolute; right: 6px;",
                            onclick: move |_| {
                                query.set(String::new());
                            },
                            span { class: "material-symbols-sharp", style: "font-size: 16px;", "close" }
                        }
                    } else {
                        span {
                            style: "position: absolute; right: 8px; font-size: 10px; color: var(--text-dim); background: var(--theme-bg-outer); border: 1px solid var(--theme-border); border-radius: 3px; padding: 1px 5px; font-family: var(--font-monospace); pointer-events: none;",
                            ":"
                        }
                    }
                }

                // Pipe-separated Software Pills
                div {
                    style: "display: flex; align-items: center; gap: 4px; flex-wrap: wrap;",
                    
                    button {
                        class: if selected_software() == "all" { "filter-tab-btn active" } else { "filter-tab-btn" },
                        onclick: move |_| {
                            selected_software.set("all".to_string());
                            selected_index.set(0);
                        },
                        "ALL"
                    }
                    span { style: "color: var(--theme-border); font-size: 12px; margin: 0 2px;", "|" }
                    button {
                        class: if selected_software() == "vscode" { "filter-tab-btn active" } else { "filter-tab-btn" },
                        onclick: move |_| {
                            selected_software.set("vscode".to_string());
                            selected_index.set(0);
                        },
                        "VS CODE"
                    }
                    span { style: "color: var(--theme-border); font-size: 12px; margin: 0 2px;", "|" }
                    button {
                        class: if selected_software() == "vim" { "filter-tab-btn active" } else { "filter-tab-btn" },
                        onclick: move |_| {
                            selected_software.set("vim".to_string());
                            selected_index.set(0);
                        },
                        "VIM"
                    }
                    span { style: "color: var(--theme-border); font-size: 12px; margin: 0 2px;", "|" }
                    button {
                        class: if selected_software() == "lazyvim" { "filter-tab-btn active" } else { "filter-tab-btn" },
                        onclick: move |_| {
                            selected_software.set("lazyvim".to_string());
                            selected_index.set(0);
                        },
                        "LAZYVIM"
                    }
                    span { style: "color: var(--theme-border); font-size: 12px; margin: 0 2px;", "|" }
                    button {
                        class: if selected_software() == "tmux" { "filter-tab-btn active" } else { "filter-tab-btn" },
                        onclick: move |_| {
                            selected_software.set("tmux".to_string());
                            selected_index.set(0);
                        },
                        "TMUX"
                    }
                    span { style: "color: var(--theme-border); font-size: 12px; margin: 0 2px;", "|" }
                    button {
                        class: if selected_software() == "terminal" { "filter-tab-btn active" } else { "filter-tab-btn" },
                        onclick: move |_| {
                            selected_software.set("terminal".to_string());
                            selected_index.set(0);
                        },
                        "TERMINAL"
                    }
                    span { style: "color: var(--theme-border); font-size: 12px; margin: 0 2px;", "|" }
                    button {
                        class: "filter-tab-btn",
                        onclick: move |_| {
                            mode.set(Mode::SelectSoftware);
                            software_query.set(String::new());
                            sw_selected_index.set(0);
                        },
                        span { class: "material-symbols-sharp", style: "font-size: 14px; margin-right: 2px;", "tune" }
                        "SWITCH (:)"
                    }
                }
            }

            // 3. MAIN WORKSPACE CANVAS
            div {
                style: "background-color: var(--theme-bg-page); flex: 1; display: flex; flex-direction: column; overflow: hidden; position: relative;",

                // Table Header
                div {
                    style: "background-color: var(--theme-bg-panel); height: 32px; display: flex; align-items: center; padding: 0 16px; border-bottom: 1px solid var(--theme-border); font-size: 11px; font-weight: 600; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.5px; user-select: none; flex-shrink: 0;",
                    div { style: "width: 100px; flex-shrink: 0;", "SOFTWARE" }
                    div { style: "width: 270px; flex-shrink: 0;", "KEY COMBINATION" }
                    div { style: "flex: 1;", "ACTION / DESCRIPTION" }
                    div { style: "width: 60px; text-align: right;", "COPY" }
                }

                // Table Rows
                div {
                    style: "flex: 1; overflow-y: auto; overflow-x: hidden;",

                    if fi.is_empty() {
                        div {
                            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; gap: 10px; color: var(--text-muted); user-select: none;",
                            span { class: "material-symbols-sharp", style: "font-size: 36px; color: var(--text-dim);", "search_off" }
                            div { style: "font-size: 13px; color: var(--text-bright); font-weight: 500;", "No shortcuts found" }
                            div { style: "font-size: 11px; color: var(--text-muted);", "Try another keyword or press ':' to change software" }
                        }
                    } else {
                        {
                            fi.iter().enumerate().map(|(idx, &(i, _))| {
                                let hotkey = &ALL_HOTKEYS[i];
                                let is_selected = mode() == Mode::Search && idx == selected_index();
                                let is_copied = copied_id() == Some(i);
                                let row_class = if is_selected { "hotkey-row selected" } else { "hotkey-row" };
                                let keys_text = hotkey.keys;
                                
                                rsx! {
                                    div {
                                        key: "{i}",
                                        class: "{row_class}",
                                        onclick: move |_| {
                                            selected_index.set(idx);
                                            copy_shortcut(i, keys_text);
                                        },

                                        // Software Tag
                                        div {
                                            style: "width: 100px; flex-shrink: 0;",
                                            span {
                                                style: "font-size: 10px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.4px; color: var(--text-muted); background: var(--theme-bg-outer); padding: 2px 6px; border-radius: 3px;",
                                                "{hotkey.software}"
                                            }
                                        }

                                        // Key Combination Chip
                                        div {
                                            style: "width: 270px; flex-shrink: 0; padding-right: 12px; display: flex; align-items: center; gap: 4px;",
                                            span {
                                                class: "key-chip",
                                                "{hotkey.keys}"
                                            }
                                        }

                                        // Description
                                        div {
                                            style: "flex: 1; color: var(--text-normal); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-size: 12px;",
                                            "{hotkey.description}"
                                        }

                                        // Copy Action
                                        div {
                                            style: "width: 60px; text-align: right;",
                                            if is_copied {
                                                span {
                                                    style: "font-size: 11px; color: var(--theme-success); font-weight: 500; display: inline-flex; align-items: center; gap: 2px;",
                                                    span { class: "material-symbols-sharp", style: "font-size: 14px;", "check" }
                                                    "COPIED"
                                                }
                                            } else {
                                                button {
                                                    class: "icon-btn",
                                                    title: "Copy shortcut",
                                                    onclick: move |evt| {
                                                        evt.stop_propagation();
                                                        copy_shortcut(i, keys_text);
                                                    },
                                                    span { class: "material-symbols-sharp", style: "font-size: 16px;", "content_copy" }
                                                }
                                            }
                                        }
                                    }
                                }
                            })
                        }
                    }
                }
            }

            // 4. SOFTWARE SELECTION MODAL
            if mode() == Mode::SelectSoftware {
                div {
                    style: "position: fixed; inset: 0; background: rgba(0, 0, 0, 0.7); backdrop-filter: blur(4px); display: flex; align-items: center; justify-content: center; z-index: 1000; user-select: none;",
                    onclick: move |_| {
                        mode.set(Mode::Search);
                    },
                    div {
                        style: "width: 440px; background-color: var(--theme-bg-panel); border-radius: 6px; box-shadow: 0 20px 50px rgba(0, 0, 0, 0.8); overflow: hidden; display: flex; flex-direction: column; border: none;",
                        onclick: move |evt| {
                            evt.stop_propagation();
                        },

                        // Modal Header
                        div {
                            style: "padding: 12px 16px; border-bottom: 1px solid var(--theme-border); display: flex; align-items: center; justify-content: space-between;",
                            div {
                                style: "display: flex; align-items: center; gap: 6px;",
                                span { class: "material-symbols-sharp", style: "font-size: 16px; color: var(--theme-accent);", "filter_list" }
                                span { style: "font-size: 13px; font-weight: 600; color: var(--text-bright);", "Filter by Software" }
                            }
                            button {
                                class: "icon-btn",
                                onclick: move |_| mode.set(Mode::Search),
                                span { class: "material-symbols-sharp", style: "font-size: 16px;", "close" }
                            }
                        }

                        // Search Input
                        div {
                            style: "padding: 12px 16px; border-bottom: 1px solid var(--theme-border-subtle);",
                            div {
                                style: "position: relative; display: flex; align-items: center;",
                                span {
                                    style: "position: absolute; left: 10px; font-size: 12px; color: var(--theme-accent); font-family: var(--font-monospace);",
                                    ":"
                                }
                                input {
                                    r#type: "text",
                                    autofocus: true,
                                    style: "width: 100%; background: var(--theme-bg-outer); border: 1px solid var(--theme-border); border-radius: 4px; padding: 6px 12px 6px 24px; color: var(--text-bright); font-size: 12px; outline: none;",
                                    placeholder: "Type software name (Tab to complete, Enter to select)...",
                                    value: "{software_query()}",
                                    oninput: move |evt| {
                                        software_query.set(evt.value());
                                        sw_selected_index.set(0);
                                    },
                                }
                            }
                        }

                        // Software List
                        div {
                            style: "max-height: 220px; overflow-y: auto; padding: 4px 0;",
                            {
                                fs.iter().enumerate().map(|(idx, sw)| {
                                    let sw_str = sw.clone();
                                    let is_active = idx == sw_selected_index();
                                    let is_current = selected_software() == *sw;
                                    let item_style = if is_active {
                                        "padding: 8px 16px; display: flex; align-items: center; justify-content: space-between; font-size: 12px; background: var(--theme-bg-active); color: var(--theme-accent); font-weight: 600; cursor: pointer;"
                                    } else {
                                        "padding: 8px 16px; display: flex; align-items: center; justify-content: space-between; font-size: 12px; color: var(--text-normal); cursor: pointer;"
                                    };

                                    rsx! {
                                        div {
                                            key: "{sw}",
                                            style: "{item_style}",
                                            onclick: move |_| {
                                                selected_software.set(sw_str.clone());
                                                mode.set(Mode::Search);
                                                query.set(String::new());
                                                selected_index.set(0);
                                            },
                                            div {
                                                style: "display: flex; align-items: center; gap: 8px;",
                                                if is_current {
                                                    span { class: "material-symbols-sharp", style: "font-size: 16px; color: var(--theme-accent);", "check" }
                                                } else {
                                                    span { style: "width: 16px; display: inline-block;" }
                                                }
                                                span { style: "text-transform: uppercase;", "{sw}" }
                                            }
                                            if is_current {
                                                span { style: "font-size: 10px; color: var(--text-muted);", "ACTIVE" }
                                            }
                                        }
                                    }
                                })
                            }
                        }
                    }
                }
            }

            // 5. STATUS FOOTER BAR (Section 6 of NullDark Spec)
            footer {
                style: "height: 28px; background-color: var(--theme-bg-outer); border-top: 1px solid var(--theme-border); padding: 0 16px; display: flex; align-items: center; justify-content: space-between; font-size: 11px; color: var(--text-muted); user-select: none; flex-shrink: 0;",
                
                // Keyboard helpers
                div {
                    style: "display: flex; align-items: center; gap: 14px;",
                    span {
                        span { style: "font-family: var(--font-monospace); color: var(--text-bright);", "↑↓ " }
                        "Navigate"
                    }
                    span {
                        span { style: "font-family: var(--font-monospace); color: var(--text-bright);", "Enter " }
                        "Copy Shortcut"
                    }
                    span {
                        span { style: "font-family: var(--font-monospace); color: var(--text-bright);", ": " }
                        "Select Software"
                    }
                    span {
                        span { style: "font-family: var(--font-monospace); color: var(--text-bright);", "Esc " }
                        "Clear Search"
                    }
                }

                // Items Count
                div {
                    style: "display: flex; align-items: center; gap: 6px;",
                    span { "Category: " }
                    span { style: "color: var(--theme-accent); font-weight: 600; text-transform: uppercase;", "{selected_software()}" }
                    span { style: "color: var(--theme-border); margin: 0 2px;", "•" }
                    span { "Showing {matched_count} of {total_count}" }
                }
            }
        }
    }
}
