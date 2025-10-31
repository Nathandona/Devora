'use client';

import { motion } from 'framer-motion';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { FeatureCard } from './feature-card';
import { fadeInUp, staggerChildren } from '@/lib/animations';
import { FEATURES } from '@/lib/constants';

export function FeatureGrid() {
  return (
    <section id="features" className="py-20 px-4">
      <div className="container max-w-7xl mx-auto">
        <motion.div
          initial="initial"
          whileInView="animate"
          viewport={{ once: true }}
          variants={staggerChildren}
          className="text-center space-y-4 mb-16"
        >

          <motion.h2
            variants={fadeInUp}
            className="text-3xl md:text-5xl font-bold tracking-tight"
          >
            Built for Developers
          </motion.h2>

          <motion.p
            variants={fadeInUp}
            className="text-lg md:text-xl text-muted-foreground max-w-3xl mx-auto"
          >
            Everything you need to scaffold projects across any language.
            From simple templates to complex enterprise architectures.
          </motion.p>
        </motion.div>

        {/* Features Grid */}
        <motion.div
          initial="initial"
          whileInView="animate"
          viewport={{ once: true }}
          variants={staggerChildren}
          className="grid md:grid-cols-2 lg:grid-cols-3 gap-6"
        >
          {FEATURES.map((feature, index) => (
            <FeatureCard key={feature.title} feature={feature} index={index} />
          ))}
        </motion.div>
      </div>
    </section>
  );
}