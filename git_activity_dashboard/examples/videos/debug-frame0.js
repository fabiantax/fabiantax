#!/usr/bin/env node
/**
 * Debug script to verify frame generation
 */
const { createCanvas } = require('canvas');

const json = require('./activity-nov2025-jan2026.json');
const gitData = json.weeks.map((week, i) => ({
  period: week,
  commits: json.commits[i],
}));

const frame = 0;
const framesPerBar = 12;
const width = 1920;
const height = 1080;
const chartHeight = 600;
const chartBottom = 800;
const chartLeft = 100;
const chartRight = width - 100;
const chartWidth = chartRight - chartLeft;
const barWidth = 50;
const gap = (chartWidth - barWidth * gitData.length) / (gitData.length + 1);
const maxCommits = Math.max(...gitData.map(d => d.commits));

const canvas = createCanvas(width, height);
const ctx = canvas.getContext('2d');

// Draw background
const gradient = ctx.createLinearGradient(0, 0, width, height);
gradient.addColorStop(0, '#0a0a0f');
gradient.addColorStop(1, '#16162a');
ctx.fillStyle = gradient;
ctx.fillRect(0, 0, width, height);

// Draw axis
ctx.strokeStyle = '#333';
ctx.lineWidth = 2;
ctx.beginPath();
ctx.moveTo(chartLeft, chartBottom);
ctx.lineTo(chartRight, chartBottom);
ctx.stroke();

console.log('\n🎨 Frame 0 Rendering:');
console.log('='.repeat(60));

gitData.forEach((week, i) => {
  const x = chartLeft + gap + i * (barWidth + gap);
  const targetHeight = (week.commits / maxCommits) * chartHeight * 0.8;

  const barStartFrame = i * framesPerBar;
  let barProgress = 0;
  if (frame >= barStartFrame) {
    barProgress = Math.min(1, (frame - barStartFrame) / framesPerBar);
  }

  const easeOutQuad = t => 1 - (1 - t) * (1 - t);
  const currentHeight = targetHeight * easeOutQuad(barProgress);

  // Only draw if height > 0
  if (currentHeight > 0) {
    const intensity = Math.min(1, week.commits / 150);
    const r = Math.floor(56 + intensity * 69);
    const g = Math.floor(189 + intensity * 22);
    const b = Math.floor(248 - intensity * 50);

    ctx.fillStyle = `rgba(${r}, ${g}, ${b}, 0.8)`;
    ctx.fillRect(x, chartBottom - currentHeight, barWidth, currentHeight);
  }

  console.log(`  ${week.period}: targetHeight=${targetHeight.toFixed(0)}px, currentHeight=${currentHeight.toFixed(2)}px, drawn=${currentHeight > 0 ? 'YES' : 'NO'}`);
});

console.log('='.repeat(60));
console.log('Saving to debug-frame0.png...');
const out = require('fs').createWriteStream('debug-frame0.png');
canvas.createPNGStream().pipe(out);
console.log('✅ Saved! Open debug-frame0.png to verify frame 0 has no bars.');
