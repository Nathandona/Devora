'use client';

import { useState, useEffect } from 'react';
import { DEVORA_COMMANDS } from '@/lib/constants';

export function Typewriter() {
  const [currentIndex, setCurrentIndex] = useState(0);
  const [displayText, setDisplayText] = useState('');
  const [isTyping, setIsTyping] = useState(true);
  const [showCursor, setShowCursor] = useState(true);

  const currentCommand = DEVORA_COMMANDS[currentIndex];

  useEffect(() => {
    if (isTyping) {
      const timeout = setTimeout(() => {
        if (displayText.length < currentCommand.length) {
          setDisplayText(currentCommand.slice(0, displayText.length + 1));
        } else {
          setIsTyping(false);
          // Wait 2 seconds before starting to delete
          setTimeout(() => {
            setIsTyping(true);
            setDisplayText('');
            setCurrentIndex((prev) => (prev + 1) % DEVORA_COMMANDS.length);
          }, 2000);
        }
      }, 50 + Math.random() * 50); // Variable typing speed

      return () => clearTimeout(timeout);
    }
  }, [displayText, isTyping, currentCommand]);

  useEffect(() => {
    const cursorInterval = setInterval(() => {
      setShowCursor((prev) => !prev);
    }, 500);

    return () => clearInterval(cursorInterval);
  }, []);

  return (
    <div className="font-mono text-lg md:text-xl">
      <span className="text-cyan-400">$ </span>
      <span className="text-foreground">{displayText}</span>
      {showCursor && (
        <span
          className="inline-block w-0.5 h-5 bg-cyan-400 ml-0.5 animate-pulse"
        />
      )}
    </div>
  );
}