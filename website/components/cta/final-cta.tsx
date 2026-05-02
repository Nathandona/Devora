'use client';

import { motion } from 'framer-motion';
import { toast } from 'sonner';
import { ArrowRight, Copy } from 'lucide-react';
import { fadeRise, stagger } from '@/lib/animations';
import { GITHUB_URL, INSTALL_COMMAND } from '@/lib/constants';

export function FinalCTA() {
  const copy = async () => {
    try {
      await navigator.clipboard.writeText(INSTALL_COMMAND);
      toast.success('Copied install command');
    } catch {
      toast.error('Could not copy. Long-press to select.');
    }
  };

  return (
    <section className="relative py-32 border-t border-border">
      <div className="container-tight">
        <motion.div
          initial="initial"
          whileInView="animate"
          viewport={{ once: true, amount: 0.4 }}
          variants={stagger}
          className="mx-auto max-w-xl text-center"
        >
          <motion.h2
            variants={fadeRise}
            className="font-medium tracking-tight"
            style={{
              fontSize: 'clamp(1.8rem, 3.4vw, 2.6rem)',
              lineHeight: 1.1,
              letterSpacing: '-0.02em',
            }}
          >
            One command away.
          </motion.h2>

          <motion.p
            variants={fadeRise}
            className="mt-5 text-[15px] leading-relaxed text-muted-foreground"
          >
            Free, MIT, no telemetry. Use it, or read the source first &mdash; we won&rsquo;t mind.
          </motion.p>

          <motion.div variants={fadeRise} className="mt-10">
            <button
              onClick={copy}
              className="group/copy mx-auto flex w-full max-w-md items-center justify-between gap-3 rounded-md bg-secondary/60 px-3.5 py-3 text-left ring-1 ring-border hover:ring-foreground/30 transition-[box-shadow] duration-200"
            >
              <span className="flex items-center gap-2.5 overflow-hidden">
                <span className="text-muted-foreground/60 font-mono text-xs select-none">$</span>
                <code className="font-mono text-[13px] truncate text-foreground/90">
                  {INSTALL_COMMAND}
                </code>
              </span>
              <span className="flex items-center gap-1.5 text-muted-foreground group-hover/copy:text-foreground transition-colors">
                <Copy className="size-3.5" />
                <span className="font-mono text-[11px] uppercase tracking-wider">copy</span>
              </span>
            </button>
          </motion.div>

          <motion.div variants={fadeRise} className="mt-5">
            <a
              href={GITHUB_URL}
              target="_blank"
              rel="noreferrer"
              className="group inline-flex items-center gap-1.5 text-sm text-muted-foreground hover:text-foreground transition-colors"
            >
              Read the source on GitHub
              <ArrowRight className="size-3.5 transition-transform group-hover:translate-x-0.5" />
            </a>
          </motion.div>
        </motion.div>
      </div>
    </section>
  );
}
