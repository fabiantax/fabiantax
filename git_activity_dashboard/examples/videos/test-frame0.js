#!/usr/bin/env node
const { createCanvas } = require('canvas');

const gitData = [
  { commits: 4 },
  { commits: 33 },
  { commits: 40 },
];

const frame = 0;
const framesPerBar = 12;
const chartHeight = 600;
const chartBottom = 800;
const maxCommits = 90;

console.log('Testing frame 0 bar heights:');
gitData.forEach((week, i) => {
  const barStartFrame = i * framesPerBar;
  const barEndFrame = barStartFrame + framesPerBar;
  
  let barProgress = 0;
  if (frame >= barStartFrame) {
    barProgress = Math.min(1, (frame - barStartFrame) / framesPerBar);
  }
  
  const easeOutQuad = t => 1 - (1 - t) * (1 - t);
  const targetHeight = (week.commits / maxCommits) * chartHeight * 0.8;
  const currentHeight = targetHeight * easeOutQuad(barProgress);
  
  console.log(`  Bar ${i}: frame=${frame}, barStartFrame=${barStartFrame}, barProgress=${barProgress.toFixed(2)}, currentHeight=${currentHeight.toFixed(2)}px`);
});
