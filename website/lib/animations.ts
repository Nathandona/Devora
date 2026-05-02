import { Transition, Variants } from 'framer-motion';

// Single ease used across the site. Smooth out, no bounce.
export const EASE_OUT: [number, number, number, number] = [0.32, 0.72, 0, 1];

// Spring used for any "land in place" motion (file tree lines, cards, hovers).
export const SPRING: Transition = {
  type: 'spring',
  stiffness: 380,
  damping: 32,
  mass: 0.8,
};

// Soft micro-spring for hovers — almost imperceptible bounce.
export const HOVER_SPRING: Transition = {
  type: 'spring',
  stiffness: 500,
  damping: 38,
  mass: 0.6,
};

// Primary entry motion. Use sparingly.
export const fadeRise: Variants = {
  initial: { opacity: 0, y: 8 },
  animate: {
    opacity: 1,
    y: 0,
    transition: { duration: 0.5, ease: EASE_OUT },
  },
};

// Pure opacity. For text blocks where movement would be noise.
export const fadeIn: Variants = {
  initial: { opacity: 0 },
  animate: {
    opacity: 1,
    transition: { duration: 0.4, ease: EASE_OUT },
  },
};

// Tight stagger for siblings entering together.
export const stagger: Variants = {
  animate: {
    transition: { staggerChildren: 0.04, delayChildren: 0.04 },
  },
};

// Terminal cursor blink — 530ms feels alive (not the textbook 1s).
export const cursorBlink: Variants = {
  animate: {
    opacity: [1, 1, 0, 0],
    transition: { duration: 1.06, repeat: Infinity, times: [0, 0.5, 0.5, 1] },
  },
};
