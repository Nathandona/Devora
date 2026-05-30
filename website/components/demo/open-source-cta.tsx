'use client';

import { motion } from 'framer-motion';
import { ArrowUpRight, Github } from 'lucide-react';
import { useGitHubStats } from '@/hooks/use-github-stats';
import { fadeRise, stagger } from '@/lib/animations';
import { GITHUB_URL, CONTRIBUTING_URL } from '@/lib/constants';

function Stat({
  label,
  value,
  loading,
}: {
  label: string;
  value: number | string | null | undefined;
  loading: boolean;
}) {
  return (
    <div className="flex flex-col gap-1">
      <span className="font-mono text-[10px] uppercase tracking-[0.18em] text-muted-foreground/70">
        {label}
      </span>
      <span className="font-medium tabular-nums text-foreground tracking-tight" style={{ fontSize: 'clamp(1.6rem, 3vw, 2rem)' }}>
        {loading || value == null ? <span className="text-muted-foreground/40">-</span> : value}
      </span>
    </div>
  );
}

export function OpenSourceCTA() {
  const { data, loading } = useGitHubStats();

  return (
    <section id="open-source" className="relative py-32 border-t border-border">
      <div className="container-tight">
        <motion.div
          initial="initial"
          whileInView="animate"
          viewport={{ once: true, amount: 0.4 }}
          variants={stagger}
          className="max-w-2xl"
        >
          <motion.h2
            variants={fadeRise}
            className="font-medium tracking-tight"
            style={{
              fontSize: 'clamp(1.8rem, 3.4vw, 2.6rem)',
              lineHeight: 1.1,
              letterSpacing: '-0.02em',
            }}
          >
            Built in the open.
            <br />
            <span className="text-muted-foreground">By whoever shows up.</span>
          </motion.h2>
          <motion.p
            variants={fadeRise}
            className="mt-5 text-[15px] leading-relaxed text-muted-foreground max-w-md"
          >
            Every commit is public. Every plugin lives outside the binary.
            Pull requests, issues, plugins &mdash; all welcome.
          </motion.p>
        </motion.div>

        <motion.div
          initial="initial"
          whileInView="animate"
          viewport={{ once: true, amount: 0.2 }}
          variants={stagger}
          className="mt-14"
        >
          <div className="grid grid-cols-2 sm:grid-cols-4 gap-x-10 gap-y-8 max-w-3xl">
            <motion.div variants={fadeRise}>
              <Stat label="Stars" value={data?.repository.stargazers_count} loading={loading} />
            </motion.div>
            <motion.div variants={fadeRise}>
              <Stat label="Forks" value={data?.repository.forks_count} loading={loading} />
            </motion.div>
            <motion.div variants={fadeRise}>
              <Stat label="Contributors" value={data?.total_contributors} loading={loading} />
            </motion.div>
            <motion.div variants={fadeRise}>
              <Stat label="License" value="MIT" loading={false} />
            </motion.div>
          </div>
        </motion.div>

        {/* Contributors row */}
        <motion.div
          initial={{ opacity: 0 }}
          whileInView={{ opacity: 1 }}
          viewport={{ once: true, amount: 0.4 }}
          transition={{ duration: 0.5, delay: 0.1 }}
          className="mt-16 max-w-3xl"
        >
          <div className="flex items-baseline justify-between gap-4 mb-5">
            <h3 className="text-xs uppercase tracking-[0.18em] text-muted-foreground/70 font-medium">
              Recent contributors
            </h3>
            <a
              href={`${GITHUB_URL}/graphs/contributors`}
              target="_blank"
              rel="noreferrer"
              className="inline-flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground transition-colors"
            >
              View all
              <ArrowUpRight className="size-3" />
            </a>
          </div>

          <div className="flex flex-wrap gap-2.5">
            {loading
              ? Array.from({ length: 8 }).map((_, i) => (
                  <span
                    key={i}
                    className="size-9 rounded-full bg-muted animate-pulse"
                  />
                ))
              : data?.contributors.slice(0, 14).map((c) => (
                  <a
                    key={c.id}
                    href={c.html_url}
                    target="_blank"
                    rel="noreferrer"
                    title={`${c.name || c.login} · ${c.contributions} commits`}
                    className="group/avatar relative inline-block"
                  >
                    {/* eslint-disable-next-line @next/next/no-img-element */}
                    <img
                      src={c.avatar_url}
                      alt={c.login}
                      width={36}
                      height={36}
                      loading="lazy"
                      className="size-9 rounded-full bg-muted ring-1 ring-border grayscale group-hover/avatar:grayscale-0 group-hover/avatar:ring-foreground/40 transition-[filter,box-shadow] duration-200"
                    />
                  </a>
                ))}
          </div>
        </motion.div>

        {/* CTA row */}
        <motion.div
          initial={{ opacity: 0, y: 8 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true, amount: 0.6 }}
          transition={{ duration: 0.5 }}
          className="mt-16 flex flex-wrap items-center gap-x-6 gap-y-3 text-sm"
        >
          <a
            href={GITHUB_URL}
            target="_blank"
            rel="noreferrer"
            className="inline-flex items-center gap-2 text-muted-foreground hover:text-foreground transition-colors"
          >
            <Github className="size-4" />
            View repository
          </a>
          <span className="text-muted-foreground/30">·</span>
          <a
            href={CONTRIBUTING_URL}
            target="_blank"
            rel="noreferrer"
            className="inline-flex items-center gap-1.5 text-muted-foreground hover:text-foreground transition-colors"
          >
            Read the contributing guide
            <ArrowUpRight className="size-3.5" />
          </a>
        </motion.div>
      </div>
    </section>
  );
}
