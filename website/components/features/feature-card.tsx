'use client';

import { motion } from 'framer-motion';
import { Card, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { FEATURES } from '@/lib/constants';
import { scaleIn } from '@/lib/animations';
import {
  Blocks,
  Zap,
  Layers,
  Puzzle,
  Sparkles,
  ArrowRight
} from 'lucide-react';

const iconMap = {
  Blocks,
  Zap,
  Layers,
  Puzzle,
  Sparkles,
  ArrowRight
};

interface FeatureCardProps {
  feature: typeof FEATURES[0];
  index: number;
}

export function FeatureCard({ feature, index }: FeatureCardProps) {
  const Icon = iconMap[feature.icon as keyof typeof iconMap];

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      whileInView={{ opacity: 1, y: 0 }}
      viewport={{ once: true }}
      transition={{
        duration: 0.6,
        delay: index * 0.1,
        ease: [0.25, 0.46, 0.45, 0.94]
      }}
    >
      <Card className="relative group overflow-hidden border-border/50 bg-card/50 backdrop-blur-sm hover:bg-card/80 transition-all duration-300 hover:shadow-lg hover:shadow-primary/10 hover:border-primary/30">
        <div className="absolute inset-0 bg-gradient-to-br from-primary/3 via-transparent to-primary/1 opacity-0 group-hover:opacity-100 transition-opacity duration-500" />

        <CardContent className="p-6 relative">
          <div className="space-y-4">
            {/* Icon */}
            <div className="inline-flex items-center justify-center w-12 h-12 rounded-xl bg-primary/8 border border-primary/15 group-hover:bg-primary/12 group-hover:scale-110 transition-all duration-300">
              <Icon className="w-6 h-6 text-primary/90" />
            </div>

            {/* Content */}
            <div className="space-y-2">
              <h3 className="text-xl font-semibold">{feature.title}</h3>
              <p className="text-muted-foreground leading-relaxed">
                {feature.description}
              </p>
            </div>
          </div>
        </CardContent>
      </Card>
    </motion.div>
  );
}