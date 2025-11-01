'use client';

import { motion } from 'framer-motion';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar';
import { Separator } from '@/components/ui/separator';
import { Skeleton } from '@/components/ui/skeleton';
import { fadeInUp, staggerChildren } from '@/lib/animations';
import { GITHUB_URL } from '@/lib/constants';
import { useGitHubStats } from '@/hooks/use-github-stats';
import { GitHubStats } from '@/lib/github-types';
import {
  Github,
  Star,
  Users,
  GitBranch,
  MessageCircle,
  Heart,
  ArrowRight,
  Activity,
  Clock,
  ExternalLink,
  AlertCircle
} from 'lucide-react';

// Stats card component with loading state
function StatsCard({
  icon: Icon,
  value,
  label,
  color,
  loading
}: {
  icon: any;
  value: number;
  label: string;
  color: string;
  loading: boolean;
}) {
  return (
    <motion.div
      whileHover={{ scale: 1.05 }}
      className="text-center p-6 rounded-2xl border border-border/30 bg-gradient-to-br from-card via-card/95 to-card/90 backdrop-blur-sm shadow-lg hover:shadow-xl transition-all duration-300"
    >
      <div className="flex items-center justify-center mb-3">
        <Icon className={`w-6 h-6 ${color}`} />
      </div>
      {loading ? (
        <Skeleton className="h-8 w-16 mx-auto mb-2" />
      ) : (
        <div className="text-3xl font-bold mb-1">{value.toLocaleString()}</div>
      )}
      <div className="text-sm text-muted-foreground">{label}</div>
    </motion.div>
  );
}

// Contributor card component
function ContributorCard({ contributor }: { contributor: any }) {
  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      whileInView={{ opacity: 1, y: 0 }}
      viewport={{ once: true }}
      whileHover={{ x: 4 }}
      className="flex items-center gap-4 p-4 rounded-xl hover:bg-accent/50 transition-all duration-300 cursor-pointer group"
      onClick={() => window.open(contributor.html_url, '_blank')}
    >
      <Avatar className="w-12 h-12 ring-2 ring-primary/10 group-hover:ring-primary/20 transition-all duration-300">
        <AvatarImage src={contributor.avatar_url} alt={contributor.name} />
        <AvatarFallback className="bg-gradient-to-br from-primary/20 to-primary/10">
          {contributor.name?.split(' ').map((n: string) => n[0]).join('') || contributor.login?.[0]}
        </AvatarFallback>
      </Avatar>

      <div className="flex-1">
        <div className="font-semibold text-foreground group-hover:text-primary transition-colors duration-300">
          {contributor.name || contributor.login}
        </div>
        <div className="text-sm text-muted-foreground">
          @{contributor.login}
        </div>
      </div>

      <div className="flex items-center gap-2">
        <div className="text-xs text-muted-foreground text-right">
          <div className="font-medium">{contributor.contributions}</div>
          <div>contributions</div>
        </div>
        <ExternalLink className="w-4 h-4 text-muted-foreground opacity-0 group-hover:opacity-100 transition-all duration-300" />
      </div>
    </motion.div>
  );
}

