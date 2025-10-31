'use client';

import { motion } from 'framer-motion';
import { Badge } from '@/components/ui/badge';
import { Card, CardContent } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { FeatureCard } from './feature-card';
import { fadeInUp, staggerChildren } from '@/lib/animations';
import { FEATURES } from '@/lib/constants';
import { Sparkles, Zap, Shield } from 'lucide-react';

export function FeatureGrid() {
  return (
    <section id="features" className="relative py-24 px-4 overflow-hidden">
      {/* Background decoration */}
      <div className="absolute inset-0 bg-gradient-to-br from-primary/5 via-transparent to-secondary/5" />
      <div className="absolute top-1/4 left-1/4 w-96 h-96 bg-primary/10 rounded-full blur-3xl" />
      <div className="absolute bottom-1/4 right-1/4 w-96 h-96 bg-secondary/10 rounded-full blur-3xl" />

      <div className="container max-w-7xl mx-auto relative">
        {/* Section Header */}
        <motion.div
          initial="initial"
          whileInView="animate"
          viewport={{ once: true }}
          variants={staggerChildren}
          className="text-center space-y-6 mb-20"
        >

          <motion.h2
            variants={fadeInUp}
            className="text-3xl md:text-5xl font-bold tracking-tight"
          >
            Built for Developers, by Developers
          </motion.h2>

          <motion.p
            variants={fadeInUp}
            className="text-lg md:text-xl text-muted-foreground max-w-4xl mx-auto leading-relaxed"
          >
            Everything you need to scaffold projects across any language.
            From simple templates to complex enterprise architectures.
          </motion.p>
        </motion.div>

        {/* Features Layout - Asymmetrical Grid */}
        <motion.div
          initial="initial"
          whileInView="animate"
          viewport={{ once: true }}
          variants={staggerChildren}
          className="relative"
        >
          {/* Floating decorative elements */}
          <motion.div
            initial={{ opacity: 0, scale: 0 }}
            whileInView={{ opacity: 0.6, scale: 1 }}
            viewport={{ once: true }}
            transition={{ duration: 1, delay: 0.5 }}
            className="absolute -top-10 left-10 w-20 h-20 rounded-2xl bg-gradient-to-br from-primary/20 to-primary/5 rotate-12 blur-sm"
          />
          <motion.div
            initial={{ opacity: 0, scale: 0 }}
            whileInView={{ opacity: 0.4, scale: 1 }}
            viewport={{ once: true }}
            transition={{ duration: 1, delay: 0.7 }}
            className="absolute -bottom-10 right-10 w-32 h-32 rounded-3xl bg-gradient-to-br from-secondary/20 to-secondary/5 -rotate-6 blur-sm"
          />

          {/* Main feature cards with unique positioning */}
          <div className="grid grid-cols-1 lg:grid-cols-12 gap-8 items-start">
            {/* Featured main card */}
            <motion.div
              variants={fadeInUp}
              className="lg:col-span-12"
            >
              <Card className="relative overflow-hidden border-border/30 bg-gradient-to-br from-card via-card/95 to-card/90 backdrop-blur-sm shadow-2xl shadow-primary/5 group hover:shadow-primary/10 transition-all duration-500">
                <div className="absolute inset-0 bg-gradient-to-br from-primary/3 via-transparent to-secondary/3 opacity-0 group-hover:opacity-100 transition-opacity duration-500" />
                <CardContent className="p-8 lg:p-12 relative">
                  <div className="grid lg:grid-cols-2 gap-8 items-center">
                    <div className="space-y-6">
                      <div className="space-y-4">
                        <h3 className="text-2xl lg:text-3xl font-bold">
                          {FEATURES[1].title}
                        </h3>
                        <p className="text-lg text-muted-foreground leading-relaxed">
                          {FEATURES[1].description}
                        </p>
                      </div>
                    </div>
                  </div>
                </CardContent>
              </Card>
            </motion.div>

            {/* Two smaller feature cards */}
            <motion.div
              variants={fadeInUp}
              className="lg:col-span-6 lg:translate-y-8"
            >
              <FeatureCard feature={FEATURES[0]} index={0} featured={false} />
            </motion.div>

            <motion.div
              variants={fadeInUp}
              className="lg:col-span-6 lg:-translate-y-8"
            >
              <FeatureCard feature={FEATURES[2]} index={2} featured={false} />
            </motion.div>
          </div>
        </motion.div>
      </div>
    </section>
  );
}