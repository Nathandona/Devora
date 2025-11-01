// GitHub API types and interfaces
export interface GitHubRepository {
  id: number;
  name: string;
  full_name: string;
  description: string | null;
  stargazers_count: number;
  forks_count: number;
  language: string | null;
  updated_at: string;
  created_at: string;
  default_branch: string;
}

export interface GitHubContributor {
  id: number;
  login: string;
  name: string | null;
  avatar_url: string;
  html_url: string;
  contributions: number;
  type: string;
}

export interface GitHubStats {
  repository: GitHubRepository;
  contributors: GitHubContributor[];
  total_contributors: number;
  last_updated: string;
}

// API response types
export interface GitHubStatsResponse {
  data: GitHubStats | null;
  error: string | null;
  cached: boolean;
  last_cached: string | null;
}