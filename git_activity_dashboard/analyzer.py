"""
Core git repository analyzer.
"""

import os
import subprocess
import re
from dataclasses import dataclass, field
from datetime import datetime, timedelta
from typing import Dict, List, Optional, Tuple
from collections import defaultdict

from .classifier import FileClassifier, ContributionType, FileClassification


@dataclass
class CommitInfo:
    """Information about a single commit."""
    hash: str
    author: str
    email: str
    date: datetime
    message: str
    files_changed: int = 0
    lines_added: int = 0
    lines_removed: int = 0
    file_classifications: List[FileClassification] = field(default_factory=list)


@dataclass
class RepoStats:
    """Statistics for a single repository."""
    name: str
    path: str
    total_commits: int = 0
    total_lines_added: int = 0
    total_lines_removed: int = 0
    total_files_changed: int = 0
    first_commit_date: Optional[datetime] = None
    last_commit_date: Optional[datetime] = None
    languages: Dict[str, int] = field(default_factory=dict)
    contribution_types: Dict[ContributionType, int] = field(default_factory=dict)
    commits: List[CommitInfo] = field(default_factory=list)
    technologies: List[str] = field(default_factory=list)
    description: str = ""


@dataclass
class ActivitySummary:
    """Summary of activity across a time period."""
    period_start: datetime
    period_end: datetime
    period_label: str
    commits: int = 0
    lines_added: int = 0
    lines_removed: int = 0
    files_changed: int = 0
    repos_active: int = 0
    contribution_breakdown: Dict[ContributionType, int] = field(default_factory=dict)
    language_breakdown: Dict[str, int] = field(default_factory=dict)


