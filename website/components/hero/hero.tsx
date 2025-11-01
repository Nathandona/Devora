'use client';

import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { toast } from 'sonner';
import { Typewriter } from './typewriter';
import { TerminalAnimation } from './terminal-animation';
import { INSTALL_COMMAND, GITHUB_URL, DOCS_URL } from '@/lib/constants';
import { Copy, ExternalLink, Github, BookOpen, Terminal } from 'lucide-react';

export function Hero() {
  const copyToClipboard = async () => {
    try {
      await navigator.clipboard.writeText(INSTALL_COMMAND);
      toast.success('Command copied to clipboard!');
    } catch (error) {
      toast.error('Failed to copy command to clipboard');
    }
  };

  return (
    <section className="min-h-screen flex items-center justify-center px-4 py-20 relative overflow-hidden">
      {/* Background Grid */}
      <div className="absolute inset-0 bg-grid-pattern opacity-5" />

      <div className="container max-w-7xl mx-auto relative z-10">
        <div className="grid lg:grid-cols-2 gap-12 lg:gap-16 items-center">

          {/* Left Side - Text Content */}
          <div className="space-y-6 animate-fade-in">

            <div className="space-y-4">
              <h1 className="text-4xl md:text-6xl lg:text-7xl font-bold tracking-tight">
                <span className="bg-gradient-to-r from-foreground to-foreground/80 bg-clip-text text-transparent">
                  Scaffold any
                </span>
                <br />
                <span className="bg-gradient-to-r from-primary/90 to-primary/70 bg-clip-text text-transparent">
                  project in seconds
                </span>
              </h1>

              <p className="text-lg md:text-xl text-muted-foreground leading-relaxed max-w-2xl">
                Universal project scaffolding that works across any language.
                Stop copy-pasting boilerplate and start creating.
              </p>
            </div>

            {/* Typewriter Command */}
            <div className="my-8">
              <div className="p-4 rounded-lg border border-border/50 bg-card/30 backdrop-blur-sm">
                <Typewriter />
              </div>
            </div>

            {/* CTAs */}
            <div className="flex flex-wrap gap-4">
              <Button
                size="lg"
                className="bg-primary hover:bg-primary/90 text-primary-foreground px-8 transform hover:scale-105 transition-transform"
                onClick={() => window.open(GITHUB_URL, '_blank')}
              >
                <Github className="w-4 h-4 mr-2" />
                Get Started
              </Button>

              <Button
                variant="outline"
                size="lg"
                className="border-border/50 hover:bg-accent/50 px-8 transform hover:scale-105 transition-transform"
                onClick={() => window.open(DOCS_URL, '_blank')}
              >
                <BookOpen className="w-4 h-4 mr-2" />
                View Docs
                <ExternalLink className="w-4 h-4 ml-2" />
              </Button>
            </div>

            {/* Install Command */}
            <div className="space-y-3">
              <p className="text-sm text-muted-foreground">Quick install:</p>
              <div className="flex items-center gap-2 p-3 rounded-lg border border-border/50 bg-card/30 backdrop-blur-sm">
                <Terminal className="w-4 h-4 text-muted-foreground" />
                <code className="font-mono text-sm flex-1">{INSTALL_COMMAND}</code>
                <Button
                  size="sm"
                  variant="ghost"
                  onClick={copyToClipboard}
                  className="h-8 px-2 hover:bg-accent/50"
                >
                  <Copy className="w-4 h-4" />
                </Button>
              </div>
            </div>
          </div>

          {/* Right Side - Terminal Animation */}
          <div className="relative">
            <div className="animate-slide-right">
              <TerminalAnimation />
            </div>

            {/* Floating elements */}
            <div className="absolute -top-8 -right-8 w-24 h-24 bg-gradient-to-br from-primary/15 to-transparent rounded-full blur-xl animate-float" />

            <div className="absolute -bottom-8 -left-8 w-32 h-32 bg-gradient-to-tr from-primary/10 to-transparent rounded-full blur-xl animate-float-delayed" />
          </div>
        </div>
      </div>

      {/* Bottom gradient fade */}
      <div className="absolute bottom-0 left-0 right-0 h-32 bg-gradient-to-t from-background to-transparent" />
    </section>
  );
}