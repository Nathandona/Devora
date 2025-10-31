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
  featured?: boolean;
}

export function FeatureCard({ feature, index, featured = true }: FeatureCardProps) {
  const Icon = iconMap[feature.icon as keyof typeof iconMap];

  return (
    <motion.div
      initial={{ opacity: 0, y: 20, scale: 0.95 }}
      whileInView={{ opacity: 1, y: 0, scale: 1 }}
      viewport={{ once: true }}
      whileHover={{ y: -4, scale: 1.02 }}
      transition={{
        duration: 0.6,
        delay: index * 0.1,
        ease: [0.25, 0.46, 0.45, 0.94]
      }}
    >
      <Card className="relative group overflow-hidden border-border/30 bg-gradient-to-br from-card via-card/95 to-card/90 backdrop-blur-sm shadow-xl shadow-primary/5 hover:shadow-2xl hover:shadow-primary/10 transition-all duration-500 hover:border-primary/20">
        {/* Gradient overlay on hover */}
        <div className="absolute inset-0 bg-gradient-to-br from-primary/5 via-transparent to-secondary/5 opacity-0 group-hover:opacity-100 transition-opacity duration-500" />

        {/* Animated background pattern */}
        <div className="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-700">
          <div className="absolute top-0 right-0 w-32 h-32 bg-gradient-to-br from-primary/10 to-transparent rounded-full blur-2xl transform translate-x-16 -translate-y-16" />
          <div className="absolute bottom-0 left-0 w-24 h-24 bg-gradient-to-tr from-secondary/10 to-transparent rounded-full blur-xl transform -translate-x-12 translate-y-12" />
        </div>

        <CardContent className="p-6 lg:p-8 relative">
          <div className="space-y-6">
            {/* Icon with enhanced animation */}
            <div className="relative">
              <div className="absolute inset-0 bg-gradient-to-br from-primary/20 to-primary/5 rounded-2xl blur-lg opacity-0 group-hover:opacity-100 transition-all duration-500 scale-110" />
              <div className="relative inline-flex items-center justify-center w-14 h-14 lg:w-16 lg:h-16 rounded-2xl bg-gradient-to-br from-primary/10 to-primary/5 border border-primary/20 group-hover:scale-110 transition-all duration-300">
                <Icon className="w-7 h-7 lg:w-8 lg:h-8 text-primary" />
              </div>
            </div>

            {/* Content with improved typography */}
            <div className="space-y-3">
              <div className="flex items-center gap-3">
                <h3 className="text-xl lg:text-2xl font-bold group-hover:text-primary transition-colors duration-300">
                  {feature.title}
                </h3>
              </div>
              <p className="text-muted-foreground leading-relaxed text-sm lg:text-base">
                {feature.description}
              </p>
            </div>

            {/* Enhanced footer with subtle indicators */}
            <div className="flex items-center justify-between pt-2">
              <div className="flex items-center gap-2 text-xs text-muted-foreground/70">
                <div className="w-1 h-1 rounded-full bg-current" />
                <span>Core feature</span>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>
    </motion.div>
  );
}