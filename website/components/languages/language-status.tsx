'use client';

import { motion } from 'framer-motion';
import { ArrowUpRight } from 'lucide-react';
import { LANGUAGES, LangRow, GOOD_FIRST_ISSUE_URL, ISSUES_URL } from '@/lib/constants';
import { fadeRise, stagger } from '@/lib/animations';

const STATE_LABEL: Record<LangRow['state'], string> = {
  stable: 'stable',
  paused: 'paused',
  wishlist: 'wishlist',
};

function StatusDot({ state }: { state: LangRow['state'] }) {
  if (state === 'stable') {
    return (
      <span className="relative inline-flex">
        <span className="size-2 rounded-full bg-foreground" />
        <span className="absolute inset-0 size-2 rounded-full bg-foreground/40 animate-ping" />
      </span>
    );
  }
  return (
    <span
      className={`inline-block size-2 rounded-full ${
        state === 'paused' ? 'bg-foreground/30' : 'border border-foreground/30 bg-transparent'
      }`}
    />
  );
}

export function LanguageStatus() {
  return (
    <section id="languages" className="relative py-32 border-t border-border">
      <div className="container-tight">
        <motion.div
          initial="initial"
          whileInView="animate"
          viewport={{ once: true, amount: 0.4 }}
          variants={stagger}
          className="max-w-2xl"
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
            Languages.
            <br />
            <span className="text-muted-foreground">No calendar.</span>
          </motion.h2>
          <motion.p
            variants={fadeRise}
            className="mt-5 text-[15px] leading-relaxed text-muted-foreground max-w-md"
          >
            Rust ships today. The rest land when they ship. If a language matters
            to you, the fastest path is a plugin.
          </motion.p>
        </motion.div>

        <motion.div
          initial="initial"
          whileInView="animate"
          viewport={{ once: true, amount: 0.2 }}
          variants={stagger}
          className="relative mt-16 max-w-2xl"
        >
          {/* Vertical hairline */}
          <span
            aria-hidden
            className="absolute left-[3px] top-1.5 bottom-1.5 w-px bg-border"
          />

          <ul className="space-y-px">
            {LANGUAGES.map((row) => (
              <motion.li
                key={row.name}
                variants={fadeRise}
                className="group/row relative grid grid-cols-[14px_minmax(0,8rem)_minmax(0,7rem)_1fr] items-baseline gap-x-4 sm:gap-x-6 py-3.5"
              >
                <span className="row-span-1 self-center -ml-px translate-y-[1px]">
                  <StatusDot state={row.state} />
                </span>
                <span className="font-mono text-[13px] text-foreground">
                  {row.name}
                </span>
                <span
                  className={`font-mono text-[11px] uppercase tracking-[0.16em] ${
                    row.state === 'stable'
                      ? 'text-foreground/85'
                      : 'text-muted-foreground/70'
                  }`}
                >
                  {STATE_LABEL[row.state]}
                </span>
                <span
                  className={`text-sm ${
                    row.state === 'stable' ? 'text-foreground/80' : 'text-muted-foreground'
                  }`}
                >
                  {row.note}
                </span>
              </motion.li>
            ))}
          </ul>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 8 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true, amount: 0.6 }}
          transition={{ duration: 0.5 }}
          className="mt-12 max-w-2xl flex flex-wrap items-center gap-x-6 gap-y-2 text-sm text-muted-foreground"
        >
          <a
            href={GOOD_FIRST_ISSUE_URL}
            target="_blank"
            rel="noreferrer"
            className="inline-flex items-center gap-1.5 hover:text-foreground transition-colors"
          >
            Good first issues
            <ArrowUpRight className="size-3.5" />
          </a>
          <span className="text-muted-foreground/40">·</span>
          <a
            href={`${ISSUES_URL}/new?labels=language-request&template=language-request.md`}
            target="_blank"
            rel="noreferrer"
            className="inline-flex items-center gap-1.5 hover:text-foreground transition-colors"
          >
            Request a language
            <ArrowUpRight className="size-3.5" />
          </a>
        </motion.div>
      </div>
    </section>
  );
}
