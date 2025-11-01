'use client';

import { motion } from 'framer-motion';
import { Badge } from '@/components/ui/badge';
import { Card, CardContent } from '@/components/ui/card';
import { Progress } from '@/components/ui/progress';
import { fadeInUp, staggerChildren } from '@/lib/animations';
import {
  CheckCircle2,
  Clock,
  Calendar,
  ArrowRight,
  Package,
  Sparkles
} from 'lucide-react';

interface RoadmapPhase {
  id: string;
  name: string;
  description: string;
  status: 'completed' | 'in-progress' | 'upcoming';
  quarter: string;
  languages: Array<{
    name: string;
    description: string;
    accent: string;
    frameworks?: number;
  }>;
  progress: number;
}

const roadmapPhases: RoadmapPhase[] = [
  {
    id: 'phase-1',
    name: 'Foundation',
    description: 'Core systems programming support',
    status: 'completed',
    quarter: 'Q4 2025',
    languages: [
      {
        name: 'Rust',
        description: 'Systems programming language',
        accent: '#b8653f',
        frameworks: 1
      }
    ],
    progress: 100
  },
  {
    id: 'phase-2',
    name: 'Expansion',
    description: 'High-performance and enterprise languages',
    status: 'in-progress',
    quarter: 'Q4 2025',
    languages: [
      {
        name: 'C++',
        description: 'High-performance systems programming',
        accent: '#4a7ba7',
        frameworks: 0
      },
      {
        name: 'Go',
        description: 'Simple, reliable, and efficient',
        accent: '#5a9caa',
        frameworks: 0
      }
    ],
    progress: 65
  },
  {
    id: 'phase-3',
    name: 'Web & Scripting',
    description: 'Modern web development and scripting languages',
    status: 'upcoming',
    quarter: 'Q1 2026',
    languages: [
      {
        name: 'Python',
        description: 'Versatile programming language',
        accent: '#5a85b5',
        frameworks: 0
      },
      {
        name: 'TypeScript',
        description: 'Typed JavaScript for modern apps',
        accent: '#5a8ac8',
        frameworks: 0
      }
    ],
    progress: 25
  },
  {
    id: 'phase-4',
    name: 'Innovation',
    description: 'Next-generation and experimental languages',
    status: 'upcoming',
    quarter: 'Q1 2026',
    languages: [
      { 
        name: 'Zig',
        description: 'Simple, fast, and safe',
        accent: '#c4a568',
        frameworks: 0
      }
    ],
    progress: 10
  }
];

