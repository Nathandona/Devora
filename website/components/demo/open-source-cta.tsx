'use client';

import { motion } from 'framer-motion';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar';
import { Separator } from '@/components/ui/separator';
import { fadeInUp } from '@/lib/animations';
import { GITHUB_URL } from '@/lib/constants';
import {
  Github,
  Star,
  Users,
  GitBranch,
  MessageCircle,
  Heart,
  ArrowRight
} from 'lucide-react';

// Mock contributor data - in real app, this would come from GitHub API
const contributors = [
  { name: 'Nathan Dona', avatar: 'https://github.com/Nathandona.png', username: 'Nathandona' },
  { name: 'Alex Chen', avatar: '', username: 'alexchen' },
  { name: 'Sarah Johnson', avatar: '', username: 'sarahj' },
  { name: 'Mike Wilson', avatar: '', username: 'mikew' },
  { name: 'Emma Davis', avatar: '', username: 'emmad' },
];

export function OpenSourceCTA() {
  const stats = {
    stars: 48,
    forks: 12,
    contributors: contributors.length,
  };

  return (
    <section className="py-20 px-4">
      <div className="container max-w-7xl mx-auto">
        <motion.div
          initial="initial"
          whileInView="animate"
          viewport={{ once: true }}
          variants={fadeInUp}
          className="text-center space-y-4 mb-16"
        >
          <h2 className="text-3xl md:text-5xl font-bold tracking-tight">
            Built by the Community
          </h2>
          <p className="text-lg md:text-xl text-muted-foreground max-w-3xl mx-auto">
            Devora is open source and powered by contributors like you.
            Join us in building the future of project scaffolding.
          </p>
        </motion.div>

        <div className="grid lg:grid-cols-2 gap-12 items-center">
          {/* Left Side - Stats and Info */}
          <motion.div
            initial={{ opacity: 0, x: -40 }}
            whileInView={{ opacity: 1, x: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.8 }}
            className="space-y-8"
          >
            <div className="grid grid-cols-3 gap-6">
              <motion.div
                whileHover={{ scale: 1.05 }}
                className="text-center p-4 rounded-xl border border-border/50 bg-card/30 backdrop-blur-sm"
              >
                <div className="flex items-center justify-center mb-2">
                  <Star className="w-5 h-5 text-yellow-500" />
                </div>
                <div className="text-2xl font-bold">{stats.stars}</div>
                <div className="text-sm text-muted-foreground">Stars</div>
              </motion.div>

              <motion.div
                whileHover={{ scale: 1.05 }}
                className="text-center p-4 rounded-xl border border-border/50 bg-card/30 backdrop-blur-sm"
              >
                <div className="flex items-center justify-center mb-2">
                  <GitBranch className="w-5 h-5 text-blue-500" />
                </div>
                <div className="text-2xl font-bold">{stats.forks}</div>
                <div className="text-sm text-muted-foreground">Forks</div>
              </motion.div>

              <motion.div
                whileHover={{ scale: 1.05 }}
                className="text-center p-4 rounded-xl border border-border/50 bg-card/30 backdrop-blur-sm"
              >
                <div className="flex items-center justify-center mb-2">
                  <Users className="w-5 h-5 text-green-500" />
                </div>
                <div className="text-2xl font-bold">{stats.contributors}</div>
                <div className="text-sm text-muted-foreground">Contributors</div>
              </motion.div>
            </div>

            <Card className="border-border/50 bg-card/30 backdrop-blur-sm">
              <CardContent className="p-6 space-y-4">
                <div className="flex items-center gap-3">
                  <Github className="w-6 h-6 text-primary" />
                  <h3 className="text-xl font-semibold">Why Open Source?</h3>
                </div>

                <div className="space-y-3 text-muted-foreground">
                  <p>
                    Development tools should be transparent, extensible, and community-driven.
                    Devora's open source nature ensures:
                  </p>

                  <ul className="space-y-2">
                    <li className="flex items-start gap-2">
                      <span className="text-green-500 mt-0.5">✓</span>
                      <span>Trust and transparency in code generation</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-green-500 mt-0.5">✓</span>
                      <span>Community-driven language support</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-green-500 mt-0.5">✓</span>
                      <span>Custom templates and plugins</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-green-500 mt-0.5">✓</span>
                      <span>Freedom to modify and extend</span>
                    </li>
                  </ul>
                </div>

                <Separator />

                <div className="flex items-center gap-4">
                  <Button
                    onClick={() => window.open(GITHUB_URL, '_blank')}
                    className="bg-primary hover:bg-primary/90"
                  >
                    <Github className="w-4 h-4 mr-2" />
                    View Repository
                    <ArrowRight className="w-4 h-4 ml-2" />
                  </Button>

                  <Button
                    variant="outline"
                    onClick={() => window.open(`${GITHUB_URL}/blob/main/CONTRIBUTING.md`, '_blank')}
                  >
                    <Heart className="w-4 h-4 mr-2" />
                    Contribute
                  </Button>
                </div>
              </CardContent>
            </Card>
          </motion.div>

          {/* Right Side - Contributors */}
          <motion.div
            initial={{ opacity: 0, x: 40 }}
            whileInView={{ opacity: 1, x: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.8, delay: 0.2 }}
            className="space-y-6"
          >
            <Card className="border-border/50 bg-card/30 backdrop-blur-sm">
              <CardContent className="p-6">
                <h3 className="text-xl font-semibold mb-6 flex items-center gap-2">
                  <Users className="w-5 h-5 text-primary" />
                  Recent Contributors
                </h3>

                <div className="space-y-4">
                  {contributors.map((contributor, index) => (
                    <motion.div
                      key={contributor.username}
                      initial={{ opacity: 0, y: 20 }}
                      whileInView={{ opacity: 1, y: 0 }}
                      viewport={{ once: true }}
                      transition={{ delay: index * 0.1 }}
                      className="flex items-center gap-3 p-3 rounded-lg hover:bg-accent/50 transition-colors"
                    >
                      <Avatar className="w-10 h-10">
                        <AvatarImage src={contributor.avatar} alt={contributor.name} />
                        <AvatarFallback>
                          {contributor.name.split(' ').map(n => n[0]).join('')}
                        </AvatarFallback>
                      </Avatar>

                      <div className="flex-1">
                        <div className="font-medium">{contributor.name}</div>
                        <div className="text-sm text-muted-foreground">
                          @{contributor.username}
                        </div>
                      </div>

                      <div className="text-xs text-muted-foreground">
                        Active contributor
                      </div>
                    </motion.div>
                  ))}
                </div>

                <div className="mt-6 text-center">
                  <Button
                    variant="ghost"
                    onClick={() => window.open(`${GITHUB_URL}/graphs/contributors`, '_blank')}
                    className="text-primary hover:text-primary/80"
                  >
                    View all contributors
                    <ArrowRight className="w-4 h-4 ml-2" />
                  </Button>
                </div>
              </CardContent>
            </Card>
          </motion.div>
        </div>
      </div>
    </section>
  );
}