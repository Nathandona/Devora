'use client';

import { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Github, Terminal, Menu, X, BookOpen } from 'lucide-react';
import { GITHUB_URL, DOCS_URL } from '@/lib/constants';

export function MotionNav() {
  const [isOpen, setIsOpen] = useState(false);
  const [scrolled, setScrolled] = useState(false);

  useEffect(() => {
    const handleScroll = () => {
      setScrolled(window.scrollY > 20);
    };

    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  const navItems = [
    { label: 'Features', href: '#features' },
    { label: 'Languages', href: '#languages' },
    { label: 'Demo', href: '#demo' },
    { label: 'Documentation', href: DOCS_URL, external: true },
  ];

  return (
    <motion.nav
      initial={{ y: -100 }}
      animate={{ y: 0 }}
      className={`fixed top-0 left-0 right-0 z-50 transition-all duration-300 ${
        scrolled
          ? 'bg-background/80 backdrop-blur-md border-b border-border/50'
          : 'bg-transparent'
      }`}
    >
      <div className="container max-w-7xl mx-auto px-4">
        <div className="flex items-center justify-between h-16">
          {/* Logo */}
          <div className="flex items-center gap-3">
            <div className="flex items-center gap-2">
              <Terminal className="w-6 h-6 text-primary" />
              <span className="text-xl font-bold">Devora</span>
            </div>
            <Badge variant="secondary" className="text-xs">
              v0.1.0
            </Badge>
          </div>

          {/* Desktop Navigation */}
          <div className="hidden md:flex items-center gap-8">
            <div className="flex items-center gap-6">
              {navItems.slice(0, 3).map((item) => (
                <a
                  key={item.label}
                  href={item.href}
                  className="text-muted-foreground hover:text-foreground transition-colors text-sm font-medium"
                >
                  {item.label}
                </a>
              ))}
            </div>

            <div className="flex items-center gap-2">
              <Button
                variant="ghost"
                size="sm"
                onClick={() => window.open(DOCS_URL, '_blank')}
                className="hover:bg-accent/50"
              >
                <BookOpen className="w-4 h-4 mr-2" />
                Docs
              </Button>
              <Button
                size="sm"
                onClick={() => window.open(GITHUB_URL, '_blank')}
                className="bg-primary hover:bg-primary/90"
              >
                <Github className="w-4 h-4 mr-2" />
                GitHub
              </Button>
            </div>
          </div>

          {/* Mobile Menu Button */}
          <Button
            variant="ghost"
            size="sm"
            className="md:hidden"
            onClick={() => setIsOpen(!isOpen)}
          >
            {isOpen ? (
              <X className="w-5 h-5" />
            ) : (
              <Menu className="w-5 h-5" />
            )}
          </Button>
        </div>
      </div>

      {/* Mobile Menu */}
      <AnimatePresence>
        {isOpen && (
          <motion.div
            initial={{ opacity: 0, height: 0 }}
            animate={{ opacity: 1, height: 'auto' }}
            exit={{ opacity: 0, height: 0 }}
            transition={{ duration: 0.2 }}
            className="md:hidden bg-background/95 backdrop-blur-md border-b border-border/50"
          >
            <div className="container max-w-7xl mx-auto px-4 py-4 space-y-3">
              {navItems.map((item) => (
                <a
                  key={item.label}
                  href={item.href}
                  className="flex items-center justify-between p-2 rounded-lg hover:bg-accent/50 transition-colors"
                  onClick={() => setIsOpen(false)}
                >
                  <span className="text-sm font-medium">{item.label}</span>
                  {item.external && (
                    <span className="text-xs text-muted-foreground">→</span>
                  )}
                </a>
              ))}

              <div className="pt-3 space-y-2">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => window.open(DOCS_URL, '_blank')}
                  className="w-full"
                >
                  <BookOpen className="w-4 h-4 mr-2" />
                  Documentation
                </Button>
                <Button
                  size="sm"
                  onClick={() => window.open(GITHUB_URL, '_blank')}
                  className="w-full bg-primary hover:bg-primary/90"
                >
                  <Github className="w-4 h-4 mr-2" />
                  View on GitHub
                </Button>
              </div>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </motion.nav>
  );
}