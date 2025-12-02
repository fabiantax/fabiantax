"""
Export generators for various formats.
"""

import json
import os
from datetime import datetime
from typing import Dict, List, Optional

from .analyzer import GitAnalyzer, RepoStats, ActivitySummary
from .classifier import ContributionType


class BaseExporter:
    """Base class for exporters."""

    def __init__(self, analyzer: GitAnalyzer):
        self.analyzer = analyzer

    def export(self, output_path: Optional[str] = None) -> str:
        """Export data. Returns the exported content."""
        raise NotImplementedError


class JSONExporter(BaseExporter):
    """Export data to JSON format."""

    def export(self, output_path: Optional[str] = None) -> str:
        """Export all data to JSON."""
        data = {
            'generated_at': datetime.now().isoformat(),
            'summary': self._serialize_stats(self.analyzer.get_total_stats()),
            'repositories': [self._serialize_repo(r) for r in self.analyzer.repos],
            'daily_activity': [self._serialize_summary(s) for s in self.analyzer.get_daily_activity(7)],
            'weekly_activity': [self._serialize_summary(s) for s in self.analyzer.get_weekly_activity(4)],
        }

        content = json.dumps(data, indent=2, default=str)

        if output_path:
            with open(output_path, 'w') as f:
                f.write(content)

        return content

    def _serialize_stats(self, stats: Dict) -> Dict:
        """Serialize stats dict, converting enum keys to strings."""
        result = {}
        for key, value in stats.items():
            if isinstance(value, dict):
                result[key] = {
                    (k.value if isinstance(k, ContributionType) else str(k)): v
                    for k, v in value.items()
                }
            else:
                result[key] = value
        return result

    def _serialize_repo(self, repo: RepoStats) -> Dict:
        """Serialize a RepoStats object."""
        return {
            'name': repo.name,
            'path': repo.path,
            'description': repo.description,
            'technologies': repo.technologies,
            'total_commits': repo.total_commits,
            'total_lines_added': repo.total_lines_added,
            'total_lines_removed': repo.total_lines_removed,
            'total_files_changed': repo.total_files_changed,
            'first_commit_date': repo.first_commit_date.isoformat() if repo.first_commit_date else None,
            'last_commit_date': repo.last_commit_date.isoformat() if repo.last_commit_date else None,
            'languages': repo.languages,
            'contribution_types': {
                k.value: v for k, v in repo.contribution_types.items()
            },
        }

    def _serialize_summary(self, summary: ActivitySummary) -> Dict:
        """Serialize an ActivitySummary object."""
        return {
            'period_label': summary.period_label,
            'period_start': summary.period_start.isoformat(),
            'period_end': summary.period_end.isoformat(),
            'commits': summary.commits,
            'lines_added': summary.lines_added,
            'lines_removed': summary.lines_removed,
            'files_changed': summary.files_changed,
            'repos_active': summary.repos_active,
        }


