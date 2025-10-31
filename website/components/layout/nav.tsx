'use client';

import { Suspense } from 'react';
import dynamic from 'next/dynamic';

const MotionNav = dynamic(() => import('./motion-nav').then(mod => ({ default: mod.MotionNav })), {
  ssr: false,
  loading: () => <NavSkeleton />
});

function NavSkeleton() {
  return (
    <nav className="fixed top-0 left-0 right-0 z-50 h-16 bg-background/80 backdrop-blur-md border-b border-border/50">
      <div className="container max-w-7xl mx-auto px-4 h-full flex items-center justify-between">
        <div className="w-24 h-6 bg-muted rounded animate-pulse" />
        <div className="w-32 h-8 bg-muted rounded animate-pulse" />
      </div>
    </nav>
  );
}

export function Nav() {
  return (
    <Suspense fallback={<NavSkeleton />}>
      <MotionNav />
    </Suspense>
  );
}