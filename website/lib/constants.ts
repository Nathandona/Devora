export const INSTALL_COMMAND = 'curl -sSL https://install.devora.sh | sh';
export const GITHUB_URL = 'https://github.com/Nathandona/devora';
export const DOCS_URL = 'https://docs.devora.sh';
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
      { kind: 'tree',    text: '├─ Cargo.toml' },
      { kind: 'tree',    text: '├─ README.md' },
      { kind: 'tree',    text: '├─ .gitignore' },
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
      { kind: 'muted',   text: 'paused     c++           templates being rethought' },
      { kind: 'muted',   text: 'wishlist   go            open to contributions' },
      { kind: 'muted',   text: 'wishlist   python        open to contributions' },
      { kind: 'muted',   text: 'wishlist   typescript    open to contributions' },
      { kind: 'muted',   text: 'wishlist   zig           open to contributions' },
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
    body: 'Languages live in plugins, not in the binary. Drop one in, restart, it works. No fork, no rebuild.',
    code: [
      '$ devora plugin add ./rust-plugin',
      '  loaded rust@0.1.0',
      '$ devora new my-app rust',
      '  created my-app in 0.42s',
    ].join('\n'),
  },
  {
    title: 'Templates with prompts',
    body: 'Templates can ask for input and branch on the answer. No more silently scaffolded files you didn\'t want.',
    code: [
      '# template.toml',
      '[[prompt]]',
      'name    = "use_tokio"',
      'message = "Add tokio runtime?"',
      'kind    = "confirm"',
      'default = true',
    ].join('\n'),
  },
  {
    title: 'Lifecycle hooks',
    body: 'Run formatters, init git, install deps — declarative, composable, skippable per-run with --no-hooks.',
    code: [
      '# devora.toml',
      '[hooks]',
      'post_create = [',
      '  "git init",',
      '  "cargo fmt",',
      ']',
    ].join('\n'),
  },
];

// Language status — replaces the old "roadmap with quarters" framing.
export type LangState = 'stable' | 'paused' | 'wishlist';

export interface LangRow {
  name: string;
  state: LangState;
  note: string;
}

export const LANGUAGES: LangRow[] = [
  { name: 'Rust',       state: 'stable',   note: 'Single template, full hook system.' },
  { name: 'C++',        state: 'paused',   note: 'Templates removed. Direction being rethought.' },
  { name: 'Go',         state: 'wishlist', note: 'Open to contributions.' },
  { name: 'Python',     state: 'wishlist', note: 'Open to contributions.' },
  { name: 'TypeScript', state: 'wishlist', note: 'Open to contributions.' },
  { name: 'Zig',        state: 'wishlist', note: 'Open to contributions.' },
];
