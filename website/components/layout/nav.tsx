'use client';

import { Suspense } from 'react';
import dynamic from 'next/dynamic';

const MotionNav = dynamic(
  () => import('./motion-nav').then(mod => ({ default: mod.MotionNav })),
  {
    ssr: false,
    loading: () => <NavSkeleton />,
  }
);

function NavSkeleton() {
  return (
    <nav className="fixed inset-x-0 top-0 z-50 h-14 border-b border-transparent">
      <div className="container-tight flex h-full items-center justify-between">
        <div className="flex items-center gap-2">
          <span className="font-mono text-sm font-medium tracking-tight">devora</span>
          <span className="font-mono text-[10px] text-muted-foreground/70 border border-border rounded px-1.5 py-px">
            v0.1
          </span>
        </div>
        <div className="size-9" />
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
