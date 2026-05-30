import { NextResponse } from 'next/server';
import { getDevoraGitHubStats } from '@/lib/github-client';
import { GitHubStatsResponse, GitHubStats } from '@/lib/github-types';

// Fallback data for when API fails
// Conservative fallback used only when the GitHub API is unavailable.
// No invented stats or contributors — just the real repo identity.
const fallbackData: GitHubStats = {
  repository: {
    id: 0,
    name: 'Devora',
    full_name: 'Nathandona/Devora',
    description: 'A universal, modular project scaffolding framework',
    stargazers_count: 0,
    forks_count: 0,
    language: 'Rust',
    updated_at: new Date().toISOString(),
    created_at: new Date().toISOString(),
    default_branch: 'main'
  },
  contributors: [
    {
      id: 0,
      login: 'Nathandona',
      name: 'Nathan Donadey',
      avatar_url: 'https://github.com/Nathandona.png',
      html_url: 'https://github.com/Nathandona',
      contributions: 1,
      type: 'User'
    }
  ],
  total_contributors: 1,
  last_updated: new Date().toISOString()
};

export async function GET(): Promise<NextResponse<GitHubStatsResponse>> {
  try {
    const stats = await getDevoraGitHubStats();

    const response: GitHubStatsResponse = {
      data: stats,
      error: null,
      cached: true, // Since we're using caching
      last_cached: new Date().toISOString()
    };

    // Add cache headers
    return NextResponse.json(response, {
      headers: {
        'Cache-Control': 'public, s-maxage=3600, stale-while-revalidate=7200',
        'Access-Control-Allow-Origin': '*',
        'Access-Control-Allow-Methods': 'GET',
      }
    });

  } catch (error) {
    console.error('GitHub API error:', error);

    const errorResponse: GitHubStatsResponse = {
      data: fallbackData,
      error: error instanceof Error ? error.message : 'Unknown error occurred',
      cached: false,
      last_cached: null
    };

    return NextResponse.json(errorResponse, {
      status: 500,
      headers: {
        'Cache-Control': 'public, s-maxage=300', // Cache error responses for 5 minutes
        'Access-Control-Allow-Origin': '*',
        'Access-Control-Allow-Methods': 'GET',
      }
    });
  }
}

// Handle preflight requests
export async function OPTIONS(): Promise<NextResponse> {
  return new NextResponse(null, {
    status: 200,
    headers: {
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Methods': 'GET',
      'Access-Control-Allow-Headers': 'Content-Type',
    }
  });
}