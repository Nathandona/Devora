export const DEVORA_COMMANDS = [
  'devora new my-app rust',
  'devora new game cpp --framework=sfml',
  'devora new api python --framework=fastapi',
  'devora new web-project typescript --framework=next',
] as const;

export const LANGUAGES = [
  {
    name: 'Rust',
    status: 'available' as const,
    frameworks: 1,
    accent: '#b8653f',
    description: 'Systems programming language'
  },
  {
    name: 'C++',
    status: 'coming-soon' as const,
    frameworks: 0,
    accent: '#4a7ba7',
    description: 'High-performance systems programming'
  },
  {
    name: 'Python',
    status: 'planned' as const,
    frameworks: 0,
    accent: '#5a85b5',
    description: 'Versatile programming language'
  },
  {
    name: 'TypeScript',
    status: 'planned' as const,
    frameworks: 0,
    accent: '#5a8ac8',
    description: 'Typed JavaScript for modern apps'
  },
  {
    name: 'Go',
    status: 'planned' as const,
    frameworks: 0,
    accent: '#5a9caa',
    description: 'Simple, reliable, and efficient'
  },
  {
    name: 'Zig',
    status: 'planned' as const,
    frameworks: 0,
    accent: '#c4a568',
    description: 'Simple, fast, and safe'
  }
];

export const FEATURES = [
  {
    icon: 'Blocks',
    title: 'Plugin Architecture',
    description: 'Add languages without recompilation. Extensible by design.'
  },
  {
    icon: 'Zap',
    title: 'Smart Templates',
    description: 'Context-aware templates with interactive prompts and conditional logic.'
  },
  {
    icon: 'Layers',
    title: 'Hook System',
    description: 'Auto-format, git init, dependency install. Full automation.'
  }
];

export const DEMO_COMMANDS = [
  {
    command: 'devora new rust my-project',
    description: 'Create a new Rust project',
    output: [
      '🦀 Creating Rust project "my-project"...',
      '📁 Generated project structure:',
      '├── Cargo.toml',
      '├── README.md',
      '├── .gitignore',
      '├── src/',
      '│   ├── main.rs',
      '│   └── tests.rs',
      '✅ Project created successfully!',
      '🚀 Run "cd my-project && cargo run" to start'
    ]
  },
  {
    command: 'devora list',
    description: 'List available languages',
    output: [
      'Available languages:',
      '  🦀 Rust (1 framework)',
      '     - base: Basic Rust project',
      '',
      'Coming soon:',
      '  🐍 Python',
      '  🟨 C++',
      '  📘 TypeScript'
    ]
  }
];

export const INSTALL_COMMAND = 'curl -sSL https://install.devora.sh | sh';

export const GITHUB_URL = 'https://github.com/Nathandona/devora';
export const DOCS_URL = 'https://docs.devora.sh';