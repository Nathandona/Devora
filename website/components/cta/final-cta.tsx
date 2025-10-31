'use client';

import { motion } from 'framer-motion';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { fadeInUp, scaleIn } from '@/lib/animations';
import { INSTALL_COMMAND, GITHUB_URL } from '@/lib/constants';
import { Terminal, ArrowRight, Copy, CheckCircle2, Sparkles } from 'lucide-react';
import { useState } from 'react';

export function FinalCTA() {
  const [copied, setCopied] = useState(false);

  const copyToClipboard = async () => {
    await navigator.clipboard.writeText(INSTALL_COMMAND);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
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
          {/* Badge */}
          <motion.div variants={scaleIn}>
            <Badge className="bg-primary/10 text-primary border-primary/20 px-4 py-2">
              <Sparkles className="w-4 h-4 mr-2" />
              Ready to get started?
            </Badge>
          </motion.div>

          {/* Main Headline */}
          <motion.h2
            variants={fadeInUp}
            className="text-4xl md:text-6xl lg:text-7xl font-bold tracking-tight leading-tight"
          >
            Stop copy-pasting.
            <br />
            <span className="bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent">
              Start creating.
            </span>
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
                  {copied ? (
                    <CheckCircle2 className="w-4 h-4 text-green-500" />
                  ) : (
                    <Copy className="w-4 h-4" />
                  )}
                </Button>
              </div>

              {copied && (
                <motion.div
                  initial={{ opacity: 0, y: -5 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: -5 }}
                  className="text-sm text-green-500 flex items-center justify-center gap-2"
                >
                  <CheckCircle2 className="w-4 h-4" />
                  Command copied to clipboard!
                </motion.div>
              )}
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