export const INSTALL_COMMAND = 'curl -LsSf https://github.com/Nathandona/Devora/releases/latest/download/devora-installer.sh | sh';
export const GITHUB_URL = 'https://github.com/Nathandona/Devora';
export const DOCS_URL = `${GITHUB_URL}#readme`;

// Per-OS install commands. The unix curl works on macOS and Linux; Windows
// uses the PowerShell installer. Both are published by dist on each release.
export type InstallTargetId = 'unix' | 'windows';

export interface InstallTarget {
  id: InstallTargetId;
  label: string;
  prompt: string;
  command: string;
}

export const INSTALL_TARGETS: InstallTarget[] = [
  {
    id: 'unix',
    label: 'macOS / Linux',
    prompt: '$',
    command:
      'curl -LsSf https://github.com/Nathandona/Devora/releases/latest/download/devora-installer.sh | sh',
  },
  {
    id: 'windows',
    label: 'Windows',
    prompt: '>',
    command:
      'irm https://github.com/Nathandona/Devora/releases/latest/download/devora-installer.ps1 | iex',
  },
];
export const CONTRIBUTING_URL = `${GITHUB_URL}/blob/main/CONTRIBUTING.md`;
export const ISSUES_URL = `${GITHUB_URL}/issues`;
export const GOOD_FIRST_ISSUE_URL = `${GITHUB_URL}/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22`;

// Hero terminal sequences. Only the things that actually work today.
export interface TerminalLine {
  kind: 'prompt' | 'command' | 'output' | 'tree' | 'success' | 'muted';
  text: string;
}

export interface TerminalSequence {
  id: string;
  lines: TerminalLine[];
}

export const TERMINAL_SEQUENCES: TerminalSequence[] = [
  {
    id: 'new-rust',
    lines: [
      { kind: 'command', text: 'devora new my-app rust' },
      { kind: 'muted',   text: 'Resolving plugin: rust@0.1.0' },
      { kind: 'muted',   text: 'Rendering templates …' },
      { kind: 'tree',    text: 'my-app' },
      { kind: 'tree',    text: '├─ .gitignore' },
      { kind: 'tree',    text: '├─ Cargo.toml' },
      { kind: 'tree',    text: '├─ README.md' },
      { kind: 'tree',    text: '└─ src' },
      { kind: 'tree',    text: '   └─ main.rs' },
      { kind: 'success', text: 'created my-app in 0.42s' },
    ],
  },
  {
    id: 'list',
    lines: [
      { kind: 'command', text: 'devora list' },
      { kind: 'output',  text: 'stable     rust          1 template' },
      { kind: 'output',  text: 'stable     c++           1 template' },
      { kind: 'output',  text: 'stable     go            1 template' },
      { kind: 'output',  text: 'stable     python        1 template' },
      { kind: 'output',  text: 'stable     c#            1 template' },
    ],
  },
];

// Three feature blocks, each with a real-looking snippet.
export interface Feature {
  title: string;
  body: string;
  code: string;
}

export const FEATURES: Feature[] = [
  {
    title: 'Plugin architecture',
    body: 'Every language is a self-contained plugin - a manifest plus Tera templates. The engine stays generic; languages never touch its code.',
    code: [
      'plugins/rust/',
      '├─ manifest.toml',
      '└─ frameworks/base/',
      '   ├─ manifest.toml',
      '   └─ templates/',
    ].join('\n'),
  },
  {
    title: 'Typed template variables',
    body: 'Templates declare variables with defaults and prompts. Pass them with --var or answer interactively; conditionals branch on the values.',
    code: [
      '# manifest.toml',
      '[variables]',
      'license       = { default = "MIT" }',
      'include_tests = { default = true }',
    ].join('\n'),
  },
  {
    title: 'Lifecycle hooks',
    body: 'Run formatters, init git, install deps after generation - declarative, cross-platform, and skippable per run with --no-hooks.',
    code: [
      '# manifest.toml',
      '[[post_hooks]]',
      'command = "cargo fmt"',
      '',
      '[[post_hooks]]',
      'command = "devora_git_init"',
    ].join('\n'),
  },
];

// Language status - replaces the old "roadmap with quarters" framing.
export type LangState = 'stable' | 'paused' | 'wishlist';

export interface LangRow {
  name: string;
  state: LangState;
  note: string;
}

export const LANGUAGES: LangRow[] = [
  { name: 'Rust',   state: 'stable', note: 'Single template, full hook system.' },
  { name: 'C++',    state: 'stable', note: 'CMake template with vendored doctest tests.' },
  { name: 'Go',     state: 'stable', note: 'Go module with built-in testing.' },
  { name: 'Python', state: 'stable', note: 'src layout package with unittest.' },
  { name: 'C#',     state: 'stable', note: '.NET console app with an xUnit test project.' },
];
