'use client';

import { motion } from 'framer-motion';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { toast } from 'sonner';
import { fadeInUp, scaleIn } from '@/lib/animations';
import { INSTALL_COMMAND, GITHUB_URL } from '@/lib/constants';
import { Terminal, ArrowRight, Copy, Sparkles } from 'lucide-react';

export function FinalCTA() {
  const copyToClipboard = async () => {
    try {
      await navigator.clipboard.writeText(INSTALL_COMMAND);
      toast.success('Command copied to clipboard!');
    } catch (error) {
      toast.error('Failed to copy command to clipboard');
    }
  };

  return (
    <section className="py-20 px-4 relative overflow-hidden">
      {/* Background Effects */}
      <div className="absolute inset-0 bg-gradient-to-br from-primary/5 via-transparent to-secondary/5" />

      {/* Floating elements */}
      <motion.div
        animate={{
          y: [-20, 20, -20],
          rotate: [-5, 5, -5],
        }}
        transition={{
          duration: 10,
          repeat: Infinity,
          ease: "easeInOut"
        }}
        className="absolute top-20 left-10 w-32 h-32 bg-gradient-to-br from-primary/20 to-secondary/20 rounded-full blur-2xl"
      />

      <motion.div
        animate={{
          y: [20, -20, 20],
          rotate: [5, -5, 5],
        }}
        transition={{
          duration: 12,
          repeat: Infinity,
          ease: "easeInOut",
          delay: 2
        }}
        className="absolute bottom-20 right-10 w-40 h-40 bg-gradient-to-tr from-cyan-500/20 to-blue-500/20 rounded-full blur-2xl"
      />

      <div className="container max-w-4xl mx-auto relative z-10">
        <motion.div
          initial="initial"
          whileInView="animate"
          viewport={{ once: true }}
          variants={fadeInUp}
          className="text-center space-y-8"
        >

          {/* Main Headline */}
          <motion.h2
            variants={fadeInUp}
            className="text-3xl md:text-5xl font-bold tracking-tight"
          >
            Stop copy-pasting. Start creating.
          </motion.h2>

          {/* Description */}
          <motion.p
            variants={fadeInUp}
            className="text-lg md:text-xl text-muted-foreground max-w-2xl mx-auto leading-relaxed"
          >
            Join thousands of developers who've streamlined their workflow with Devora.
            Your next project is just one command away.
          </motion.p>

          {/* Install Command */}
          <motion.div
            variants={scaleIn}
            className="max-w-2xl mx-auto"
          >
            <div className="p-6 rounded-xl border border-border/50 bg-card/50 backdrop-blur-sm space-y-4">
              <div className="flex items-center gap-3 text-sm text-muted-foreground">
                <Terminal className="w-4 h-4" />
                <span>Quick install:</span>
              </div>

              <div className="flex items-center gap-3 p-4 rounded-lg bg-black/50 border border-border/30">
                <code className="font-mono text-sm md:text-base flex-1 text-foreground/90">
                  {INSTALL_COMMAND}
                </code>
                <Button
                  size="sm"
                  variant="ghost"
                  onClick={copyToClipboard}
                  className="h-8 px-3 hover:bg-accent/50 text-muted-foreground hover:text-foreground"
                >
                  <Copy className="w-4 h-4" />
                </Button>
              </div>
            </div>
          </motion.div>

          {/* CTA Buttons */}
          <motion.div
            variants={scaleIn}
            className="flex flex-col sm:flex-row gap-4 justify-center items-center"
          >
            <Button
              size="lg"
              onClick={() => window.open(GITHUB_URL, '_blank')}
              className="bg-primary hover:bg-primary/90 text-primary-foreground px-8 py-6 text-lg h-auto"
            >
              Get Started Now
              <ArrowRight className="w-5 h-5 ml-2" />
            </Button>

            <Button
              variant="outline"
              size="lg"
              onClick={() => window.open(`${GITHUB_URL}/blob/main/README.md`, '_blank')}
              className="border-border/50 hover:bg-accent/50 px-8 py-6 text-lg h-auto"
            >
              Read the Docs
              <ArrowRight className="w-5 h-5 ml-2" />
            </Button>
          </motion.div>
        </motion.div>
      </div>
    </section>
  );
}