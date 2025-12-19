/**
 * Git Activity Dashboard - TypeScript Definitions
 *
 * Analyze git contributions across repositories with daily/weekly views,
 * contribution type breakdown, and multiple export formats.
 */

export interface FileClassification {
  file_path: string;
  contribution_type: ContributionType;
  language: string | null;
  lines_added: number;
  lines_removed: number;
}

export type ContributionType =
  | "production_code"
  | "tests"
  | "documentation"
  | "specs_config"
  | "infrastructure"
  | "styling"
  | "other";

export interface CommitInfo {
  hash: string;
  author: string;
  email: string;
  date: string; // ISO 8601
  message: string;
  files_changed: number;
  lines_added: number;
  lines_removed: number;
  file_classifications?: FileClassification[];
}

export interface RepoStats {
  name: string;
  path: string;
  description: string;
  technologies: string[];
  total_commits: number;
  total_lines_added: number;
  total_lines_removed: number;
  total_files_changed: number;
  first_commit_date: string | null;
  last_commit_date: string | null;
  languages: Record<string, number>;
  contribution_types: Record<ContributionType, number>;
  commits?: CommitInfo[];
}

export interface ActivitySummary {
  period_start: string;
  period_end: string;
  period_label: string;
  commits: number;
  lines_added: number;
  lines_removed: number;
  files_changed: number;
  repos_active: number;
  contribution_breakdown: Record<ContributionType, number>;
  language_breakdown: Record<string, number>;
}

export interface TotalStats {
  total_repos: number;
  total_commits: number;
  total_lines_added: number;
  total_lines_removed: number;
  total_lines_changed: number;
  total_files_changed: number;
  languages: Record<string, number>;
  contribution_types: Record<ContributionType, number>;
  contribution_percentages: Record<ContributionType, number>;
}

export interface DashboardData {
  generated_at: string;
  summary: TotalStats;
  repositories: RepoStats[];
  daily_activity: ActivitySummary[];
  weekly_activity: ActivitySummary[];
}

/**
 * Main analyzer class for git activity.
 *
 * @example
 * ```typescript
 * import { WasmAnalyzer } from 'git-activity-dashboard';
 * import { execSync } from 'child_process';
 *
 * const analyzer = new WasmAnalyzer('your@email.com', null);
 *
 * // Get git log from a repo
 * const gitLog = execSync(
 *   "git log --format='%H|%an|%ae|%aI|%s' --numstat",
 *   { cwd: '/path/to/repo', encoding: 'utf-8' }
 * );
 *
 * // Parse the log
 * analyzer.parseGitLog('repo-name', '/path/to/repo', gitLog);
 *
 * // Get stats
 * const stats = analyzer.getTotalStats();
 * console.log(stats);
 *
 * // Export
 * const markdown = analyzer.exportMarkdown();
 * const linkedin = analyzer.exportLinkedIn();
 * const portfolio = analyzer.exportPortfolio();
 * ```
 */
export class WasmAnalyzer {
  /**
   * Create a new analyzer instance.
   * @param author_email - Filter commits by this email (optional)
   * @param author_name - Filter commits by this name (optional)
   */
  constructor(author_email?: string | null, author_name?: string | null);

  /**
   * Parse git log output from a repository.
   *
   * The git log should be formatted using:
   * `git log --format='%H|%an|%ae|%aI|%s' --numstat`
   *
   * @param repo_name - Name of the repository
   * @param repo_path - Path to the repository
   * @param log_output - Raw git log output
   * @returns Parsed repository statistics
   */
  parseGitLog(repo_name: string, repo_path: string, log_output: string): RepoStats;

  /**
   * Add pre-parsed repository data.
   * @param data - Repository statistics object
   */
  addRepoData(data: RepoStats): void;

  /**
   * Get total statistics across all analyzed repositories.
   */
  getTotalStats(): TotalStats;

  /**
   * Get daily activity summaries.
   * @param days - Number of days to include (default: 7)
   */
  getDailyActivity(days?: number): ActivitySummary[];

  /**
   * Get weekly activity summaries.
   * @param weeks - Number of weeks to include (default: 4)
   */
  getWeeklyActivity(weeks?: number): ActivitySummary[];

  /**
   * Get complete dashboard data including all stats and activity.
   */
  getDashboardData(): DashboardData;

  /**
   * Get all analyzed repositories.
   */
  getRepos(): RepoStats[];

  /**
   * Export as Markdown report.
   */
  exportMarkdown(): string;

  /**
   * Export as LinkedIn post.
   */
  exportLinkedIn(): string;

  /**
   * Export as professional portfolio.
   */
  exportPortfolio(): string;

  /**
   * Export as README badge/widget.
   */
  exportBadge(): string;

  /**
   * Export as JSON string.
   */
  exportJson(): string;
}

/**
 * Classify a single file by its path.
 *
 * @param file_path - Path to the file
 * @param lines_added - Number of lines added
 * @param lines_removed - Number of lines removed
 * @returns File classification result
 */
export function classifyFile(
  file_path: string,
  lines_added: number,
  lines_removed: number
): FileClassification;

/**
 * Initialize the WASM module.
 */
export function init(): void;
