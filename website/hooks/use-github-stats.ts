'use client';

import { useState, useEffect } from 'react';
import { GitHubStats, GitHubStatsResponse } from '@/lib/github-types';

interface UseGitHubStatsResult {
  data: GitHubStats | null;
  loading: boolean;
  error: string | null;
  refetch: () => void;
  isCached: boolean;
  lastCached: string | null;
}

export function useGitHubStats(): UseGitHubStatsResult {
  const [data, setData] = useState<GitHubStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isCached, setIsCached] = useState(false);
  const [lastCached, setLastCached] = useState<string | null>(null);

  const fetchStats = async () => {
    try {
      setLoading(true);
      setError(null);

      const response = await fetch('/api/github/stats', {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const result: GitHubStatsResponse = await response.json();

      if (result.error && result.data) {
        // We have fallback data but there was an error
        setError(result.error);
        setData(result.data);
      } else if (result.data) {
        setData(result.data);
        setError(null);
      }

      setIsCached(result.cached);
      setLastCached(result.last_cached);

    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to fetch GitHub stats';
      setError(errorMessage);
      console.error('Error fetching GitHub stats:', err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchStats();
  }, []);

  return {
    data,
    loading,
    error,
    refetch: fetchStats,
    isCached,
    lastCached
  };
}