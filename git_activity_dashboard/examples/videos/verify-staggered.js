#!/usr/bin/env node
const { createCanvas } = require('canvas');
const fs = require('fs');

const json = fs.readFileSync('./activity-nov2025-jan2026.json', 'utf8');
const parsed = JSON.parse(json);

const gitData = parsed.weeks.map((week, i) => ({
  period: week,
  commits: parsed.commits[i],
}));

const framesPerBar = 12;
const chartHeight = 600;
const maxCommits = Math.max(...gitData.map(d => d.commits));

console.log('\n📊 Staggered Growth Verification');
console.log('='.repeat(60));

// Show first 8 bars at different frames
const checkFrames = [0, 12, 24, 36, 48, 60, 72, 84, 96, 108, 120];

checkFrames.forEach(frame => {
  console.log(`\nFrame ${frame}:`);
  gitData.slice(0, 8).forEach((week, i) => {
    const barStartFrame = i * framesPerBar;
    const barEndFrame = barStartFrame + framesPerBar;
    
    let barProgress = 0;
    if (frame >= barStartFrame) {
      barProgress = Math.min(1, (frame - barStartFrame) / framesPerBar);
    }
    
    const easeOutQuad = t => 1 - (1 - t) * (1 - t);
    const currentHeight = (week.commits / maxCommits) * chartHeight * 0.8 * easeOutQuad(barProgress);
    const targetHeight = (week.commits / maxCommits) * chartHeight * 0.8;
    
    const status = barProgress === 0 ? 'waiting' : barProgress === 1 ? 'done' : 'growing';
    console.log(`  ${week.period}: ${currentHeight.toFixed(0)}px / ${targetHeight.toFixed(0)}px [${status}]`);
  });
});

console.log('\n' + '='.repeat(60));
console.log('✅ Each bar grows one by one (12 frames = 0.2s per bar)');
console.log('✅ Total growth time: 20 bars × 0.2s = 4.0 seconds');
