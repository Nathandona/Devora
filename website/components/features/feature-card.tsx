'use client';

import { motion } from 'framer-motion';
import { Feature } from '@/lib/constants';
import { fadeRise } from '@/lib/animations';

interface Props {
  feature: Feature;
  index: number;
}

export function FeatureCard({ feature, index }: Props) {
  return (
    <motion.div
      variants={fadeRise}
      className="relative bg-background p-6 lg:p-8 group/card"
    >
      <div className="flex items-baseline gap-3">
        <span className="font-mono text-[11px] tabular-nums text-muted-foreground/60">
          {String(index + 1).padStart(2, '0')}
        </span>
        <h3 className="text-base font-medium tracking-tight text-foreground">
          {feature.title}
        </h3>
      </div>

      <p className="mt-3 text-sm leading-relaxed text-muted-foreground">
        {feature.body}
      </p>

      <pre
        className="mt-6 overflow-x-auto rounded-md bg-terminal ring-1 ring-border/80 p-4 font-mono text-[12px] leading-[1.7] text-foreground/85 group-hover/card:ring-foreground/20 transition-[box-shadow] duration-200"
        style={{ fontFeatureSettings: '"liga" 0, "calt" 0' }}
      >
        <code>{feature.code}</code>
      </pre>
    </motion.div>
  );
}
