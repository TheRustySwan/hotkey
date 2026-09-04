use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use ratatui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};
use std::{error::Error, io};

#[derive(Clone)]
struct Hotkey {
    software: &'static str,
    keys: &'static str,
    description: &'static str,
}

lazy_static::lazy_static! {
    static ref ALL_HOTKEYS: Vec<Hotkey> = vec![
        // =======================
        // VS Code
        // =======================
        // General
        Hotkey { software: "vscode", keys: "Ctrl+Shift+P", description: "Show Command Palette" },
        Hotkey { software: "vscode", keys: "Ctrl+P", description: "Quick Open, Go to File..." },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+N", description: "New window/instance" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+W", description: "Close window/instance" },
        Hotkey { software: "vscode", keys: "Ctrl+,", description: "User Settings" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+S", description: "Keyboard Shortcuts" },
        // Basic editing
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
        // Navigation
        Hotkey { software: "vscode", keys: "Ctrl+T", description: "Show all Symbols" },
        Hotkey { software: "vscode", keys: "Ctrl+G", description: "Go to Line..." },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+O", description: "Go to Symbol..." },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+M", description: "Show Problems panel" },
        Hotkey { software: "vscode", keys: "F8", description: "Go to next error or warning" },
        Hotkey { software: "vscode", keys: "Shift+F8", description: "Go to previous error or warning" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+Tab", description: "Navigate editor group history" },
        Hotkey { software: "vscode", keys: "Alt+Left / Right", description: "Go back / forward" },
        Hotkey { software: "vscode", keys: "Ctrl+M", description: "Toggle Tab key moves focus" },
        // Search and replace
        Hotkey { software: "vscode", keys: "Ctrl+F", description: "Find" },
        Hotkey { software: "vscode", keys: "Ctrl+H", description: "Replace" },
        Hotkey { software: "vscode", keys: "F3 / Shift+F3", description: "Find next/previous" },
        Hotkey { software: "vscode", keys: "Alt+Enter", description: "Select all occurences of Find match" },
        Hotkey { software: "vscode", keys: "Ctrl+D", description: "Add selection to next Find match" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+D", description: "Move last selection to next Find match" },
        Hotkey { software: "vscode", keys: "Alt+C / R / W", description: "Toggle case-sensitive / regex / whole word" },
        // Multi-cursor and selection
        Hotkey { software: "vscode", keys: "Alt+Click", description: "Insert cursor" },
        Hotkey { software: "vscode", keys: "Ctrl+Alt+Up/Down", description: "Insert cursor above / below" },
        Hotkey { software: "vscode", keys: "Ctrl+U", description: "Undo last cursor operation" },
        Hotkey { software: "vscode", keys: "Shift+Alt+I", description: "Insert cursor at end of each line selected" },
        Hotkey { software: "vscode", keys: "Ctrl+L", description: "Select current line" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+L", description: "Select all occurrences of current selection" },
        Hotkey { software: "vscode", keys: "Ctrl+F2", description: "Select all occurrences of current word" },
        Hotkey { software: "vscode", keys: "Shift+Alt+Right", description: "Expand selection" },
        Hotkey { software: "vscode", keys: "Shift+Alt+Left", description: "Shrink selection" },
        // Rich languages editing
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
        // Editor management
        Hotkey { software: "vscode", keys: "Ctrl+F4, Ctrl+W", description: "Close editor" },
        Hotkey { software: "vscode", keys: "Ctrl+K F", description: "Close folder" },
        Hotkey { software: "vscode", keys: "Ctrl+\\", description: "Split editor" },
        Hotkey { software: "vscode", keys: "Ctrl+1 / 2 / 3", description: "Focus into 1st, 2nd or 3rd editor group" },
        Hotkey { software: "vscode", keys: "Ctrl+K Ctrl+Left/Right", description: "Focus into previous/next editor group" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+PgUp/PgDn", description: "Move editor left/right" },
        Hotkey { software: "vscode", keys: "Ctrl+K Left / Right", description: "Move active editor group" },
        // File management
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
        // Display
        Hotkey { software: "vscode", keys: "F11", description: "Toggle full screen" },
        Hotkey { software: "vscode", keys: "Shift+Alt+0", description: "Toggle editor layout (horizontal/vertical)" },
        Hotkey { software: "vscode", keys: "Ctrl+=", description: "Zoom in" },
        Hotkey { software: "vscode", keys: "Ctrl+-", description: "Zoom out" },
        Hotkey { software: "vscode", keys: "Ctrl+B", description: "Toggle Sidebar visibility" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+E", description: "Show Explorer / Toggle focus" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+F", description: "Show Search" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+G", description: "Show Source Control" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+D", description: "Show Debug" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+X", description: "Show Extensions" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+H", description: "Replace in files" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+J", description: "Toggle Search details" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+U", description: "Show Output panel" },
        Hotkey { software: "vscode", keys: "Ctrl+Shift+V", description: "Open Markdown preview" },
        Hotkey { software: "vscode", keys: "Ctrl+K V", description: "Open Markdown preview to the side" },
        Hotkey { software: "vscode", keys: "Ctrl+`", description: "Show integrated terminal" },
        // Debug
        Hotkey { software: "vscode", keys: "F9", description: "Toggle breakpoint" },
        Hotkey { software: "vscode", keys: "F5", description: "Start/Continue" },
        Hotkey { software: "vscode", keys: "Shift+F5", description: "Stop" },
        Hotkey { software: "vscode", keys: "F11 / Shift+F11", description: "Step into/out" },
        Hotkey { software: "vscode", keys: "F10", description: "Step over" },

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

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Search,
    SelectSoftware,
}

struct App {
    mode: Mode,
    query: String,
    state: ListState,
    filtered_indices: Vec<(usize, i64)>, // index in ALL_HOTKEYS, score

    selected_software: String,

    software_query: String,
    software_state: ListState,
    filtered_softwares: Vec<String>,
}

impl App {
    fn new() -> App {
        let mut app = App {
            mode: Mode::Search,
            query: String::new(),
            state: ListState::default(),
            filtered_indices: Vec::new(),
            selected_software: "all".to_string(),
            software_query: String::new(),
            software_state: ListState::default(),
            filtered_softwares: SOFTWARES.iter().map(|s| s.to_string()).collect(),
        };
        app.update_search();
        app
    }

    fn update_search(&mut self) {
        let matcher = SkimMatcherV2::default();
        let mut matches = Vec::new();

        for (i, hotkey) in ALL_HOTKEYS.iter().enumerate() {
            if self.selected_software != "all" {
                if self.selected_software == "lazyvim" {
                    if hotkey.software != "lazyvim" && hotkey.software != "vim" {
                        continue;
                    }
                } else if hotkey.software != self.selected_software {
                    continue;
                }
            }

            if self.query.is_empty() {
                matches.push((i, 0));
            } else {
                // Match against software, keys, and description
                let target = format!("{} {} {}", hotkey.software, hotkey.keys, hotkey.description);
                if let Some(score) = matcher.fuzzy_match(&target, &self.query) {
                    matches.push((i, score));
                }
            }
        }

        if !self.query.is_empty() {
            matches.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by score descending
        }

        self.filtered_indices = matches;

        if self.filtered_indices.is_empty() {
            self.state.select(None);
        } else {
            self.state.select(Some(0));
        }
    }

    fn update_software_search(&mut self) {
        let matcher = SkimMatcherV2::default();
        let mut matches = Vec::new();

        if self.software_query.is_empty() {
            self.filtered_softwares = SOFTWARES.iter().map(|s| s.to_string()).collect();
        } else {
            for sw in SOFTWARES.iter() {
                if let Some(score) = matcher.fuzzy_match(sw, &self.software_query) {
                    matches.push((sw.to_string(), score));
                }
            }
            matches.sort_by(|a, b| b.1.cmp(&a.1));
            self.filtered_softwares = matches.into_iter().map(|(s, _)| s).collect();
        }

        if self.filtered_softwares.is_empty() {
            self.software_state.select(None);
        } else {
            self.software_state.select(Some(0));
        }
    }

    fn next(&mut self) {
        match self.mode {
            Mode::Search => {
                let i = match self.state.selected() {
                    Some(i) => {
                        if i >= self.filtered_indices.len().saturating_sub(1) {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                if !self.filtered_indices.is_empty() {
                    self.state.select(Some(i));
                }
            }
            Mode::SelectSoftware => {
                let i = match self.software_state.selected() {
                    Some(i) => {
                        if i >= self.filtered_softwares.len().saturating_sub(1) {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                if !self.filtered_softwares.is_empty() {
                    self.software_state.select(Some(i));
                }
            }
        }
    }

    fn previous(&mut self) {
        match self.mode {
            Mode::Search => {
                let i = match self.state.selected() {
                    Some(i) => {
                        if i == 0 {
                            self.filtered_indices.len().saturating_sub(1)
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                if !self.filtered_indices.is_empty() {
                    self.state.select(Some(i));
                }
            }
            Mode::SelectSoftware => {
                let i = match self.software_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            self.filtered_softwares.len().saturating_sub(1)
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                if !self.filtered_softwares.is_empty() {
                    self.software_state.select(Some(i));
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>>
where
    <B as Backend>::Error: 'static + Error,
{
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match app.mode {
                    Mode::Search => match key.code {
                        KeyCode::Esc => return Ok(()),
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(());
                        }
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        KeyCode::Backspace => {
                            app.query.pop();
                            app.update_search();
                        }
                        KeyCode::Char(':') if app.query.is_empty() => {
                            app.mode = Mode::SelectSoftware;
                            app.software_query.clear();
                            app.update_software_search();
                        }
                        KeyCode::Char(c) => {
                            app.query.push(c);
                            app.update_search();
                        }
                        _ => {}
                    },
                    Mode::SelectSoftware => match key.code {
                        KeyCode::Esc => {
                            app.mode = Mode::Search;
                        }
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(());
                        }
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        KeyCode::Tab => {
                            if let Some(i) = app.software_state.selected() {
                                if let Some(sw) = app.filtered_softwares.get(i) {
                                    app.software_query = sw.clone();
                                    app.update_software_search();
                                }
                            }
                        }
                        KeyCode::Enter => {
                            if let Some(i) = app.software_state.selected() {
                                if let Some(sw) = app.filtered_softwares.get(i) {
                                    app.selected_software = sw.clone();
                                    app.mode = Mode::Search;
                                    app.update_search();
                                }
                            } else {
                                app.mode = Mode::Search;
                            }
                        }
                        KeyCode::Backspace => {
                            app.software_query.pop();
                            app.update_software_search();
                        }
                        KeyCode::Char(c) => {
                            app.software_query.push(c);
                            app.update_software_search();
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}

// Helper to center a rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

// NullDark Theme Color Tokens
const THEME_BG_OUTER: Color = Color::Rgb(24, 24, 24);      // #181818
const THEME_BG_PAGE: Color = Color::Rgb(30, 30, 30);       // #1e1e1e
const THEME_BG_PANEL: Color = Color::Rgb(36, 36, 36);      // #242424
const THEME_BG_ACTIVE: Color = Color::Rgb(42, 42, 42);     // #2a2a2a
const THEME_BORDER: Color = Color::Rgb(54, 54, 54);        // #363636
const THEME_BORDER_SUBTLE: Color = Color::Rgb(44, 44, 44); // #2c2c2c

const THEME_ACCENT: Color = Color::Rgb(214, 6, 69);        // #d60645 (Signature Magenta)
const THEME_ACCENT_BRIGHT: Color = Color::Rgb(222, 27, 84);// #de1b54
const THEME_SUCCESS: Color = Color::Rgb(166, 210, 75);     // #a6d24b

const THEME_TEXT_BRIGHT: Color = Color::Rgb(189, 189, 189);// #bdbcbc
const THEME_TEXT_NORMAL: Color = Color::Rgb(168, 168, 168);// #a8a8a8
const THEME_TEXT_MUTED: Color = Color::Rgb(120, 120, 120); // #787878
const THEME_TEXT_DIM: Color = Color::Rgb(85, 85, 85);      // #555555

fn ui(f: &mut Frame, app: &mut App) {
    // Fill entire terminal with NullDark outer background (#181818)
    f.render_widget(Block::default().style(Style::default().bg(THEME_BG_OUTER)), f.area());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1), // Brand Header
            Constraint::Length(3), // Search Bar
            Constraint::Min(1),    // Results List
            Constraint::Length(1), // Status Footer
        ].as_ref())
        .split(f.area());

    // 1. Brand Header
    let header_line = Line::from(vec![
        Span::styled("NULL", Style::default().fg(THEME_TEXT_MUTED).add_modifier(Modifier::BOLD)),
        Span::styled("HOTKEYS", Style::default().fg(THEME_TEXT_BRIGHT).add_modifier(Modifier::BOLD)),
        Span::styled(" | ", Style::default().fg(THEME_BORDER)),
        Span::styled("KEYBOARD SHORTCUT WORKSPACE", Style::default().fg(THEME_TEXT_MUTED)),
        Span::styled("  •  ", Style::default().fg(THEME_BORDER)),
        Span::styled(format!("[{}]", app.selected_software.to_uppercase()), Style::default().fg(THEME_ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled(format!(" ({} shortcuts)", app.filtered_indices.len()), Style::default().fg(THEME_TEXT_DIM)),
    ]);
    f.render_widget(Paragraph::new(header_line), chunks[0]);

    // 2. Search Input
    let cursor_char = if app.mode == Mode::Search { "█" } else { "" };
    let input_line = Line::from(vec![
        Span::styled("> ", Style::default().fg(THEME_ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled(&app.query, Style::default().fg(THEME_TEXT_BRIGHT)),
        Span::styled(cursor_char, Style::default().fg(THEME_ACCENT)),
    ]);
    let input = Paragraph::new(input_line)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(THEME_BORDER))
                .style(Style::default().bg(THEME_BG_PANEL))
                .title(Span::styled(" SEARCH ", Style::default().fg(THEME_TEXT_MUTED).add_modifier(Modifier::BOLD)))
        );
    f.render_widget(input, chunks[1]);

    // 3. Results List
    let items: Vec<ListItem> = app
        .filtered_indices
        .iter()
        .map(|&(i, _)| {
            let hotkey = &ALL_HOTKEYS[i];

            let sw_span = Span::styled(
                format!("[{:<8}] ", hotkey.software.to_uppercase()),
                Style::default().fg(THEME_TEXT_MUTED),
            );

            let keys_span = Span::styled(
                format!("{:<28}", hotkey.keys),
                Style::default()
                    .fg(THEME_TEXT_BRIGHT)
                    .add_modifier(Modifier::BOLD),
            );

            let desc_span = Span::styled(
                hotkey.description,
                Style::default().fg(THEME_TEXT_NORMAL),
            );
            ListItem::new(Line::from(vec![sw_span, keys_span, desc_span]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(THEME_BORDER))
                .style(Style::default().bg(THEME_BG_PAGE))
                .title(Span::styled(" SHORTCUTS ", Style::default().fg(THEME_TEXT_MUTED).add_modifier(Modifier::BOLD)))
        )
        .highlight_style(
            Style::default()
                .bg(THEME_BG_ACTIVE)
                .fg(THEME_ACCENT_BRIGHT)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[2], &mut app.state);

    // 4. Status Footer
    let footer_line = Line::from(vec![
        Span::styled("↑↓ ", Style::default().fg(THEME_TEXT_BRIGHT)),
        Span::styled("Navigate  ", Style::default().fg(THEME_TEXT_DIM)),
        Span::styled(": ", Style::default().fg(THEME_TEXT_BRIGHT)),
        Span::styled("Select Software  ", Style::default().fg(THEME_TEXT_DIM)),
        Span::styled("Esc ", Style::default().fg(THEME_TEXT_BRIGHT)),
        Span::styled("Clear  ", Style::default().fg(THEME_TEXT_DIM)),
        Span::styled("q / Ctrl+C ", Style::default().fg(THEME_TEXT_BRIGHT)),
        Span::styled("Quit", Style::default().fg(THEME_TEXT_DIM)),
    ]);
    f.render_widget(Paragraph::new(footer_line), chunks[3]);

    // 5. Software Selection Modal
    if let Mode::SelectSoftware = app.mode {
        let area = centered_rect(50, 45, f.area());
        f.render_widget(Clear, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(area);

        let popup_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(THEME_ACCENT))
            .title(Span::styled(" FILTER BY SOFTWARE (Tab autocomplete, Enter select, Esc cancel) ", Style::default().fg(THEME_TEXT_BRIGHT).add_modifier(Modifier::BOLD)))
            .style(Style::default().bg(THEME_BG_PANEL));
        f.render_widget(popup_block, area);

        let sw_input_line = Line::from(vec![
            Span::styled(":", Style::default().fg(THEME_ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled(&app.software_query, Style::default().fg(THEME_TEXT_BRIGHT)),
            Span::styled("█", Style::default().fg(THEME_ACCENT)),
        ]);
        let sw_input = Paragraph::new(sw_input_line)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(Style::default().fg(THEME_BORDER_SUBTLE))
            );
        f.render_widget(sw_input, popup_chunks[0]);

        let sw_items: Vec<ListItem> = app
            .filtered_softwares
            .iter()
            .map(|s| {
                let is_current = app.selected_software == *s;
                if is_current {
                    ListItem::new(Line::from(vec![
                        Span::styled(format!("{:<15} ", s.to_uppercase()), Style::default().fg(THEME_TEXT_BRIGHT).add_modifier(Modifier::BOLD)),
                        Span::styled("[ACTIVE]", Style::default().fg(THEME_SUCCESS).add_modifier(Modifier::BOLD)),
                    ]))
                } else {
                    ListItem::new(Span::styled(s.to_uppercase(), Style::default().fg(THEME_TEXT_NORMAL)))
                }
            })
            .collect();

        let sw_list = List::new(sw_items)
            .highlight_style(
                Style::default()
                    .bg(THEME_BG_ACTIVE)
                    .fg(THEME_ACCENT)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        f.render_stateful_widget(sw_list, popup_chunks[1], &mut app.software_state);
    }
}