class MarkdownExporter(BaseExporter):
    """Export data to Markdown format."""

    def export(self, output_path: Optional[str] = None) -> str:
        """Export summary to Markdown."""
        stats = self.analyzer.get_total_stats()
        lines = []

        lines.append("# Git Activity Dashboard")
        lines.append("")
        lines.append(f"*Generated on {datetime.now().strftime('%Y-%m-%d %H:%M')}*")
        lines.append("")

        # Overall stats
        lines.append("## Overview")
        lines.append("")
        lines.append(f"| Metric | Value |")
        lines.append(f"|--------|-------|")
        lines.append(f"| Repositories | {stats['total_repos']} |")
        lines.append(f"| Total Commits | {stats['total_commits']:,} |")
        lines.append(f"| Lines Added | {stats['total_lines_added']:,} |")
        lines.append(f"| Lines Removed | {stats['total_lines_removed']:,} |")
        lines.append(f"| Files Changed | {stats['total_files_changed']:,} |")
        lines.append("")

        # Contribution breakdown
        lines.append("## Contribution Breakdown")
        lines.append("")
        lines.append("| Type | Lines | Percentage |")
        lines.append("|------|-------|------------|")

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
            lines.append(f"| {label} | {count:,} | {pct}% |")
        lines.append("")

        # Languages
        if stats['languages']:
            lines.append("## Languages")
            lines.append("")
            lines.append("| Language | Lines |")
            lines.append("|----------|-------|")
            for lang, count in sorted(
                stats['languages'].items(),
                key=lambda x: x[1],
                reverse=True
            )[:10]:
                lines.append(f"| {lang} | {count:,} |")
            lines.append("")

        # Weekly activity
        lines.append("## Weekly Activity")
        lines.append("")
        weekly = self.analyzer.get_weekly_activity(4)
        lines.append("| Week | Commits | Lines Changed |")
        lines.append("|------|---------|---------------|")
        for week in weekly:
            total_lines = week.lines_added + week.lines_removed
            lines.append(f"| {week.period_label} | {week.commits} | {total_lines:,} |")
        lines.append("")

        # Repository list
        lines.append("## Repositories")
        lines.append("")
        for repo in sorted(self.analyzer.repos, key=lambda r: r.total_commits, reverse=True):
            lines.append(f"### {repo.name}")
            lines.append("")
            if repo.description:
                lines.append(f"> {repo.description}")
                lines.append("")
            if repo.technologies:
                lines.append(f"**Technologies:** {', '.join(repo.technologies)}")
                lines.append("")
            lines.append(f"- Commits: {repo.total_commits}")
            lines.append(f"- Lines: +{repo.total_lines_added:,} / -{repo.total_lines_removed:,}")
            if repo.first_commit_date and repo.last_commit_date:
                lines.append(f"- Active: {repo.first_commit_date.strftime('%Y-%m-%d')} to {repo.last_commit_date.strftime('%Y-%m-%d')}")
            lines.append("")

        content = '\n'.join(lines)

        if output_path:
            with open(output_path, 'w') as f:
                f.write(content)

        return content


class LinkedInExporter(BaseExporter):
    """Export data formatted for LinkedIn posts."""

    def export(self, output_path: Optional[str] = None) -> str:
        """Export a LinkedIn-ready summary."""
        stats = self.analyzer.get_total_stats()
        weekly = self.analyzer.get_weekly_activity(1)[0] if self.analyzer.get_weekly_activity(1) else None

        lines = []

        # Headline
        lines.append("My Developer Activity This Week")
        lines.append("")

        if weekly and weekly.commits > 0:
            lines.append(f"Commits: {weekly.commits}")
            lines.append(f"Lines of code: {weekly.lines_added + weekly.lines_removed:,}")
            lines.append(f"Active repos: {weekly.repos_active}")
            lines.append("")

        # Contribution quality indicators
        pcts = stats['contribution_percentages']
        quality_metrics = []

        test_pct = pcts.get(ContributionType.TESTS, 0)
        if test_pct > 0:
            quality_metrics.append(f"Tests: {test_pct}%")

        doc_pct = pcts.get(ContributionType.DOCUMENTATION, 0)
        if doc_pct > 0:
            quality_metrics.append(f"Documentation: {doc_pct}%")

        if quality_metrics:
            lines.append("Code Quality Breakdown:")
            for metric in quality_metrics:
                lines.append(f"  {metric}")
            lines.append("")

        # Top languages
        if stats['languages']:
            top_langs = sorted(stats['languages'].items(), key=lambda x: x[1], reverse=True)[:3]
            lines.append("Top Languages: " + ", ".join(lang for lang, _ in top_langs))
            lines.append("")

        # Call to action
        lines.append("#coding #developer #programming #softwareengineering")

        content = '\n'.join(lines)

        if output_path:
            with open(output_path, 'w') as f:
                f.write(content)

        return content