export function RoadmapTimeline() {
  const getStatusIcon = (status: RoadmapPhase['status']) => {
    switch (status) {
      case 'completed':
        return <CheckCircle2 className="w-5 h-5 text-green-500" />;
      case 'in-progress':
        return <Clock className="w-5 h-5 text-yellow-500" />;
      case 'upcoming':
        return <Package className="w-5 h-5 text-muted-foreground" />;
    }
  };

  const getStatusBadge = (status: RoadmapPhase['status']) => {
    switch (status) {
      case 'completed':
        return <Badge variant="default" className="bg-green-500/10 text-green-600 border-green-500/20">Completed</Badge>;
      case 'in-progress':
        return <Badge variant="secondary" className="bg-yellow-500/10 text-yellow-600 border-yellow-500/20">In Progress</Badge>;
      case 'upcoming':
        return <Badge variant="outline">Planned</Badge>;
    }
  };

  return (
    <section className="relative py-24 px-4 overflow-hidden">
      {/* Background decoration */}
      <div className="absolute inset-0 bg-gradient-to-br from-primary/3 via-transparent to-secondary/3" />
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
            Our plan
          </motion.h2>

          <motion.p
            variants={fadeInUp}
            className="text-lg md:text-xl text-muted-foreground max-w-4xl mx-auto leading-relaxed"
          >
            From systems programming to web development.
            Our roadmap shows how we're expanding Devora's language support over time.
          </motion.p>
        </motion.div>

        {/* Timeline */}
        <motion.div
          initial="initial"
          whileInView="animate"
          viewport={{ once: true }}
          variants={staggerChildren}
          className="relative"
        >
          {/* Timeline line */}
          <div className="absolute left-8 md:left-1/2 top-0 bottom-0 w-0.5 bg-gradient-to-b from-primary/50 via-primary/20 to-secondary/50 transform md:-translate-x-0.5" />

          {/* Timeline phases */}
          <div className="space-y-16">
            {roadmapPhases.map((phase, index) => (
              <motion.div
                key={phase.id}
                variants={fadeInUp}
                className={`relative flex items-center ${index % 2 === 0 ? 'md:flex-row' : 'md:flex-row-reverse'} gap-8`}
              >
                {/* Timeline dot */}
                <div className="absolute left-8 md:left-1/2 w-6 h-6 -translate-x-1/2 transform md:-translate-x-1/2">
                  <div className={`w-6 h-6 rounded-full border-4 ${
                    phase.status === 'completed'
                      ? 'bg-green-500 border-green-500/20'
                      : phase.status === 'in-progress'
                      ? 'bg-yellow-500 border-yellow-500/20 animate-pulse'
                      : 'bg-muted border-border'
                  }`} />
                </div>

                {/* Phase content */}
                <div className={`ml-16 md:ml-0 ${index % 2 === 0 ? 'md:mr-auto md:pr-12' : 'md:ml-auto md:pl-12'} md:w-5/12 lg:w-6/12`}>
                  <Card className="relative overflow-hidden border-border/30 bg-gradient-to-br from-card via-card/95 to-card/90 backdrop-blur-sm shadow-xl hover:shadow-2xl transition-all duration-500 group">
                    {/* Gradient overlay */}
                    <div className="absolute inset-0 bg-gradient-to-br from-primary/3 via-transparent to-secondary/3 opacity-0 group-hover:opacity-100 transition-opacity duration-500" />

                    <CardContent className="p-8 relative">
                      <div className="space-y-6">
                        {/* Phase header */}
                        <div className="flex items-start justify-between">
                          <div className="space-y-3">
                            <div className="flex items-center gap-3">
                              {getStatusIcon(phase.status)}
                              <h3 className="text-2xl font-bold">{phase.name}</h3>
                            </div>
                            <p className="text-muted-foreground leading-relaxed">
                              {phase.description}
                            </p>
                          </div>
                          {getStatusBadge(phase.status)}
                        </div>

                        {/* Progress bar */}
                        <div className="space-y-2">
                          <div className="flex items-center justify-between text-sm">
                            <span className="text-muted-foreground">Progress</span>
                            <span className="font-medium">{phase.progress}%</span>
                          </div>
                          <Progress
                            value={phase.progress}
                            className={`h-2 ${
                              phase.status === 'completed'
                                ? '[&>div]:bg-green-500'
                                : phase.status === 'in-progress'
                                ? '[&>div]:bg-yellow-500'
                                : ''
                            }`}
                          />
                        </div>

                        {/* Languages in this phase */}
                        <div className="space-y-4">
                          <div className="flex items-center gap-2">
                            <span className="font-medium">Languages in this phase:</span>
                          </div>
                          <div className="grid gap-3">
                            {phase.languages.map((language) => (
                              <div
                                key={language.name}
                                className="flex items-center justify-between p-4 rounded-xl border border-border/20 bg-accent/30 transition-colors duration-300"
                              >
                                <div className="flex items-center gap-3">
                                  <div
                                    className="w-3 h-3 rounded-full"
                                    style={{ backgroundColor: language.accent }}
                                  />
                                  <div>
                                    <div className="font-medium">
                                      {language.name}
                                    </div>
                                    <div className="text-sm text-muted-foreground">
                                      {language.description}
                                    </div>
                                  </div>
                                </div>
                                {language.frameworks !== undefined && (
                                  <Badge variant="outline" className="text-xs">
                                    {language.frameworks} framework{language.frameworks !== 1 ? 's' : ''}
                                  </Badge>
                                )}
                              </div>
                            ))}
                          </div>
                        </div>

                        {/* Timeline info */}
                        <div className="flex items-center gap-2 pt-4 border-t border-border/20">
                          <Calendar className="w-4 h-4 text-muted-foreground" />
                          <span className="text-sm text-muted-foreground">{phase.quarter}</span>
                          {phase.status === 'in-progress' && (
                            <Badge variant="secondary" className="ml-auto text-xs">
                              Currently Active
                            </Badge>
                          )}
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                </div>
              </motion.div>
            ))}
          </div>
        </motion.div>

        {/* Bottom CTA */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ delay: 0.8 }}
          className="mt-24 text-center"
        >
          <Card className="relative overflow-hidden border-border/30 bg-gradient-to-br from-card via-card/95 to-card/90 backdrop-blur-sm shadow-xl">
            <CardContent className="p-12">
              <div className="space-y-6 max-w-3xl mx-auto">
                <h3 className="text-3xl font-bold">Want to influence our roadmap?</h3>
                <p className="text-lg text-muted-foreground leading-relaxed">
                  Have a language you'd like to see supported?
                  Join our community and help shape the future of Devora.
                </p>
                <div className="flex flex-col sm:flex-row gap-4 justify-center">
                  <button
                    onClick={() => window.open('https://github.com/Nathandona/devora/issues/new', '_blank')}
                    className="px-8 py-3 bg-primary hover:bg-primary/90 text-primary-foreground rounded-lg font-medium transition-all duration-300 shadow-lg hover:shadow-xl"
                  >
                    Request a Language
                    <ArrowRight className="w-4 h-4 ml-2 inline" />
                  </button>
                  <button
                    onClick={() => window.open('https://github.com/Nathandona/devora/blob/main/CONTRIBUTING.md', '_blank')}
                    className="px-8 py-3 border border-primary/20 hover:border-primary/40 hover:bg-primary/5 text-foreground rounded-lg font-medium transition-all duration-300"
                  >
                    Contribute to Development
                  </button>
                </div>
              </div>
            </CardContent>
          </Card>
        </motion.div>
      </div>
    </section>
  );
}