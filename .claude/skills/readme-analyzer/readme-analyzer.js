#!/usr/bin/env node

/**
 * README Analyzer
 * Analyzes README.md files against repository best practices
 */

const fs = require('fs');
const path = require('path');

// Best practices checklist
const CHECKLIST = {
  essential: [
    { name: 'Title', patterns: [/^#\s+\w+/m, /^___[^_]+___$/m], weight: 10 },
    { name: 'Description', patterns: [/^(?!#)(?!>)[A-Z].{50,}/m], weight: 10 },
    { name: 'Installation', patterns: [/##\s*Installation/mi, /##\s*Install/mi, /##\s*Setup/mi, /##\s*Getting Started/mi, /##\s*Quick Start/mi, /##\s*Quickstart/mi], weight: 10 },
    { name: 'Usage', patterns: [/##\s*Usage/mi, /##\s*How to use/mi, /##\s*Examples/mi, /##\s*Quickstart/mi, /##\s*Quick Start/mi, /###\s*Usage/mi], weight: 10 },
    { name: 'License', patterns: [/##?\s*License/mi, /MIT\b/mi, /Apache\b/mi, /GPL\b/mi, /SPDX-License-Identifier:/m], weight: 5 },
    { name: 'Badges', patterns: [/\[!\[.*?\]\(.*?\)\]\(.*?\)/g], weight: 5 }
  ],
  important: [
    { name: 'Prerequisites', patterns: [/##\s*Prerequisites/mi, /##\s*Requirements/mi, /##\s*Dependencies/mi, /##\s*Prerequisite/mi], weight: 6 },
    { name: 'Configuration', patterns: [/##\s*Configuration/mi, /##\s*Config/mi, /##\s*Settings/mi, /###\s*Configuration/mi], weight: 6 },
    { name: 'Contributing', patterns: [/##\s*Contributing/mi, /CONTRIBUTING.md/mi, /##\s*Contributing Guidelines/mi], weight: 6 },
    { name: 'Changelog', patterns: [/##\s*Changelog/mi, /##\s*Changes/mi, /##\s*Version History/mi, /CHANGELOG.md/mi, /##\s*What's New/mi], weight: 6 },
    { name: 'Authors', patterns: [/##\s*Authors/mi, /##\s*Maintainers/mi, /##\s*Credits/mi, /##\s*Author/mi], weight: 6 }
  ],
  niceToHave: [
    { name: 'Table of Contents', patterns: [/##\s*Table of Contents/mi, /##\s*Contents/mi, /<details>/m], weight: 3 },
    { name: 'Screenshots', patterns: [/!\[.*?\]\(.*?\.(png|jpg|gif|png)/mi], weight: 3 },
    { name: 'Features', patterns: [/##\s*Features/mi, /##\s*Key Features/mi, /^-\s+.*\n/m, /##\s*Feature/mi], weight: 3 },
    { name: 'Roadmap', patterns: [/##\s*Roadmap/mi, /##\s*Planned/mi, /##\s*Future/mi], weight: 3 },
    { name: 'FAQ', patterns: [/##\s*FAQ/mi, /##\s*Questions/mi, /##\s*Troubleshooting/mi, /##\s*FAQs/mi], weight: 3 }
  ]
};

const QUALITY_CHECKS = {
  hasCodeBlocks: (content) => (/```[\w]*\n[\s\S]*?```/g.test(content)),
  hasSyntaxHighlighting: (content) => (/```\w+\n/m.test(content)),
  hasLists: (content) => (/^\s*[-*+]\s+/m.test(content)),
  hasLinks: (content) => (/\[.*?\]\(.*?\)/g.test(content)),
  hasEmojis: (content) => (/[\u{1F300}-\u{1F9FF}]/u.test(content)),
  hasHeaders: (content) => (/^#{1,6}\s+/m.test(content)),
  reasonableLength: (content) => (content.length > 500 && content.length < 50000),
  hasBadLinks: (content) => {
    const links = content.match(/\[.*?\]\(https?:\/\/[^\)]+\)/g) || [];
    // Check for common broken link patterns
    const badPatterns = [
      /\.github\.com\/user\/reponame/,
      /https?:\/\/example\.com/,
      /https?:\/\/localhost/
    ];
    return links.some(link => badPatterns.some(pattern => pattern.test(link)));
  }
};

function analyzeReadme(content, filePath) {
  const results = {
    path: filePath,
    score: 0,
    maxScore: 100,
    essential: { present: [], missing: [], score: 0 },
    important: { present: [], missing: [], score: 0 },
    niceToHave: { present: [], missing: [], score: 0 },
    quality: { issues: [], score: 0 },
    recommendations: []
  };

  // Analyze sections
  let essentialScore = 0;
  let essentialMax = 0;

  for (const check of CHECKLIST.essential) {
    essentialMax += check.weight;
    const found = check.patterns.some(pattern => pattern.test(content));
    if (found) {
      results.essential.present.push(check.name);
      essentialScore += check.weight;
    } else {
      results.essential.missing.push(check.name);
      results.recommendations.push(`Add ${check.name} section`);
    }
  }

  results.essential.score = essentialScore;
  results.score += essentialScore;

  let importantScore = 0;
  for (const check of CHECKLIST.important) {
    const found = check.patterns.some(pattern => pattern.test(content));
    if (found) {
      results.important.present.push(check.name);
      importantScore += check.weight;
    } else {
      results.important.missing.push(check.name);
    }
  }

  results.important.score = importantScore;
  results.score += importantScore;

  let niceToHaveScore = 0;
  for (const check of CHECKLIST.niceToHave) {
    const found = check.patterns.some(pattern => pattern.test(content));
    if (found) {
      results.niceToHave.present.push(check.name);
      niceToHaveScore += check.weight;
    } else {
      results.niceToHave.missing.push(check.name);
    }
  }

  results.niceToHave.score = niceToHaveScore;
  results.score += niceToHaveScore;

  // Quality checks
  let qualityScore = 5;

  if (!QUALITY_CHECKS.hasCodeBlocks(content)) {
    results.quality.issues.push('No code blocks found');
    results.recommendations.push('Add code examples with proper formatting');
    qualityScore -= 1;
  }

  if (!QUALITY_CHECKS.hasSyntaxHighlighting(content)) {
    results.quality.issues.push('Code blocks lack syntax highlighting');
    results.recommendations.push('Add language identifiers to code blocks (e.g., ```javascript)');
    qualityScore -= 1;
  }

  if (QUALITY_CHECKS.hasBadLinks(content)) {
    results.quality.issues.push('Potentially broken or placeholder links detected');
    results.recommendations.push('Review and fix placeholder/example links');
    qualityScore -= 2;
  }

  if (!QUALITY_CHECKS.reasonableLength(content)) {
    results.quality.issues.push(content.length < 500 ? 'README too short' : 'README very long, consider adding TOC');
    qualityScore -= 1;
  }

  results.quality.score = Math.max(0, qualityScore);
  results.score += results.quality.score;

  return results;
}

function printReport(results, options) {
  const score = results.score;
  const maxScore = results.maxScore;
  const percentage = Math.round((score / maxScore) * 100);

  let rating = 'Needs Improvement';
  let stars = '';
  if (percentage >= 90) {
    rating = 'Excellent';
    stars = '⭐⭐⭐';
  } else if (percentage >= 75) {
    rating = 'Good';
    stars = '⭐⭐';
  } else if (percentage >= 60) {
    rating = 'Fair';
    stars = '⭐';
  }

  console.log(`\n📊 README Analysis: ${results.path}`);
  console.log('━'.repeat(60));
  console.log(`Score: ${score}/${maxScore} (${percentage}%) - ${rating} ${stars}\n`);

  // Essential sections
  console.log('✅ Essential Sections:');
  console.log(`   Present: ${results.essential.present.length}/${CHECKLIST.essential.length}`);
  results.essential.present.forEach(item => console.log(`   • ${item} ✓`));
  if (results.essential.missing.length > 0) {
    console.log(`   Missing: ${results.essential.missing.length}`);
    results.essential.missing.forEach(item => console.log(`   • ${item} ✗`));
  }

  // Important sections
  console.log('\n📋 Important Sections:');
  console.log(`   Present: ${results.important.present.length}/${CHECKLIST.important.length}`);
  results.important.present.forEach(item => console.log(`   • ${item} ✓`));
  if (results.important.missing.length > 0) {
    console.log(`   Missing: ${results.important.missing.length}`);
    results.important.missing.forEach(item => console.log(`   • ${item} ✗`));
  }

  // Nice-to-have sections
  if (options.verbose) {
    console.log('\n💡 Nice-to-Have Sections:');
    console.log(`   Present: ${results.niceToHave.present.length}/${CHECKLIST.niceToHave.length}`);
    results.niceToHave.present.forEach(item => console.log(`   • ${item} ✓`));
  }

  // Quality issues
  if (results.quality.issues.length > 0) {
    console.log('\n⚠️  Quality Issues:');
    results.quality.issues.forEach(issue => console.log(`   • ${issue}`));
  }

  // Recommendations
  if (results.recommendations.length > 0) {
    console.log('\n💡 Recommendations:');
    results.recommendations.slice(0, 5).forEach((rec, i) => console.log(`   ${i + 1}. ${rec}`));
    if (results.recommendations.length > 5) {
      console.log(`   ... and ${results.recommendations.length - 5} more`);
    }
  }

  console.log('');
}

// Main execution
function main() {
  const args = process.argv.slice(2);
  let readmePath = 'README.md';
  let options = { verbose: false, scoreOnly: false };

  for (const arg of args) {
    if (arg === '--verbose' || arg === '-v') {
      options.verbose = true;
    } else if (arg === '--score') {
      options.scoreOnly = true;
    } else if (!arg.startsWith('-')) {
      readmePath = arg;
    }
  }

  // Find README if not specified
  if (!fs.existsSync(readmePath)) {
    const possibleFiles = [
      'README.md',
      'readme.md',
      'README.markdown',
      'README.txt'
    ];

    for (const file of possibleFiles) {
      if (fs.existsSync(file)) {
        readmePath = file;
        break;
      }
    }

    if (!fs.existsSync(readmePath)) {
      console.error('No README.md found in current directory');
      process.exit(1);
    }
  }

  const content = fs.readFileSync(readmePath, 'utf8');
  const results = analyzeReadme(content, readmePath);

  if (options.scoreOnly) {
    console.log(results.score);
  } else {
    printReport(results, options);
  }

  // Exit with code based on score
  process.exit(results.score >= 60 ? 0 : 1);
}

if (require.main === module) {
  main();
}

module.exports = { analyzeReadme, CHECKLIST };
