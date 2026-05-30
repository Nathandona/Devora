'use client';

import { motion } from 'framer-motion';
import { ArrowRight, ArrowUpRight } from 'lucide-react';
import { TerminalAnimation } from './terminal-animation';
import { InstallCommand } from '@/components/install/install-command';
import { GITHUB_URL } from '@/lib/constants';
import { EASE_OUT, fadeRise, stagger } from '@/lib/animations';

export function Hero() {
  return (
    <section
      id="hero"
      className="relative flex min-h-[100svh] items-center pt-28 pb-24"
    >
      <div className="container-tight">
        <div className="grid gap-16 lg:grid-cols-[1.05fr_1fr] lg:gap-12 items-center">
          {/* Left - text */}
          <motion.div
            initial="initial"
            animate="animate"
            variants={stagger}
            className="max-w-xl"
          >
            <motion.div variants={fadeRise} className="mb-6">
              <span className="inline-flex items-center gap-2 font-mono text-[11px] uppercase tracking-[0.18em] text-muted-foreground">
                <span className="size-1.5 rounded-full bg-foreground/80" />
                v0.1 · Rust ships today
              </span>
            </motion.div>

            <motion.h1
              variants={fadeRise}
              className="font-medium tracking-tight text-foreground"
              style={{
                fontSize: 'clamp(2.4rem, 6vw, 4.4rem)',
                lineHeight: 1.02,
                letterSpacing: '-0.025em',
              }}
            >
              Project scaffolding
              <br />
              <span className="text-muted-foreground">that respects your time.</span>
            </motion.h1>

            <motion.p
              variants={fadeRise}
              className="mt-7 max-w-md text-[15px] leading-relaxed text-muted-foreground"
            >
              Devora is a plugin-driven CLI for generating project boilerplate.
              Rust ships today. Other languages land when they&rsquo;re ready,
              not when a roadmap says so.
            </motion.p>

            {/* Install command: OS-aware, quiet, full width */}
            <motion.div variants={fadeRise} className="mt-10">
              <InstallCommand />
            </motion.div>

            {/* CTAs */}
            <motion.div variants={fadeRise} className="mt-6 flex flex-wrap items-center gap-2">
              <a
                href={GITHUB_URL}
                target="_blank"
                rel="noreferrer"
                className="group/btn inline-flex items-center gap-2 rounded-md bg-foreground px-4 py-2.5 text-sm font-medium text-background transition-[transform,opacity] duration-150 active:translate-y-px hover:opacity-90"
              >
                Star on GitHub
                <ArrowRight className="size-3.5 transition-transform duration-200 group-hover/btn:translate-x-0.5" />
              </a>

              <a
                href="#features"
                className="inline-flex items-center gap-2 rounded-md px-4 py-2.5 text-sm text-muted-foreground hover:text-foreground transition-colors"
              >
                What it does
                <ArrowUpRight className="size-3.5" />
              </a>
            </motion.div>
          </motion.div>

          {/* Right - terminal */}
          <motion.div
            initial={{ opacity: 0, y: 12 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.7, delay: 0.15, ease: EASE_OUT }}
            className="relative"
          >
            <TerminalAnimation />
            <p className="mt-3 text-center font-mono text-[11px] uppercase tracking-[0.18em] text-muted-foreground/60">
              hover to pause · click to restart
            </p>
          </motion.div>
        </div>
      </div>
    </section>
  );
}