class GitAnalyzer:
    """Analyzes git repositories for activity metrics."""

    def __init__(self, author_email: Optional[str] = None, author_name: Optional[str] = None):
        """
        Initialize the analyzer.

        Args:
            author_email: Filter commits by this email (optional)
            author_name: Filter commits by this name (optional)
        """
        self.author_email = author_email
        self.author_name = author_name
        self.classifier = FileClassifier()
        self._repos: List[RepoStats] = []

    def _run_git_command(self, repo_path: str, args: List[str]) -> Tuple[bool, str]:
        """Run a git command and return the output."""
        try:
            result = subprocess.run(
                ['git'] + args,
                cwd=repo_path,
                capture_output=True,
                text=True,
                timeout=60
            )
            return result.returncode == 0, result.stdout
        except (subprocess.TimeoutExpired, FileNotFoundError) as e:
            return False, str(e)

    def _get_repo_description(self, repo_path: str) -> str:
        """Try to get repository description from README or git description."""
        readme_files = ['README.md', 'README.rst', 'README.txt', 'README']
        for readme in readme_files:
            readme_path = os.path.join(repo_path, readme)
            if os.path.exists(readme_path):
                try:
                    with open(readme_path, 'r', encoding='utf-8', errors='ignore') as f:
                        content = f.read(500)  # First 500 chars
                        # Try to extract first paragraph
                        lines = content.split('\n')
                        desc_lines = []
                        for line in lines:
                            line = line.strip()
                            if line and not line.startswith('#') and not line.startswith('!'):
                                desc_lines.append(line)
                                if len(' '.join(desc_lines)) > 150:
                                    break
                        if desc_lines:
                            return ' '.join(desc_lines)[:200]
                except Exception:
                    pass
        return ""

    def _detect_technologies(self, repo_path: str) -> List[str]:
        """Detect technologies used in the repository."""
        tech_indicators = {
            'package.json': ['Node.js', 'JavaScript'],
            'tsconfig.json': ['TypeScript'],
            'requirements.txt': ['Python'],
            'pyproject.toml': ['Python'],
            'Pipfile': ['Python'],
            'Gemfile': ['Ruby'],
            'Cargo.toml': ['Rust'],
            'go.mod': ['Go'],
            'pom.xml': ['Java', 'Maven'],
            'build.gradle': ['Java', 'Gradle'],
            '*.csproj': ['C#', '.NET'],
            '*.sln': ['C#', '.NET'],
            'Dockerfile': ['Docker'],
            'docker-compose.yml': ['Docker'],
            'kubernetes/': ['Kubernetes'],
            'terraform/': ['Terraform'],
            '.github/workflows/': ['GitHub Actions'],
            'angular.json': ['Angular'],
            'next.config.js': ['Next.js'],
            'nuxt.config.js': ['Nuxt.js'],
            'vue.config.js': ['Vue.js'],
            'tailwind.config.js': ['Tailwind CSS'],
            'webpack.config.js': ['Webpack'],
            '.eslintrc': ['ESLint'],
            'jest.config': ['Jest'],
            'pytest.ini': ['Pytest'],
            'setup.py': ['Python'],
        }

        technologies = set()

        for indicator, techs in tech_indicators.items():
            if '*' in indicator:
                # Glob pattern
                pattern = indicator.replace('*', '')
                for item in os.listdir(repo_path):
                    if pattern in item:
                        technologies.update(techs)
                        break
            elif indicator.endswith('/'):
                # Directory
                if os.path.isdir(os.path.join(repo_path, indicator.rstrip('/'))):
                    technologies.update(techs)
            else:
                # File
                if os.path.exists(os.path.join(repo_path, indicator)):
                    technologies.update(techs)

        return sorted(list(technologies))

    def analyze_repo(self, repo_path: str) -> Optional[RepoStats]:
        """Analyze a single git repository."""
        if not os.path.isdir(os.path.join(repo_path, '.git')):
            return None

        repo_name = os.path.basename(os.path.abspath(repo_path))
        stats = RepoStats(name=repo_name, path=repo_path)

        # Get repo description and technologies
        stats.description = self._get_repo_description(repo_path)
        stats.technologies = self._detect_technologies(repo_path)

        # Build author filter
        author_filter = []
        if self.author_email:
            author_filter.extend(['--author', self.author_email])
        elif self.author_name:
            author_filter.extend(['--author', self.author_name])

        # Get commit log with stats
        log_format = '%H|%an|%ae|%aI|%s'
        success, output = self._run_git_command(repo_path, [
            'log', '--format=' + log_format, '--numstat'
        ] + author_filter)

        if not success:
            return None

        # Parse commits
        current_commit = None
        languages = defaultdict(int)
        contribution_types = defaultdict(int)

        for line in output.split('\n'):
            line = line.strip()
            if not line:
                continue

            # Check if this is a commit line
            if '|' in line and line.count('|') >= 4:
                parts = line.split('|', 4)
                if len(parts) == 5:
                    # Save previous commit
                    if current_commit:
                        stats.commits.append(current_commit)

                    try:
                        commit_date = datetime.fromisoformat(parts[3].replace('Z', '+00:00'))
                    except ValueError:
                        commit_date = datetime.now()

                    current_commit = CommitInfo(
                        hash=parts[0],
                        author=parts[1],
                        email=parts[2],
                        date=commit_date,
                        message=parts[4]
                    )
                    continue

            # Check if this is a numstat line (additions\tdeletions\tfilename)
            numstat_match = re.match(r'^(\d+|-)\t(\d+|-)\t(.+)$', line)
            if numstat_match and current_commit:
                added = 0 if numstat_match.group(1) == '-' else int(numstat_match.group(1))
                removed = 0 if numstat_match.group(2) == '-' else int(numstat_match.group(2))
                filepath = numstat_match.group(3)

                # Classify the file
                classification = self.classifier.classify(filepath, added, removed)
                current_commit.file_classifications.append(classification)
                current_commit.lines_added += added
                current_commit.lines_removed += removed
                current_commit.files_changed += 1

                # Track language and contribution type
                if classification.language:
                    languages[classification.language] += added + removed
                contribution_types[classification.contribution_type] += added + removed

        # Don't forget the last commit
        if current_commit:
            stats.commits.append(current_commit)

        # Calculate totals
        stats.total_commits = len(stats.commits)
        stats.languages = dict(languages)
        stats.contribution_types = dict(contribution_types)

        for commit in stats.commits:
            stats.total_lines_added += commit.lines_added
            stats.total_lines_removed += commit.lines_removed
            stats.total_files_changed += commit.files_changed

        if stats.commits:
            dates = [c.date for c in stats.commits]
            stats.first_commit_date = min(dates)
            stats.last_commit_date = max(dates)

        self._repos.append(stats)
        return stats

    def analyze_multiple_repos(self, repo_paths: List[str]) -> List[RepoStats]:
        """Analyze multiple repositories."""
        results = []
        for path in repo_paths:
            stats = self.analyze_repo(path)
            if stats:
                results.append(stats)
        return results

    def find_git_repos(self, base_path: str, max_depth: int = 3) -> List[str]:
        """Find all git repositories under a base path."""
        repos = []

        def search(path: str, depth: int):
            if depth > max_depth:
                return

            try:
                if os.path.isdir(os.path.join(path, '.git')):
                    repos.append(path)
                    return  # Don't search inside git repos

                for item in os.listdir(path):
                    item_path = os.path.join(path, item)
                    if os.path.isdir(item_path) and not item.startswith('.'):
                        search(item_path, depth + 1)
            except PermissionError:
                pass

        search(base_path, 0)
        return repos

    def get_activity_summary(
        self,
        period: str = 'week',
        custom_start: Optional[datetime] = None,
        custom_end: Optional[datetime] = None
    ) -> ActivitySummary:
        """
        Get activity summary for a time period.

        Args:
            period: 'day', 'week', 'month', or 'custom'
            custom_start: Start date for custom period
            custom_end: End date for custom period
        """
        now = datetime.now().astimezone()

        if period == 'day':
            start = now.replace(hour=0, minute=0, second=0, microsecond=0)
            end = now
            label = "Today"
        elif period == 'week':
            start = now - timedelta(days=now.weekday())
            start = start.replace(hour=0, minute=0, second=0, microsecond=0)
            end = now
            label = "This Week"
        elif period == 'month':
            start = now.replace(day=1, hour=0, minute=0, second=0, microsecond=0)
            end = now
            label = "This Month"
        elif period == 'custom' and custom_start and custom_end:
            start = custom_start
            end = custom_end
            label = f"{start.strftime('%Y-%m-%d')} to {end.strftime('%Y-%m-%d')}"
        else:
            # Default to last 7 days
            start = now - timedelta(days=7)
            end = now
            label = "Last 7 Days"

        summary = ActivitySummary(
            period_start=start,
            period_end=end,
            period_label=label
        )

        active_repos = set()
        contribution_breakdown = defaultdict(int)
        language_breakdown = defaultdict(int)

        for repo in self._repos:
            for commit in repo.commits:
                # Make commit date timezone-aware if it isn't
                commit_date = commit.date
                if commit_date.tzinfo is None:
                    commit_date = commit_date.replace(tzinfo=start.tzinfo)

                if start <= commit_date <= end:
                    summary.commits += 1
                    summary.lines_added += commit.lines_added
                    summary.lines_removed += commit.lines_removed
                    summary.files_changed += commit.files_changed
                    active_repos.add(repo.name)

                    for classification in commit.file_classifications:
                        contribution_breakdown[classification.contribution_type] += (
                            classification.lines_added + classification.lines_removed
                        )
                        if classification.language:
                            language_breakdown[classification.language] += (
                                classification.lines_added + classification.lines_removed
                            )

        summary.repos_active = len(active_repos)
        summary.contribution_breakdown = dict(contribution_breakdown)
        summary.language_breakdown = dict(language_breakdown)

        return summary

    def get_daily_activity(self, days: int = 7) -> List[ActivitySummary]:
        """Get daily activity summaries for the past N days."""
        summaries = []
        now = datetime.now().astimezone()

        for i in range(days):
            day = now - timedelta(days=i)
            start = day.replace(hour=0, minute=0, second=0, microsecond=0)
            end = day.replace(hour=23, minute=59, second=59, microsecond=999999)

            summary = ActivitySummary(
                period_start=start,
                period_end=end,
                period_label=day.strftime('%A, %b %d')
            )

            for repo in self._repos:
                for commit in repo.commits:
                    commit_date = commit.date
                    if commit_date.tzinfo is None:
                        commit_date = commit_date.replace(tzinfo=start.tzinfo)

                    if start <= commit_date <= end:
                        summary.commits += 1
                        summary.lines_added += commit.lines_added
                        summary.lines_removed += commit.lines_removed

            summaries.append(summary)

        return summaries

    def get_weekly_activity(self, weeks: int = 4) -> List[ActivitySummary]:
        """Get weekly activity summaries for the past N weeks."""
        summaries = []
        now = datetime.now().astimezone()

        for i in range(weeks):
            week_start = now - timedelta(days=now.weekday() + (i * 7))
            week_start = week_start.replace(hour=0, minute=0, second=0, microsecond=0)
            week_end = week_start + timedelta(days=6, hours=23, minutes=59, seconds=59)

            summary = ActivitySummary(
                period_start=week_start,
                period_end=week_end,
                period_label=f"Week of {week_start.strftime('%b %d')}"
            )

            active_repos = set()

            for repo in self._repos:
                for commit in repo.commits:
                    commit_date = commit.date
                    if commit_date.tzinfo is None:
                        commit_date = commit_date.replace(tzinfo=week_start.tzinfo)

                    if week_start <= commit_date <= week_end:
                        summary.commits += 1
                        summary.lines_added += commit.lines_added
                        summary.lines_removed += commit.lines_removed
                        summary.files_changed += commit.files_changed
                        active_repos.add(repo.name)

            summary.repos_active = len(active_repos)
            summaries.append(summary)

        return summaries

    @property
    def repos(self) -> List[RepoStats]:
        """Get all analyzed repositories."""
        return self._repos

    def get_total_stats(self) -> Dict:
        """Get aggregated statistics across all repos."""
        total_commits = sum(r.total_commits for r in self._repos)
        total_lines_added = sum(r.total_lines_added for r in self._repos)
        total_lines_removed = sum(r.total_lines_removed for r in self._repos)
        total_files_changed = sum(r.total_files_changed for r in self._repos)

        # Aggregate languages
        all_languages = defaultdict(int)
        for repo in self._repos:
            for lang, count in repo.languages.items():
                all_languages[lang] += count

        # Aggregate contribution types
        all_contribution_types = defaultdict(int)
        for repo in self._repos:
            for ctype, count in repo.contribution_types.items():
                all_contribution_types[ctype] += count

        # Calculate percentages for contribution types
        total_lines = sum(all_contribution_types.values())
        contribution_percentages = {}
        if total_lines > 0:
            for ctype, count in all_contribution_types.items():
                contribution_percentages[ctype] = round((count / total_lines) * 100, 1)

        return {
            'total_repos': len(self._repos),
            'total_commits': total_commits,
            'total_lines_added': total_lines_added,
            'total_lines_removed': total_lines_removed,
            'total_lines_changed': total_lines_added + total_lines_removed,
            'total_files_changed': total_files_changed,
            'languages': dict(all_languages),
            'contribution_types': dict(all_contribution_types),
            'contribution_percentages': contribution_percentages,
        }