class PortfolioExporter(BaseExporter):
    """Export a portfolio of projects for potential employers/clients."""

    def export(self, output_path: Optional[str] = None) -> str:
        """Export a professional portfolio document."""
        lines = []

        lines.append("# Project Portfolio")
        lines.append("")
        lines.append(f"*Generated on {datetime.now().strftime('%Y-%m-%d')}*")
        lines.append("")

        # Summary stats
        stats = self.analyzer.get_total_stats()
        lines.append("## Summary")
        lines.append("")
        lines.append(f"- **Total Projects:** {stats['total_repos']}")
        lines.append(f"- **Total Commits:** {stats['total_commits']:,}")
        lines.append(f"- **Total Lines of Code:** {stats['total_lines_added']:,}")
        lines.append("")

        # Skills based on languages
        if stats['languages']:
            lines.append("## Technical Skills")
            lines.append("")
            sorted_langs = sorted(stats['languages'].items(), key=lambda x: x[1], reverse=True)
            total = sum(count for _, count in sorted_langs)
            for lang, count in sorted_langs[:10]:
                pct = round((count / total) * 100, 1)
                bar = "" * int(pct / 5)
                lines.append(f"- **{lang}**: {pct}% {bar}")
            lines.append("")

        # Code quality indicators
        lines.append("## Code Quality Practices")
        lines.append("")
        pcts = stats['contribution_percentages']

        prod_pct = pcts.get(ContributionType.PRODUCTION_CODE, 0)
        test_pct = pcts.get(ContributionType.TESTS, 0)
        doc_pct = pcts.get(ContributionType.DOCUMENTATION, 0)
        infra_pct = pcts.get(ContributionType.INFRASTRUCTURE, 0)

        lines.append(f"| Category | Percentage |")
        lines.append(f"|----------|------------|")
        lines.append(f"| Production Code | {prod_pct}% |")
        lines.append(f"| Tests | {test_pct}% |")
        lines.append(f"| Documentation | {doc_pct}% |")
        lines.append(f"| Infrastructure/DevOps | {infra_pct}% |")
        lines.append("")

        # Project details
        lines.append("## Projects")
        lines.append("")

        for repo in sorted(self.analyzer.repos, key=lambda r: r.total_commits, reverse=True):
            lines.append(f"### {repo.name}")
            lines.append("")

            if repo.description:
                lines.append(f"{repo.description}")
                lines.append("")

            # Technologies
            if repo.technologies:
                lines.append(f"**Technologies:** {', '.join(repo.technologies)}")
                lines.append("")

            # Contribution summary
            lines.append("**My Contribution:**")
            lines.append(f"- {repo.total_commits} commits")
            lines.append(f"- {repo.total_lines_added:,} lines added, {repo.total_lines_removed:,} lines removed")

            # Duration
            if repo.first_commit_date and repo.last_commit_date:
                duration = (repo.last_commit_date - repo.first_commit_date).days
                if duration > 30:
                    months = duration // 30
                    lines.append(f"- Project duration: {months} month(s)")
                else:
                    lines.append(f"- Project duration: {duration} day(s)")

            # Top languages in this repo
            if repo.languages:
                top_langs = sorted(repo.languages.items(), key=lambda x: x[1], reverse=True)[:3]
                lines.append(f"- Primary languages: {', '.join(lang for lang, _ in top_langs)}")

            lines.append("")
            lines.append("---")
            lines.append("")

        content = '\n'.join(lines)

        if output_path:
            with open(output_path, 'w') as f:
                f.write(content)

        return content


class ReadmeBadgeExporter(BaseExporter):
    """Generate README badges/widgets for GitHub profile."""

    def export(self, output_path: Optional[str] = None) -> str:
        """Generate embeddable stats widget in Markdown."""
        stats = self.analyzer.get_total_stats()
        weekly = self.analyzer.get_weekly_activity(1)
        current_week = weekly[0] if weekly else None

        lines = []

        lines.append("<!-- Git Activity Dashboard Widget -->")
        lines.append("<div align='center'>")
        lines.append("")
        lines.append("### Developer Activity")
        lines.append("")
        lines.append("| Metric | All Time | This Week |")
        lines.append("|--------|----------|-----------|")

        week_commits = current_week.commits if current_week else 0
        week_lines = (current_week.lines_added + current_week.lines_removed) if current_week else 0

        lines.append(f"| Commits | {stats['total_commits']:,} | {week_commits} |")
        lines.append(f"| Lines Changed | {stats['total_lines_changed']:,} | {week_lines:,} |")
        lines.append(f"| Repositories | {stats['total_repos']} | {current_week.repos_active if current_week else 0} |")
        lines.append("")

        # Contribution type badges
        pcts = stats['contribution_percentages']
        badges = []

        if pcts.get(ContributionType.TESTS, 0) > 0:
            badges.append(f"Tests: {pcts[ContributionType.TESTS]}%")
        if pcts.get(ContributionType.DOCUMENTATION, 0) > 0:
            badges.append(f"Docs: {pcts[ContributionType.DOCUMENTATION]}%")

        if badges:
            lines.append(f"**Code Quality:** {' | '.join(badges)}")
            lines.append("")

        lines.append("</div>")
        lines.append("<!-- End Git Activity Dashboard Widget -->")

        content = '\n'.join(lines)

        if output_path:
            with open(output_path, 'w') as f:
                f.write(content)

        return content