export function OpenSourceCTA() {
  const { data, loading, error, isCached, lastCached } = useGitHubStats();

  return (
    <section className="relative py-24 px-4 overflow-hidden">
      {/* Background decoration */}
      <div className="absolute inset-0 bg-gradient-to-br from-primary/3 via-transparent to-secondary/3" />
      <div className="absolute top-1/3 right-1/4 w-96 h-96 bg-primary/10 rounded-full blur-3xl" />

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
            Built by the Community
          </motion.h2>

          <motion.p
            variants={fadeInUp}
            className="text-lg md:text-xl text-muted-foreground max-w-4xl mx-auto leading-relaxed"
          >
            Devora is open source and powered by contributors like you.
            Join us in building the future of project scaffolding.
          </motion.p>

          {/* Status indicator */}
          {lastCached && (
            <motion.div
              variants={fadeInUp}
              className="flex items-center justify-center gap-2 text-xs text-muted-foreground"
            >
              <Clock className="w-3 h-3" />
              <span>
                {isCached ? 'Cached' : 'Live'} data • Updated {new Date(lastCached).toLocaleTimeString()}
              </span>
              {error && (
                <div className="flex items-center gap-1 text-yellow-600 ml-2">
                  <AlertCircle className="w-3 h-3" />
                  <span>Fallback data</span>
                </div>
              )}
            </motion.div>
          )}
        </motion.div>

        <div className="grid lg:grid-cols-2 gap-12 items-start">
          {/* Left Side - Stats and Info */}
          <motion.div
            initial={{ opacity: 0, x: -40 }}
            whileInView={{ opacity: 1, x: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.8 }}
            className="space-y-8"
          >
            {/* Stats Cards */}
            <div className="grid grid-cols-3 gap-4">
              <StatsCard
                icon={Star}
                value={data?.repository.stargazers_count || 0}
                label="Stars"
                color="text-yellow-500"
                loading={loading}
              />
              <StatsCard
                icon={GitBranch}
                value={data?.repository.forks_count || 0}
                label="Forks"
                color="text-blue-500"
                loading={loading}
              />
              <StatsCard
                icon={Users}
                value={data?.total_contributors || 0}
                label="Contributors"
                color="text-green-500"
                loading={loading}
              />
            </div>

            {/* Why Open Source Card */}
            <Card className="relative overflow-hidden border-border/30 bg-gradient-to-br from-card via-card/95 to-card/90 backdrop-blur-sm shadow-xl shadow-primary/5">
              <div className="absolute inset-0 bg-gradient-to-br from-primary/3 via-transparent to-secondary/3 opacity-50" />
              <CardContent className="p-8 relative">
                <div className="flex items-center gap-3 mb-6">
                  <div className="w-10 h-10 rounded-xl bg-primary/10 border border-primary/20 flex items-center justify-center">
                    <Github className="w-5 h-5 text-primary" />
                  </div>
                  <h3 className="text-2xl font-bold">Why Open Source?</h3>
                </div>

                <div className="space-y-4 text-muted-foreground">
                  <p className="text-base leading-relaxed">
                    Development tools should be transparent, extensible, and community-driven.
                    Devora's open source nature ensures:
                  </p>

                  <ul className="space-y-3">
                    <li className="flex items-start gap-3">
                      <div className="w-1.5 h-1.5 rounded-full bg-green-500 mt-2 flex-shrink-0" />
                      <span>Trust and transparency in code generation</span>
                    </li>
                    <li className="flex items-start gap-3">
                      <div className="w-1.5 h-1.5 rounded-full bg-green-500 mt-2 flex-shrink-0" />
                      <span>Community-driven language support</span>
                    </li>
                    <li className="flex items-start gap-3">
                      <div className="w-1.5 h-1.5 rounded-full bg-green-500 mt-2 flex-shrink-0" />
                      <span>Custom templates and plugins</span>
                    </li>
                    <li className="flex items-start gap-3">
                      <div className="w-1.5 h-1.5 rounded-full bg-green-500 mt-2 flex-shrink-0" />
                      <span>Freedom to modify and extend</span>
                    </li>
                  </ul>
                </div>

                <Separator className="my-6" />

                <div className="flex flex-col sm:flex-row gap-4">
                  <Button
                    onClick={() => window.open(GITHUB_URL, '_blank')}
                    className="bg-primary hover:bg-primary/90 shadow-lg hover:shadow-xl transition-all duration-300"
                    size="lg"
                  >
                    <Github className="w-4 h-4 mr-2" />
                    View Repository
                    <ArrowRight className="w-4 h-4 ml-2" />
                  </Button>

                  <Button
                    variant="outline"
                    onClick={() => window.open(`${GITHUB_URL}/blob/main/CONTRIBUTING.md`, '_blank')}
                    size="lg"
                    className="border-primary/20 hover:border-primary/40 hover:bg-primary/5"
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
            <Card className="relative overflow-hidden border-border/30 bg-gradient-to-br from-card via-card/95 to-card/90 backdrop-blur-sm shadow-xl shadow-primary/5">
              <CardContent className="p-8">
                <div className="flex items-center justify-between mb-8">
                  <div className="flex items-center gap-3">
                    <div className="w-10 h-10 rounded-xl bg-primary/10 border border-primary/20 flex items-center justify-center">
                      <Users className="w-5 h-5 text-primary" />
                    </div>
                    <h3 className="text-2xl font-bold">Recent Contributors</h3>
                  </div>
                  <Badge variant="secondary" className="px-3 py-1">
                    {data?.total_contributors || 0} total
                  </Badge>
                </div>

                <div className="space-y-3">
                  {loading ? (
                    // Loading skeletons
                    Array.from({ length: 5 }).map((_, i) => (
                      <div key={i} className="flex items-center gap-4 p-4">
                        <Skeleton className="w-12 h-12 rounded-full" />
                        <div className="flex-1">
                          <Skeleton className="h-4 w-24 mb-2" />
                          <Skeleton className="h-3 w-32" />
                        </div>
                        <Skeleton className="h-8 w-16" />
                      </div>
                    ))
                  ) : (
                    data?.contributors.map((contributor, index) => (
                      <ContributorCard key={contributor.id} contributor={contributor} />
                    ))
                  )}
                </div>

                {!loading && (
                  <div className="mt-8 pt-6 border-t border-border/30">
                    <Button
                      variant="ghost"
                      onClick={() => window.open(`${GITHUB_URL}/graphs/contributors`, '_blank')}
                      className="w-full text-primary hover:text-primary/80 hover:bg-primary/5 transition-all duration-300"
                      size="lg"
                    >
                      View all contributors
                      <ArrowRight className="w-4 h-4 ml-2" />
                    </Button>
                  </div>
                )}
              </CardContent>
            </Card>
          </motion.div>
        </div>
      </div>
    </section>
  );
}