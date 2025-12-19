/**
 * Example usage of git-activity-dashboard from TypeScript
 *
 * This example shows how to use the WASM module in a Node.js/TypeScript environment.
 */

import { execSync } from "child_process";
import { WasmAnalyzer, TotalStats, DashboardData } from "../pkg/git_activity_dashboard";

// Helper to get git log from a repository
function getGitLog(repoPath: string, authorEmail?: string): string {
  const authorFilter = authorEmail ? `--author=${authorEmail}` : "";
  return execSync(
    `git log --format='%H|%an|%ae|%aI|%s' --numstat ${authorFilter}`,
    { cwd: repoPath, encoding: "utf-8", maxBuffer: 50 * 1024 * 1024 }
  );
}

// Helper to find git repos in a directory
function findGitRepos(basePath: string, maxDepth = 3): string[] {
  const result = execSync(
    `find ${basePath} -maxdepth ${maxDepth} -name .git -type d`,
    { encoding: "utf-8" }
  );
  return result
    .trim()
    .split("\n")
    .filter(Boolean)
    .map((p) => p.replace("/.git", ""));
}

async function main() {
  // Create analyzer (optionally filter by email)
  const analyzer = new WasmAnalyzer("your@email.com", null);

  // Option 1: Analyze specific repos
  const repos = ["/path/to/repo1", "/path/to/repo2"];

  // Option 2: Scan a directory
  // const repos = findGitRepos('/home/user/projects');

  // Parse each repository
  for (const repoPath of repos) {
    const repoName = repoPath.split("/").pop() || "unknown";
    console.log(`Analyzing: ${repoName}`);

    try {
      const gitLog = getGitLog(repoPath);
      analyzer.parseGitLog(repoName, repoPath, gitLog);
    } catch (error) {
      console.error(`Failed to analyze ${repoName}:`, error);
    }
  }

  // Get statistics
  const stats: TotalStats = analyzer.getTotalStats();
  console.log("\n=== Total Stats ===");
  console.log(`Repositories: ${stats.total_repos}`);
  console.log(`Commits: ${stats.total_commits}`);
  console.log(`Lines added: ${stats.total_lines_added}`);
  console.log(`Lines removed: ${stats.total_lines_removed}`);

  // Contribution breakdown
  console.log("\n=== Contribution Breakdown ===");
  for (const [type, percentage] of Object.entries(stats.contribution_percentages)) {
    console.log(`${type}: ${percentage}%`);
  }

  // Weekly activity
  const weekly = analyzer.getWeeklyActivity(4);
  console.log("\n=== Weekly Activity ===");
  for (const week of weekly) {
    console.log(`${week.period_label}: ${week.commits} commits, ${week.lines_added + week.lines_removed} lines`);
  }

  // Export options
  console.log("\n=== Exports ===");

  // Markdown report
  const markdown = analyzer.exportMarkdown();
  console.log("Markdown report generated");

  // LinkedIn post
  const linkedin = analyzer.exportLinkedIn();
  console.log("\nLinkedIn post:");
  console.log(linkedin);

  // Portfolio for employers
  const portfolio = analyzer.exportPortfolio();
  console.log("\nPortfolio generated");

  // JSON data for custom visualizations
  const dashboardData: DashboardData = analyzer.getDashboardData();
  console.log(`\nDashboard data has ${dashboardData.repositories.length} repos`);

  // Save exports to files
  // fs.writeFileSync('report.md', markdown);
  // fs.writeFileSync('linkedin.txt', linkedin);
  // fs.writeFileSync('portfolio.md', portfolio);
  // fs.writeFileSync('activity.json', analyzer.exportJson());
}

main().catch(console.error);
