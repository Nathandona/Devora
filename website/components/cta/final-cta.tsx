'use client';

import { motion } from 'framer-motion';
import { ArrowRight } from 'lucide-react';
import { fadeRise, stagger } from '@/lib/animations';
import { InstallCommand } from '@/components/install/install-command';
import { GITHUB_URL } from '@/lib/constants';

export function FinalCTA() {
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
            Free, MIT, no telemetry. Use it, or read the source first. We won&rsquo;t mind.
          </motion.p>

          <motion.div variants={fadeRise} className="mt-10">
            <InstallCommand className="mx-auto" />
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
