import { GitHubRepository, GitHubContributor, GitHubStats } from './github-types';
import { GITHUB_URL } from './constants';

const GITHUB_API_BASE = 'https://api.github.com';
const CACHE_DURATION = 60 * 60 * 1000; // 1 hour in milliseconds

// Simple in-memory cache for server-side
let cachedData: { data: GitHubStats; timestamp: number } | null = null;

export class GitHubClient {
  private baseUrl: string;
  private headers: Record<string, string>;

  constructor() {
    this.baseUrl = GITHUB_API_BASE;
    this.headers = {
      'Accept': 'application/vnd.github.v3+json',
      'User-Agent': 'Devora-Website',
      ...(process.env.GITHUB_TOKEN && {
        'Authorization': `token ${process.env.GITHUB_TOKEN}`
      })
    };
  }

  private async fetchFromGitHub<T>(endpoint: string): Promise<T> {
    const response = await fetch(`${this.baseUrl}${endpoint}`, {
      headers: this.headers,
      next: { revalidate: CACHE_DURATION / 1000 } // Convert to seconds for Next.js
    });

    if (!response.ok) {
      throw new Error(`GitHub API error: ${response.status} ${response.statusText}`);
    }

    return response.json();
  }

  async getRepository(owner: string, repo: string): Promise<GitHubRepository> {
    return this.fetchFromGitHub<GitHubRepository>(`/repos/${owner}/${repo}`);
  }

  async getContributors(owner: string, repo: string, limit: number = 10): Promise<GitHubContributor[]> {
    const contributors = await this.fetchFromGitHub<GitHubContributor[]>(
      `/repos/${owner}/${repo}/contributors?per_page=${limit}`
    );

    // Enhance contributor data with user details if needed
    const enhancedContributors = await Promise.all(
      contributors.slice(0, limit).map(async (contributor) => {
        if (contributor.type === 'User') {
          try {
            const userDetail = await this.fetchFromGitHub<{
              name: string | null;
              login: string;
            }>(`/users/${contributor.login}`);

            return {
              ...contributor,
              name: userDetail.name || contributor.login
            };
          } catch (error) {
            // Fallback to login name if user detail fetch fails
            return {
              ...contributor,
              name: contributor.login
            };
          }
        }
        return contributor;
      })
    );

    return enhancedContributors;
  }

  async getRepositoryStats(owner: string, repo: string): Promise<GitHubStats> {
    // Check cache first
    if (cachedData && Date.now() - cachedData.timestamp < CACHE_DURATION) {
      return cachedData.data;
    }

    try {
      const [repository, contributors] = await Promise.all([
        this.getRepository(owner, repo),
        this.getContributors(owner, repo)
      ]);

      const stats: GitHubStats = {
        repository,
        contributors,
        total_contributors: contributors.length,
        last_updated: new Date().toISOString()
      };

      // Cache the result
      cachedData = {
        data: stats,
        timestamp: Date.now()
      };

      return stats;
    } catch (error) {
      console.error('Error fetching GitHub stats:', error);
      throw error;
    }
  }

  // Parse GitHub URL to extract owner and repo
  static parseGitHubUrl(url: string): { owner: string; repo: string } | null {
    try {
      const urlObj = new URL(url);
      const pathParts = urlObj.pathname.split('/').filter(Boolean);

      if (pathParts.length >= 2 && urlObj.hostname === 'github.com') {
        return {
          owner: pathParts[0],
          repo: pathParts[1]
        };
      }

      return null;
    } catch {
      return null;
    }
  }
}

// Singleton instance
export const githubClient = new GitHubClient();

// Convenience function to get stats for the main repository
export async function getDevoraGitHubStats(): Promise<GitHubStats> {
  const repoInfo = GitHubClient.parseGitHubUrl(GITHUB_URL);

  if (!repoInfo) {
    throw new Error('Invalid GitHub URL configuration');
  }

  return githubClient.getRepositoryStats(repoInfo.owner, repoInfo.repo);
}