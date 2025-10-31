'use client';

import { motion } from 'framer-motion';
import { Card, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { CheckCircle2, Clock, Package } from 'lucide-react';

interface LanguageCardProps {
  language: {
    name: string;
    status: 'available' | 'coming-soon' | 'planned';
    frameworks: number;
    accent: string;
    description: string;
  };
  index: number;
}

export function LanguageCard({ language, index }: LanguageCardProps) {
  const getStatusInfo = () => {
    switch (language.status) {
      case 'available':
        return {
          label: 'Available',
          variant: 'default' as const,
          icon: CheckCircle2,
          color: 'text-green-500',
          bgColor: 'bg-green-500/10',
          borderColor: 'border-green-500/20'
        };
      case 'coming-soon':
        return {
          label: 'Coming Soon',
          variant: 'secondary' as const,
          icon: Clock,
          color: 'text-yellow-500',
          bgColor: 'bg-yellow-500/10',
          borderColor: 'border-yellow-500/20'
        };
      case 'planned':
        return {
          label: 'Planned',
          variant: 'outline' as const,
          icon: Package,
          color: 'text-muted-foreground',
          bgColor: 'bg-muted/30',
          borderColor: 'border-border/50'
        };
    }
  };

  const statusInfo = getStatusInfo();
  const StatusIcon = statusInfo.icon;

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
      <Card
        className="relative group overflow-hidden border-border/50 bg-card/50 backdrop-blur-sm hover:bg-card/80 transition-all duration-300 hover:shadow-lg hover:border-opacity-50"
        style={{
          borderColor: language.status === 'available' ? `${language.accent}30` : undefined
        }}
      >
        {/* Gradient overlay */}
        <div
          className="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500"
          style={{
            background: language.status === 'available'
              ? `linear-gradient(135deg, ${language.accent}10 0%, transparent 50%, ${language.accent}05 100%)`
              : 'linear-gradient(135deg, var(--primary)10 0%, transparent 50%, var(--secondary)05 100%)'
          }}
        />

        <CardContent className="p-6 relative">
          <div className="space-y-4">
            {/* Header */}
            <div className="flex items-start justify-between">
              <div className="space-y-2">
                <h3 className="text-xl font-semibold flex items-center gap-2">
                  <span
                    className="w-3 h-3 rounded-full"
                    style={{ backgroundColor: language.accent }}
                  />
                  {language.name}
                </h3>
                <p className="text-sm text-muted-foreground">
                  {language.description}
                </p>
              </div>

              <Badge
                variant={statusInfo.variant}
                className={`${statusInfo.bgColor} ${statusInfo.borderColor} ${statusInfo.color} border`}
              >
                <StatusIcon className="w-3 h-3 mr-1" />
                {statusInfo.label}
              </Badge>
            </div>

            {/* Frameworks Info */}
            <div className="flex items-center justify-between">
              <div className="text-sm text-muted-foreground">
                {language.frameworks > 0 ? (
                  <>
                    <span className="font-medium">{language.frameworks}</span>
                    <span> framework{language.frameworks !== 1 ? 's' : ''} available</span>
                  </>
                ) : (
                  <span>No frameworks yet</span>
                )}
              </div>

              {language.status === 'available' && (
                <motion.div
                  initial={{ scale: 0 }}
                  animate={{ scale: 1 }}
                  transition={{ delay: 0.5 + index * 0.1 }}
                  className="text-xs text-green-500 font-medium"
                >
                  Ready to use
                </motion.div>
              )}
            </div>

            {/* Hover details */}
            <div className="space-y-2 opacity-0 group-hover:opacity-100 transition-opacity duration-300">
              {language.frameworks > 0 && (
                <div className="text-xs text-muted-foreground">
                  <span>Popular frameworks:</span>
                  <div className="mt-1 space-y-1">
                    <div>• Base project template</div>
                    {language.frameworks > 1 && <div>• +{language.frameworks - 1} more</div>}
                  </div>
                </div>
              )}
            </div>
          </div>
        </CardContent>
      </Card>
    </motion.div>
  );
}