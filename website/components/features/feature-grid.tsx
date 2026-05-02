'use client';

import { motion } from 'framer-motion';
import { FEATURES } from '@/lib/constants';
import { fadeRise, stagger } from '@/lib/animations';
import { FeatureCard } from './feature-card';

export function FeatureGrid() {
  return (
    <section id="features" className="relative py-32">
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
            Three pieces.
            <br />
            <span className="text-muted-foreground">Nothing else.</span>
          </motion.h2>
          <motion.p
            variants={fadeRise}
            className="mt-5 text-[15px] leading-relaxed text-muted-foreground max-w-md"
          >
            Plugins for languages. Templates that ask. Hooks that finish the job.
            That&rsquo;s the whole tool.
          </motion.p>
        </motion.div>

        <motion.div
          initial="initial"
          whileInView="animate"
          viewport={{ once: true, amount: 0.15 }}
          variants={stagger}
          className="mt-16 grid gap-px overflow-hidden rounded-lg ring-1 ring-border bg-border md:grid-cols-3"
        >
          {FEATURES.map((feature, i) => (
            <FeatureCard key={feature.title} feature={feature} index={i} />
          ))}
        </motion.div>
      </div>
    </section>
  );
}
