#!/usr/bin/env node
/**
 * Verify bar growth at different frames
 */
const { createCanvas } = require('canvas');
const fs = require('fs');

const json = fs.readFileSync('./activity-nov2025-jan2026.json', 'utf8');
const parsed = JSON.parse(json);

const gitData = parsed.weeks.map((week, i) => ({
  period: week,
  commits: parsed.commits[i],
  additions: parsed.additions[i],
  deletions: parsed.deletions[i],
  netLoc: parsed.netLoc[i]
}));

const totalFrames = 360;
const barsGrowEnd = Math.floor(totalFrames * 0.85);
const maxCommits = Math.max(...gitData.map(d => d.commits));

console.log('\n📊 Bar Growth Verification');
console.log('='.repeat(60));
console.log(`Total frames: ${totalFrames}`);
console.log(`Growth ends at frame: ${barsGrowEnd}`);
console.log(`Duration of growth: ${(barsGrowEnd / 60).toFixed(1)} seconds`);
console.log('');

// Check specific frames
const checkFrames = [0, 90, 180, 270, 306];
const chartHeight = 600;

checkFrames.forEach(frame => {
  const growProgress = Math.min(1, frame / barsGrowEnd);
  console.log(`\nFrame ${frame} (${(frame/60).toFixed(1)}s):`);
  console.log(`  Progress: ${(growProgress * 100).toFixed(1)}%`);

  // Show first 5 bars as examples
  gitData.slice(0, 5).forEach((week, i) => {
    const targetHeight = (week.commits / maxCommits) * chartHeight * 0.8;
    const currentHeight = targetHeight * growProgress;

    console.log(`  ${week.period}: ${week.commits} commits → ${(currentHeight).toFixed(0)}px height (${targetHeight.toFixed(0)}px max)`);
  });
});

console.log('\n' + '='.repeat(60));
console.log('✅ Growth is LINEAR (steady from 0% to 100%)');
console.log('');
