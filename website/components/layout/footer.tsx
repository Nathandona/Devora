import Link from 'next/link';
import { Github } from 'lucide-react';
import {
  GITHUB_URL,
  DOCS_URL,
  CONTRIBUTING_URL,
  ISSUES_URL,
} from '@/lib/constants';

const COLS = [
  {
    heading: 'Project',
    links: [
      { label: 'GitHub',     href: GITHUB_URL,       external: true  },
      { label: 'Docs',       href: DOCS_URL,         external: true  },
      { label: 'Contribute', href: CONTRIBUTING_URL, external: true  },
      { label: 'Issues',     href: ISSUES_URL,       external: true  },
    ],
  },
  {
    heading: 'On this page',
    links: [
      { label: 'Features',    href: '#features',    external: false },
      { label: 'Languages',   href: '#languages',   external: false },
      { label: 'Open source', href: '#open-source', external: false },
    ],
  },
];

export function Footer() {
  return (
    <footer className="border-t border-border">
      <div className="container-tight py-14">
        <div className="grid gap-10 md:grid-cols-[1.6fr_1fr_1fr]">
          <div className="max-w-sm">
            <Link href="#hero" className="inline-flex items-center gap-2">
              <span className="font-mono text-sm font-medium">devora</span>
              <span className="font-mono text-[10px] text-muted-foreground/70 border border-border rounded px-1.5 py-px">
                v0.1
              </span>
            </Link>
            <p className="mt-4 text-sm leading-relaxed text-muted-foreground">
              A plugin-driven CLI for project boilerplate. Open source, MIT.
              Built quietly.
            </p>
            <a
              href={GITHUB_URL}
              target="_blank"
              rel="noreferrer"
              className="mt-5 inline-flex items-center gap-2 text-sm text-muted-foreground hover:text-foreground transition-colors"
            >
              <Github className="size-4" />
              github.com/Nathandona/devora
            </a>
          </div>

          {COLS.map(col => (
            <div key={col.heading}>
              <h3 className="text-xs uppercase tracking-[0.18em] text-muted-foreground/70 font-medium">
                {col.heading}
              </h3>
              <ul className="mt-4 space-y-2.5">
                {col.links.map(link => (
                  <li key={link.label}>
                    <a
                      href={link.href}
                      target={link.external ? '_blank' : undefined}
                      rel={link.external ? 'noreferrer' : undefined}
                      className="text-sm text-muted-foreground hover:text-foreground transition-colors"
                    >
                      {link.label}
                    </a>
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>

        <div className="mt-14 pt-6 border-t border-border flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3 text-xs text-muted-foreground">
          <span>MIT &middot; &copy; {new Date().getFullYear()} Devora contributors</span>
          <span className="font-mono tracking-wider">/dev/null was here</span>
        </div>
      </div>
    </footer>
  );
}
