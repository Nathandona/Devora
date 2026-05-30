'use client';

import { useEffect, useState } from 'react';
import { AnimatePresence, motion } from 'framer-motion';
import { Github, Menu, X, Star } from 'lucide-react';
import { GITHUB_URL, DOCS_URL } from '@/lib/constants';
import { useGitHubStats } from '@/hooks/use-github-stats';
import { EASE_OUT } from '@/lib/animations';

const NAV_ITEMS = [
  { label: 'Features',  href: '#features' },
  { label: 'Languages', href: '#languages' },
  { label: 'Open source', href: '#open-source' },
];

function StarButton() {
  const { data, loading } = useGitHubStats();
  const stars = data?.repository.stargazers_count;

  return (
    <a
      href={GITHUB_URL}
      target="_blank"
      rel="noreferrer"
      className="group inline-flex items-center gap-2 rounded-md border border-border px-3 py-1.5 text-xs text-muted-foreground hover:text-foreground hover:border-foreground/40 transition-colors"
    >
      <Github className="size-3.5" />
      <span className="font-medium">GitHub</span>
      <span className="hidden sm:inline-flex items-center gap-1 border-l border-border pl-2 ml-1">
        <Star className="size-3 fill-current opacity-60" />
        <span className="font-mono tabular-nums">
          {loading || stars == null ? '-' : stars}
        </span>
      </span>
    </a>
  );
}

export function MotionNav() {
  const [isOpen, setIsOpen] = useState(false);
  const [scrolled, setScrolled] = useState(false);

  useEffect(() => {
    const onScroll = () => setScrolled(window.scrollY > 24);
    onScroll();
    window.addEventListener('scroll', onScroll, { passive: true });
    return () => window.removeEventListener('scroll', onScroll);
  }, []);

  return (
    <motion.nav
      initial={{ y: -32, opacity: 0 }}
      animate={{ y: 0, opacity: 1 }}
      transition={{ duration: 0.5, ease: EASE_OUT }}
      className={`fixed inset-x-0 top-0 z-50 transition-[background-color,backdrop-filter,border-color] duration-200 ${
        scrolled
          ? 'bg-background/72 backdrop-blur-xl border-b border-border'
          : 'bg-transparent border-b border-transparent'
      }`}
    >
      <div className="container-tight flex h-14 items-center justify-between">
        <a href="#hero" className="flex items-center gap-2">
          <span className="font-mono text-sm font-medium tracking-tight">devora</span>
          <span className="font-mono text-[10px] text-muted-foreground/70 border border-border rounded px-1.5 py-px">
            v0.1
          </span>
        </a>

        {/* Desktop */}
        <div className="hidden md:flex items-center gap-1">
          {NAV_ITEMS.map(item => (
            <a
              key={item.label}
              href={item.href}
              className="px-3 py-1.5 text-sm text-muted-foreground hover:text-foreground transition-colors"
            >
              {item.label}
            </a>
          ))}
          <a
            href={DOCS_URL}
            target="_blank"
            rel="noreferrer"
            className="px-3 py-1.5 text-sm text-muted-foreground hover:text-foreground transition-colors"
          >
            Docs
          </a>
          <div className="ml-2">
            <StarButton />
          </div>
        </div>

        {/* Mobile toggle */}
        <button
          aria-label={isOpen ? 'Close menu' : 'Open menu'}
          onClick={() => setIsOpen(o => !o)}
          className="md:hidden inline-flex size-9 items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
        >
          {isOpen ? <X className="size-4" /> : <Menu className="size-4" />}
        </button>
      </div>

      <AnimatePresence>
        {isOpen && (
          <motion.div
            initial={{ opacity: 0, height: 0 }}
            animate={{ opacity: 1, height: 'auto' }}
            exit={{ opacity: 0, height: 0 }}
            transition={{ duration: 0.2, ease: EASE_OUT }}
            className="md:hidden overflow-hidden border-t border-border bg-background/95 backdrop-blur-xl"
          >
            <div className="container-tight flex flex-col py-3 gap-1">
              {NAV_ITEMS.map(item => (
                <a
                  key={item.label}
                  href={item.href}
                  onClick={() => setIsOpen(false)}
                  className="px-2 py-2.5 text-sm text-muted-foreground hover:text-foreground transition-colors"
                >
                  {item.label}
                </a>
              ))}
              <a
                href={DOCS_URL}
                target="_blank"
                rel="noreferrer"
                onClick={() => setIsOpen(false)}
                className="px-2 py-2.5 text-sm text-muted-foreground hover:text-foreground transition-colors"
              >
                Docs
              </a>
              <div className="pt-2">
                <StarButton />
              </div>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </motion.nav>
  );
}
