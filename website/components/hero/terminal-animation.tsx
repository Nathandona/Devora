'use client';

import { useEffect, useMemo, useRef, useState } from 'react';
import { motion, useReducedMotion } from 'framer-motion';
import { TERMINAL_SEQUENCES, TerminalLine } from '@/lib/constants';
import { EASE_OUT } from '@/lib/animations';

const PROMPT = '~/projects';
const PAUSE_BETWEEN_SEQUENCES_MS = 1800;
const PAUSE_BEFORE_NEXT_LINE_MS = 220;
const TYPE_MIN = 28;
const TYPE_MAX = 58;
const TYPE_JITTER = 14;

function classFor(kind: TerminalLine['kind']) {
  switch (kind) {
    case 'command':  return 'text-foreground';
    case 'output':   return 'text-foreground/85';
    case 'tree':     return 'text-foreground/70';
    case 'success':  return 'text-foreground';
    case 'muted':    return 'text-muted-foreground';
    case 'prompt':   return 'text-muted-foreground';
  }
}

function prefixFor(kind: TerminalLine['kind']) {
  if (kind === 'command') return <span className="text-muted-foreground select-none">{PROMPT} ❯ </span>;
  if (kind === 'success') return <span className="text-foreground/60 select-none">  ✓ </span>;
  if (kind === 'output')  return <span className="text-foreground/40 select-none">  · </span>;
  if (kind === 'muted')   return <span className="text-foreground/30 select-none">  · </span>;
  return null;
}

function StaticFrame({ sequence }: { sequence: typeof TERMINAL_SEQUENCES[number] }) {
  return (
    <>
      {sequence.lines.map((line, i) => (
        <div key={i} className={`whitespace-pre ${classFor(line.kind)}`}>
          {prefixFor(line.kind)}
          {line.text}
        </div>
      ))}
    </>
  );
}

export function TerminalAnimation() {
  const prefersReducedMotion = useReducedMotion();
  const [seqIndex, setSeqIndex] = useState(0);
  const [doneCount, setDoneCount] = useState(0);
  const [typingChar, setTypingChar] = useState(0);
  const [hovered, setHovered] = useState(false);
  const [restartTick, setRestartTick] = useState(0);
  const containerRef = useRef<HTMLDivElement>(null);

  const sequence = TERMINAL_SEQUENCES[seqIndex];
  const renderedLines = useMemo(
    () => sequence.lines.slice(0, doneCount),
    [sequence, doneCount]
  );
  const currentLine: TerminalLine | undefined = sequence.lines[doneCount];
  const isBetween = !currentLine; // sequence finished, waiting for next
  const isCommandTyping = currentLine?.kind === 'command';

  // Drive the animation forward. The effect only schedules timeouts; it never
  // setStates synchronously, so each render reads stable derived values.
  useEffect(() => {
    if (prefersReducedMotion || hovered) return;

    if (isBetween) {
      const t = setTimeout(() => {
        setSeqIndex(i => (i + 1) % TERMINAL_SEQUENCES.length);
        setDoneCount(0);
        setTypingChar(0);
      }, PAUSE_BETWEEN_SEQUENCES_MS);
      return () => clearTimeout(t);
    }

    if (isCommandTyping) {
      if (typingChar < currentLine!.text.length) {
        const variance = TYPE_MIN + Math.random() * (TYPE_MAX - TYPE_MIN);
        const jitter = (Math.random() - 0.5) * TYPE_JITTER;
        const t = setTimeout(() => setTypingChar(c => c + 1), variance + jitter);
        return () => clearTimeout(t);
      }
      const t = setTimeout(() => {
        setDoneCount(c => c + 1);
        setTypingChar(0);
      }, 240);
      return () => clearTimeout(t);
    }

    const delay = currentLine!.kind === 'tree' ? 60 : PAUSE_BEFORE_NEXT_LINE_MS;
    const t = setTimeout(() => setDoneCount(c => c + 1), delay);
    return () => clearTimeout(t);
  }, [
    prefersReducedMotion,
    hovered,
    isBetween,
    isCommandTyping,
    typingChar,
    currentLine,
    restartTick,
  ]);

  const handleClick = () => {
    setDoneCount(0);
    setTypingChar(0);
    setRestartTick(t => t + 1);
  };

  if (prefersReducedMotion) {
    return (
      <div
        className="relative w-full overflow-hidden rounded-lg bg-terminal ring-hairline"
        style={{ fontFeatureSettings: '"liga" 0, "calt" 0' }}
      >
        <TerminalChrome hovered={false} />
        <div className="px-4 py-4 font-mono text-[13px] leading-[1.7] min-h-[340px]">
          <StaticFrame sequence={sequence} />
        </div>
      </div>
    );
  }

  const visibleCommandSlice = isCommandTyping ? currentLine!.text.slice(0, typingChar) : '';

  return (
    <div
      ref={containerRef}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onClick={handleClick}
      role="img"
      aria-label="Animated demonstration of the Devora CLI"
      className="group relative w-full overflow-hidden rounded-lg bg-terminal ring-hairline cursor-pointer select-none"
      style={{ fontFeatureSettings: '"liga" 0, "calt" 0' }}
    >
      <TerminalChrome hovered={hovered} />

      <div className="px-4 py-4 font-mono text-[13px] leading-[1.7] min-h-[340px]">
        {renderedLines.map((line, i) => (
          <motion.div
            key={`${seqIndex}-${i}-${restartTick}`}
            initial={{ opacity: 0, y: 4 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.28, ease: EASE_OUT }}
            className={`whitespace-pre ${classFor(line.kind)}`}
          >
            {prefixFor(line.kind)}
            {line.text}
          </motion.div>
        ))}

        {isCommandTyping && (
          <div className="whitespace-pre text-foreground">
            {prefixFor('command')}
            {visibleCommandSlice}
            <Cursor blinking={typingChar === 0 || typingChar === currentLine!.text.length} />
          </div>
        )}

        {isBetween && (
          <div className="whitespace-pre text-foreground">
            {prefixFor('command')}
            <Cursor blinking />
          </div>
        )}
      </div>

      <div className="pointer-events-none absolute inset-x-0 top-0 h-12 bg-gradient-to-b from-background/40 to-transparent" />
      <div className="pointer-events-none absolute inset-x-0 bottom-0 h-12 bg-gradient-to-t from-background/40 to-transparent" />
    </div>
  );
}

function TerminalChrome({ hovered }: { hovered: boolean }) {
  return (
    <div className="flex items-center justify-between border-b border-border/60 px-4 py-2.5">
      <div className="flex items-center gap-1.5">
        <span className="size-2 rounded-full bg-foreground/15" />
        <span className="size-2 rounded-full bg-foreground/15" />
        <span className="size-2 rounded-full bg-foreground/15" />
      </div>
      <span className="font-mono text-[10px] uppercase tracking-[0.18em] text-muted-foreground/60">
        devora · zsh
      </span>
      <span className="font-mono text-[10px] text-muted-foreground/40">
        {hovered ? 'paused' : ''}
      </span>
    </div>
  );
}

function Cursor({ blinking }: { blinking: boolean }) {
  return (
    <motion.span
      aria-hidden
      className="ml-[1px] inline-block h-[1em] w-[0.55ch] -mb-[2px] align-baseline bg-foreground/85"
      animate={blinking ? { opacity: [1, 1, 0, 0] } : { opacity: 1 }}
      transition={blinking ? { duration: 1.06, repeat: Infinity, times: [0, 0.5, 0.5, 1] } : undefined}
    />
  );
}
