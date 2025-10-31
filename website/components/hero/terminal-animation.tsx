'use client';

import { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { fileTreeExpand, terminalCursor } from '@/lib/animations';
import { CheckCircle2, Loader2, FolderTree, FileCode } from 'lucide-react';

interface TerminalStep {
  type: 'command' | 'output' | 'file' | 'success';
  content: string;
  delay: number;
}

const TERMINAL_SEQUENCES: TerminalStep[][] = [
  [
    { type: 'command', content: '$ devora new rust my-project', delay: 0 },
    { type: 'output', content: '🦀 Creating Rust project "my-project"...', delay: 800 },
    { type: 'output', content: '📁 Generated project structure:', delay: 1600 },
    { type: 'file', content: '├── Cargo.toml', delay: 2400 },
    { type: 'file', content: '├── README.md', delay: 2800 },
    { type: 'file', content: '├── .gitignore', delay: 3200 },
    { type: 'file', content: '├── src/', delay: 3600 },
    { type: 'file', content: '│   ├── main.rs', delay: 4000 },
    { type: 'file', content: '│   └── tests.rs', delay: 4400 },
    { type: 'success', content: '✅ Project created successfully!', delay: 5200 },
  ],
  [
    { type: 'command', content: '$ devora new api python --framework=fastapi', delay: 0 },
    { type: 'output', content: '🐍 Creating Python API project "api"...', delay: 800 },
    { type: 'output', content: '📦 Installing FastAPI dependencies...', delay: 1600 },
    { type: 'file', content: '├── requirements.txt', delay: 2400 },
    { type: 'file', content: '├── main.py', delay: 2800 },
    { type: 'file', content: '├── Dockerfile', delay: 3200 },
    { type: 'success', content: '✅ Python API project ready!', delay: 4000 },
  ],
];

export function TerminalAnimation() {
  const [currentSequence, setCurrentSequence] = useState(0);
  const [currentStep, setCurrentStep] = useState(0);
  const [visibleSteps, setVisibleSteps] = useState<TerminalStep[]>([]);
  const [isTyping, setIsTyping] = useState(false);

  const sequence = TERMINAL_SEQUENCES[currentSequence];

  useEffect(() => {
    const timer = setInterval(() => {
      if (currentStep < sequence.length) {
        const step = sequence[currentStep];
        setVisibleSteps(prev => [...prev, step]);
        setCurrentStep(prev => prev + 1);
        setIsTyping(true);
        setTimeout(() => setIsTyping(false), 300);
      } else {
        // Reset after completing sequence
        setTimeout(() => {
          setCurrentStep(0);
          setVisibleSteps([]);
          setCurrentSequence(prev => (prev + 1) % TERMINAL_SEQUENCES.length);
        }, 3000);
      }
    }, sequence[currentStep]?.delay || 100);

    return () => clearInterval(timer);
  }, [currentStep, sequence, currentSequence]);

  return (
    <Card className="relative overflow-hidden border-border/50 bg-card/50 backdrop-blur-sm">
      <div className="flex items-center gap-2 border-b border-border/50 p-3 bg-muted/30">
        <div className="flex gap-1.5">
          <div className="w-3 h-3 rounded-full bg-red-500/80" />
          <div className="w-3 h-3 rounded-full bg-yellow-500/80" />
          <div className="w-3 h-3 rounded-full bg-green-500/80" />
        </div>
        <div className="flex-1 flex justify-center">
          <Badge variant="secondary" className="text-xs">
            terminal
          </Badge>
        </div>
        <div className="w-16" />
      </div>

      <div className="p-4 font-mono text-sm space-y-1 min-h-[300px]">
        <AnimatePresence mode="popLayout">
          {visibleSteps.map((step, index) => (
            <motion.div
              key={`${currentSequence}-${index}`}
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -10 }}
              transition={{ duration: 0.3 }}
              className="flex items-start gap-2"
            >
              {step.type === 'command' && (
                <>
                  <span className="text-cyan-400">$</span>
                  <span className="text-foreground">{step.content}</span>
                </>
              )}
              {step.type === 'output' && (
                <>
                  <span className="text-green-400">→</span>
                  <span className="text-muted-foreground">{step.content}</span>
                </>
              )}
              {step.type === 'file' && (
                <>
                  <FolderTree className="w-4 h-4 text-blue-400 mt-0.5" />
                  <span className="text-foreground/80">{step.content}</span>
                </>
              )}
              {step.type === 'success' && (
                <>
                  <CheckCircle2 className="w-4 h-4 text-green-500 mt-0.5" />
                  <span className="text-green-500 font-medium">{step.content}</span>
                </>
              )}
            </motion.div>
          ))}
        </AnimatePresence>

        {isTyping && (
          <motion.div
            variants={terminalCursor}
            initial="initial"
            animate="animate"
            className="inline-block w-2 h-4 bg-cyan-400 ml-2"
          />
        )}
      </div>

      <div className="absolute bottom-3 right-3">
        <div className="flex items-center gap-2 text-xs text-muted-foreground">
          {currentStep < sequence.length && (
            <Loader2 className="w-3 h-3 animate-spin" />
          )}
          <span>{currentSequence + 1}/{TERMINAL_SEQUENCES.length}</span>
        </div>
      </div>
    </Card>
  );
}