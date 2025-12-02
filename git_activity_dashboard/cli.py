"""
Command-line interface for Git Activity Dashboard.
"""

import argparse
import os
import sys
from typing import List, Optional

from .analyzer import GitAnalyzer
from .exporters import (
    JSONExporter,
    MarkdownExporter,
    LinkedInExporter,
    PortfolioExporter,
    ReadmeBadgeExporter,
)


def create_parser() -> argparse.ArgumentParser:
    """Create the argument parser."""
    parser = argparse.ArgumentParser(
        description="Git Activity Dashboard - Analyze your git contributions across repositories",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Analyze current directory
  python -m git_activity_dashboard

  # Analyze specific repos
  python -m git_activity_dashboard -r ~/projects/repo1 ~/projects/repo2

  # Scan a directory for all git repos
  python -m git_activity_dashboard -s ~/projects

  # Filter by author email
  python -m git_activity_dashboard -s ~/projects -e your@email.com

  # Export to different formats
  python -m git_activity_dashboard -s ~/projects --json output.json
  python -m git_activity_dashboard -s ~/projects --markdown report.md
  python -m git_activity_dashboard -s ~/projects --portfolio portfolio.md
  python -m git_activity_dashboard -s ~/projects --linkedin linkedin.txt
        """
    )

    parser.add_argument(
        '-r', '--repos',
        nargs='+',
        help='Specific repository paths to analyze'
    )

    parser.add_argument(
        '-s', '--scan',
        type=str,
        help='Scan directory for git repositories'
    )

    parser.add_argument(
        '-d', '--depth',
        type=int,
        default=3,
        help='Maximum depth when scanning for repos (default: 3)'
    )

    parser.add_argument(
        '-e', '--email',
        type=str,
        help='Filter commits by author email'
    )

    parser.add_argument(
        '-a', '--author',
        type=str,
        help='Filter commits by author name'
    )

    parser.add_argument(
        '--json',
        type=str,
        metavar='FILE',
        help='Export to JSON file'
    )

    parser.add_argument(
        '--markdown',
        type=str,
        metavar='FILE',
        help='Export to Markdown file'
    )

    parser.add_argument(
        '--linkedin',
        type=str,
        metavar='FILE',
        help='Export LinkedIn-ready summary'
    )

    parser.add_argument(
        '--portfolio',
        type=str,
        metavar='FILE',
        help='Export project portfolio'
    )

    parser.add_argument(
        '--badge',
        type=str,
        metavar='FILE',
        help='Export README badge/widget'
    )

    parser.add_argument(
        '--all-exports',
        type=str,
        metavar='DIR',
        help='Export all formats to specified directory'
    )

    parser.add_argument(
        '-q', '--quiet',
        action='store_true',
        help='Suppress console output'
    )

    return parser


def print_summary(analyzer: GitAnalyzer) -> None:
    """Print a summary to the console."""
    stats = analyzer.get_total_stats()

    print("\n" + "=" * 60)
    print("GIT ACTIVITY DASHBOARD")
    print("=" * 60)

    print(f"\nRepositories analyzed: {stats['total_repos']}")
    print(f"Total commits: {stats['total_commits']:,}")
    print(f"Lines added: {stats['total_lines_added']:,}")
    print(f"Lines removed: {stats['total_lines_removed']:,}")
    print(f"Files changed: {stats['total_files_changed']:,}")

    print("\n" + "-" * 40)
    print("CONTRIBUTION BREAKDOWN")
    print("-" * 40)

    from .classifier import ContributionType
    type_labels = {
        ContributionType.PRODUCTION_CODE: "Production Code",
        ContributionType.TESTS: "Tests",
        ContributionType.DOCUMENTATION: "Documentation",
        ContributionType.SPECS_CONFIG: "Specs & Config",
        ContributionType.INFRASTRUCTURE: "Infrastructure",
        ContributionType.STYLING: "Styling",
        ContributionType.OTHER: "Other",
    }

    for ctype, count in sorted(
        stats['contribution_types'].items(),
        key=lambda x: x[1],
        reverse=True
    ):
        pct = stats['contribution_percentages'].get(ctype, 0)
        label = type_labels.get(ctype, str(ctype))
        bar = "" * int(pct / 2)
        print(f"  {label:20} {pct:5.1f}% {bar}")

    if stats['languages']:
        print("\n" + "-" * 40)
        print("TOP LANGUAGES")
        print("-" * 40)

        total = sum(stats['languages'].values())
        for lang, count in sorted(
            stats['languages'].items(),
            key=lambda x: x[1],
            reverse=True
        )[:8]:
            pct = (count / total) * 100
            bar = "" * int(pct / 2)
            print(f"  {lang:20} {pct:5.1f}% {bar}")

    # Weekly activity
    weekly = analyzer.get_weekly_activity(4)
    if weekly:
        print("\n" + "-" * 40)
        print("WEEKLY ACTIVITY")
        print("-" * 40)

        for week in weekly:
            total_lines = week.lines_added + week.lines_removed
            bar = "" * min(int(week.commits / 2), 20)
            print(f"  {week.period_label:20} {week.commits:3} commits  {total_lines:6,} lines  {bar}")

    # Repository list
    print("\n" + "-" * 40)
    print("REPOSITORIES")
    print("-" * 40)

    for repo in sorted(analyzer.repos, key=lambda r: r.total_commits, reverse=True)[:10]:
        techs = ', '.join(repo.technologies[:3]) if repo.technologies else 'N/A'
        print(f"  {repo.name:30} {repo.total_commits:4} commits  [{techs}]")

    print("\n" + "=" * 60 + "\n")


def main(args: Optional[List[str]] = None) -> int:
    """Main entry point."""
    parser = create_parser()
    opts = parser.parse_args(args)

    # Determine which repos to analyze
    repo_paths: List[str] = []

    if opts.repos:
        repo_paths = [os.path.abspath(r) for r in opts.repos]
    elif opts.scan:
        scan_path = os.path.abspath(opts.scan)
        if not os.path.isdir(scan_path):
            print(f"Error: {scan_path} is not a directory", file=sys.stderr)
            return 1

        analyzer = GitAnalyzer(author_email=opts.email, author_name=opts.author)
        repo_paths = analyzer.find_git_repos(scan_path, opts.depth)

        if not repo_paths:
            print(f"No git repositories found in {scan_path}", file=sys.stderr)
            return 1

        if not opts.quiet:
            print(f"Found {len(repo_paths)} repositories")
    else:
        # Default to current directory
        cwd = os.getcwd()
        if os.path.isdir(os.path.join(cwd, '.git')):
            repo_paths = [cwd]
        else:
            print("Error: Current directory is not a git repository.", file=sys.stderr)
            print("Use -r to specify repos or -s to scan a directory.", file=sys.stderr)
            return 1

    # Create analyzer and analyze repos
    analyzer = GitAnalyzer(author_email=opts.email, author_name=opts.author)

    for path in repo_paths:
        if not opts.quiet:
            print(f"Analyzing: {path}")
        analyzer.analyze_repo(path)

    if not analyzer.repos:
        print("No repositories were successfully analyzed.", file=sys.stderr)
        return 1

    # Print summary unless quiet
    if not opts.quiet:
        print_summary(analyzer)

    # Handle exports
    if opts.all_exports:
        export_dir = opts.all_exports
        os.makedirs(export_dir, exist_ok=True)

        JSONExporter(analyzer).export(os.path.join(export_dir, 'activity.json'))
        MarkdownExporter(analyzer).export(os.path.join(export_dir, 'report.md'))
        LinkedInExporter(analyzer).export(os.path.join(export_dir, 'linkedin.txt'))
        PortfolioExporter(analyzer).export(os.path.join(export_dir, 'portfolio.md'))
        ReadmeBadgeExporter(analyzer).export(os.path.join(export_dir, 'badge.md'))

        if not opts.quiet:
            print(f"All exports saved to: {export_dir}")

    else:
        if opts.json:
            JSONExporter(analyzer).export(opts.json)
            if not opts.quiet:
                print(f"JSON exported to: {opts.json}")

        if opts.markdown:
            MarkdownExporter(analyzer).export(opts.markdown)
            if not opts.quiet:
                print(f"Markdown exported to: {opts.markdown}")

        if opts.linkedin:
            LinkedInExporter(analyzer).export(opts.linkedin)
            if not opts.quiet:
                print(f"LinkedIn summary exported to: {opts.linkedin}")

        if opts.portfolio:
            PortfolioExporter(analyzer).export(opts.portfolio)
            if not opts.quiet:
                print(f"Portfolio exported to: {opts.portfolio}")

        if opts.badge:
            ReadmeBadgeExporter(analyzer).export(opts.badge)
            if not opts.quiet:
                print(f"Badge exported to: {opts.badge}")

    return 0


if __name__ == '__main__':
    sys.exit(main())
