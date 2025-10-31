'use client';

import { useState, useEffect, useRef } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { toast } from 'sonner';
import { DEMO_COMMANDS } from '@/lib/constants';
import {
  Terminal,
  Play,
  RotateCcw,
  Copy,
  Loader2
} from 'lucide-react';

export function LiveTerminal() {
  const [selectedCommand, setSelectedCommand] = useState(0);
  const [isRunning, setIsRunning] = useState(false);
  const [currentOutput, setCurrentOutput] = useState<string[]>([]);
  const [currentLine, setCurrentLine] = useState('');
  const outputRef = useRef<HTMLDivElement>(null);

  const command = DEMO_COMMANDS[selectedCommand];

  const runCommand = async () => {
    setIsRunning(true);
    setCurrentOutput([]);
    setCurrentLine('');

    // Add command to output
    setCurrentOutput([`$ ${command.command}`]);

    // Simulate typing and output
    let lineIndex = 0;
    const typeLine = async () => {
      if (lineIndex < command.output.length) {
        const line = command.output[lineIndex];
        let charIndex = 0;

        const typeChar = async () => {
          if (charIndex < line.length) {
            setCurrentLine(line.slice(0, charIndex + 1));
            charIndex++;
            setTimeout(typeChar, 30 + Math.random() * 50);
          } else {
            setCurrentOutput(prev => [...prev, line]);
            setCurrentLine('');
            lineIndex++;
            setTimeout(typeLine, 200 + Math.random() * 300);
          }
        };

        typeChar();
      } else {
        setIsRunning(false);
        setCurrentLine('');
      }
    };

    typeLine();
  };

  const resetTerminal = () => {
    setCurrentOutput([]);
    setCurrentLine('');
    setIsRunning(false);
  };

  const copyCommand = async () => {
    try {
      await navigator.clipboard.writeText(command.command);
      toast.success('Command copied to clipboard!');
    } catch (error) {
      toast.error('Failed to copy command to clipboard');
    }
  };

  // Auto-scroll to bottom
  useEffect(() => {
    if (outputRef.current) {
      outputRef.current.scrollTop = outputRef.current.scrollHeight;
    }
  }, [currentOutput, currentLine]);

  return (
    <section id="demo" className="py-20 px-4">
      <div className="container max-w-7xl mx-auto">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          className="text-center space-y-4 mb-16"
        >
          <h2 className="text-3xl md:text-5xl font-bold tracking-tight">
            Try It Yourself
          </h2>
          <p className="text-lg md:text-xl text-muted-foreground max-w-3xl mx-auto">
            See Devora in action. Run commands and watch as projects are scaffolded instantly.
          </p>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 40 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.8 }}
          className="max-w-4xl mx-auto"
        >
          {/* Command Selector */}
          <div className="mb-6 flex flex-wrap gap-3 justify-center">
            {DEMO_COMMANDS.map((cmd, index) => (
              <Button
                key={index}
                variant={selectedCommand === index ? "default" : "outline"}
                size="sm"
                onClick={() => {
                  setSelectedCommand(index);
                  resetTerminal();
                }}
                className="font-mono text-xs"
              >
                {cmd.command.split(' ')[0]} {cmd.command.split(' ')[1]}
              </Button>
            ))}
          </div>

          {/* Terminal */}
          <Card className="overflow-hidden border-border/50 bg-card/50 backdrop-blur-sm">
            {/* Terminal Header */}
            <div className="flex items-center justify-between p-4 border-b border-border/50 bg-muted/30">
              <div className="flex items-center gap-3">
                <div className="flex gap-1.5">
                  <div className="w-3 h-3 rounded-full bg-red-400/60" />
                  <div className="w-3 h-3 rounded-full bg-yellow-400/60" />
                  <div className="w-3 h-3 rounded-full bg-green-400/60" />
                </div>
                <Badge variant="secondary" className="text-xs">
                  <Terminal className="w-3 h-3 mr-1" />
                  devora-terminal
                </Badge>
              </div>

              <div className="flex items-center gap-2">
                <Button
                  size="sm"
                  variant="ghost"
                  onClick={copyCommand}
                  disabled={isRunning}
                  className="h-8 px-2"
                >
                  <Copy className="w-4 h-4" />
                </Button>

                <Button
                  size="sm"
                  variant="ghost"
                  onClick={resetTerminal}
                  disabled={isRunning}
                  className="h-8 px-2"
                >
                  <RotateCcw className="w-4 h-4" />
                </Button>

                <Button
                  size="sm"
                  onClick={runCommand}
                  disabled={isRunning}
                  className="h-8 px-3 bg-primary hover:bg-primary/90"
                >
                  {isRunning ? (
                    <Loader2 className="w-4 h-4 animate-spin" />
                  ) : (
                    <Play className="w-4 h-4" />
                  )}
                </Button>
              </div>
            </div>

            {/* Terminal Output */}
            <div
              ref={outputRef}
              className="p-4 font-mono text-sm bg-black/50 h-96 overflow-y-auto"
            >
              <AnimatePresence mode="popLayout">
                {currentOutput.map((line, index) => (
                  <motion.div
                    key={index}
                    initial={{ opacity: 0, x: -10 }}
                    animate={{ opacity: 1, x: 0 }}
                    className="mb-1"
                  >
                    <span className={
                      line.startsWith('$') ? 'text-primary' :
                      line.includes('✅') ? 'text-green-400' :
                      line.includes('🦀') || line.includes('🐍') ? 'text-yellow-400' :
                      line.includes('📁') || line.includes('📦') ? 'text-primary/70' :
                      'text-foreground/70'
                    }>
                      {line}
                    </span>
                  </motion.div>
                ))}

                {currentLine && (
                  <motion.div
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    className="flex items-center"
                  >
                    <span className="text-foreground/80">{currentLine}</span>
                    {isRunning && (
                      <motion.span
                        animate={{ opacity: [1, 0] }}
                        transition={{ duration: 0.5, repeat: Infinity }}
                        className="ml-1 w-2 h-4 bg-primary/80"
                      />
                    )}
                  </motion.div>
                )}
              </AnimatePresence>

              {/* Empty state */}
              {currentOutput.length === 0 && !currentLine && (
                <div className="text-muted-foreground text-center mt-32">
                  <Terminal className="w-12 h-12 mx-auto mb-4 opacity-50" />
                  <p>Click the play button to run a command</p>
                </div>
              )}
            </div>
          </Card>

          {/* Command Description */}
          <motion.div
            initial={{ opacity: 0 }}
            whileInView={{ opacity: 1 }}
            viewport={{ once: true }}
            transition={{ delay: 0.3 }}
            className="mt-6 text-center"
          >
            <p className="text-muted-foreground">
              {command.description}
            </p>
          </motion.div>
        </motion.div>
      </div>
    </section>
  );
}