'use client';

import { useEffect, useRef, useState, useSyncExternalStore } from 'react';
import { AnimatePresence, motion, useReducedMotion } from 'framer-motion';
import { toast } from 'sonner';
import { Copy } from 'lucide-react';
import { INSTALL_TARGETS, InstallTargetId } from '@/lib/constants';
import { EASE_OUT } from '@/lib/animations';
import { cn } from '@/lib/utils';

// CSS cubic-bezier mirror of EASE_OUT, for the toggle's transform transition
// (kept on the compositor rather than the main thread).
const CSS_EASE = `cubic-bezier(${EASE_OUT.join(',')})`;

function detectTarget(): InstallTargetId {
  if (typeof navigator === 'undefined') return 'unix';
  const ua = navigator.userAgent || '';
  // Treat only Windows specially; macOS and Linux both use the curl installer.
  return /windows|win32|win64/i.test(ua) ? 'windows' : 'unix';
}

// The detected OS is a client-only value that never changes after load.
// useSyncExternalStore reads it without a setState-in-effect and without a
// hydration mismatch (server snapshot is always 'unix').
const subscribeOS = () => () => {};
const serverOS = (): InstallTargetId => 'unix';

/** Checkmark that draws itself in on mount (or appears instantly if the user
 *  prefers reduced motion). */
function CheckDraw({ instant }: { instant: boolean }) {
  return (
    <svg width="14" height="14" viewBox="0 0 16 16" fill="none" aria-hidden>
      <motion.path
        d="M3.5 8.4 L6.7 11.6 L12.6 4.7"
        stroke="currentColor"
        strokeWidth="1.85"
        strokeLinecap="round"
        strokeLinejoin="round"
        initial={{ pathLength: instant ? 1 : 0 }}
        animate={{ pathLength: 1 }}
        transition={{ duration: instant ? 0 : 0.22, ease: EASE_OUT }}
      />
    </svg>
  );
}

export function InstallCommand({ className }: { className?: string }) {
  const reduceMotion = useReducedMotion();
  const detected = useSyncExternalStore(subscribeOS, detectTarget, serverOS);
  // A manual toggle choice overrides detection; null means "follow the OS".
  const [override, setOverride] = useState<InstallTargetId | null>(null);
  const [copied, setCopied] = useState(false);
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const target = override ?? detected;

  useEffect(
    () => () => {
      if (timer.current) clearTimeout(timer.current);
    },
    []
  );

  const active = INSTALL_TARGETS.find((t) => t.id === target) ?? INSTALL_TARGETS[0];

  const copy = async () => {
    try {
      await navigator.clipboard.writeText(active.command);
      toast.success('Copied install command');
      setCopied(true);
      if (timer.current) clearTimeout(timer.current);
      timer.current = setTimeout(() => setCopied(false), 2000);
    } catch {
      toast.error('Could not copy. Long-press to select.');
    }
  };

  const activeIndex = INSTALL_TARGETS.findIndex((t) => t.id === target);

  return (
    <div className={cn('w-full max-w-md', className)}>
      {/* OS toggle */}
      <div
        role="tablist"
        aria-label="Operating system"
        className="relative mb-2.5 flex w-[244px] overflow-hidden rounded-md bg-secondary/40 ring-1 ring-border"
      >
        <span
          aria-hidden
          className="absolute inset-y-0 left-0 w-1/2 bg-foreground/[0.07]"
          style={{
            transform: `translateX(${activeIndex * 100}%)`,
            transition: `transform 220ms ${CSS_EASE}`,
          }}
        />
        {INSTALL_TARGETS.map((t) => (
          <button
            key={t.id}
            role="tab"
            aria-selected={t.id === target}
            onClick={() => setOverride(t.id)}
            className={cn(
              'relative z-10 flex-1 py-1.5 text-center font-mono text-[10px] uppercase tracking-[0.14em] transition-colors duration-150',
              t.id === target ? 'text-foreground' : 'text-muted-foreground hover:text-foreground/80'
            )}
          >
            {t.label}
          </button>
        ))}
      </div>

      {/* Command + copy */}
      <button
        onClick={copy}
        aria-label={`Copy the ${active.label} install command`}
        className="group/copy flex w-full items-center justify-between gap-3 rounded-md bg-secondary/60 px-3.5 py-3 text-left ring-1 ring-border transition-[box-shadow,transform] duration-200 hover:ring-foreground/30 active:scale-[0.99]"
      >
        <span className="flex items-center gap-2.5 overflow-hidden">
          <span className="select-none font-mono text-xs text-muted-foreground/60">
            {active.prompt}
          </span>
          <code className="truncate font-mono text-[13px] text-foreground/90">
            {active.command}
          </code>
        </span>

        <span className="flex shrink-0 items-center justify-end" style={{ minWidth: 76 }}>
          <AnimatePresence mode="wait" initial={false}>
            {copied ? (
              <motion.span
                key="done"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                transition={{ duration: 0.12 }}
                className="flex items-center gap-1.5 text-foreground"
              >
                <CheckDraw instant={!!reduceMotion} />
                <span className="font-mono text-[11px] uppercase tracking-wider">copied</span>
              </motion.span>
            ) : (
              <motion.span
                key="copy"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                transition={{ duration: 0.12 }}
                className="flex items-center gap-1.5 text-muted-foreground transition-colors group-hover/copy:text-foreground"
              >
                <Copy className="size-3.5" />
                <span className="font-mono text-[11px] uppercase tracking-wider">copy</span>
              </motion.span>
            )}
          </AnimatePresence>
        </span>
      </button>
    </div>
  );
}
