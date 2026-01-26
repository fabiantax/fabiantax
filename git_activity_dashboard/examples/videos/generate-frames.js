#!/usr/bin/env node
/**
 * Git Activity Frame Generator
 *
 * Generates PNG frames from git activity data
 * Use ffmpeg to convert frames to MP4: ffmpeg -r 60 -i frames/frame-%04d.png -c:v libx264 output.mp4
 */

const { createCanvas } = require('canvas');
const fs = require('fs');

const inputFile = process.argv[2] || './activity.json';
const fps = 60;
const duration = 6;
const totalFrames = fps * duration;

// Load data
const json = fs.readFileSync(inputFile, 'utf8');
const parsed = JSON.parse(json);

const gitData = parsed.weeks.map((week, i) => ({
  period: week,
  commits: parsed.commits[i],
  additions: parsed.additions[i],
  deletions: parsed.deletions[i],
  netLoc: parsed.netLoc[i]
}));

console.log(`📊 Generating ${totalFrames} frames from ${gitData.length} weeks of data...`);

const width = 1920;
const height = 1080;
const canvas = createCanvas(width, height);
const ctx = canvas.getContext('2d');

const framesDir = './frames';
if (!fs.existsSync(framesDir)) {
  fs.mkdirSync(framesDir);
}

const maxCommits = Math.max(...gitData.map(d => d.commits));
const chartTop = 200;
const chartBottom = 800;
const chartHeight = chartBottom - chartTop;
const chartLeft = 100;
const chartRight = width - 100;
const chartWidth = chartRight - chartLeft;
const barWidth = 50;
const gap = (chartWidth - barWidth * gitData.length) / (gitData.length + 1);

for (let frame = 0; frame < totalFrames; frame++) {
  const gradient = ctx.createLinearGradient(0, 0, width, height);
  gradient.addColorStop(0, '#0a0a0f');
  gradient.addColorStop(1, '#16162a');
  ctx.fillStyle = gradient;
  ctx.fillRect(0, 0, width, height);

  ctx.fillStyle = '#7dd3fc';
  ctx.font = 'bold 56px Arial, sans-serif';
  ctx.textAlign = 'center';
  ctx.fillText('🚀 Git Activity Dashboard', width / 2, 80);

  ctx.fillStyle = '#888';
  ctx.font = '24px Arial, sans-serif';
  ctx.fillText('November 2025 - January 2026', width / 2, 120);

  ctx.strokeStyle = '#333';
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.moveTo(chartLeft, chartBottom);
  ctx.lineTo(chartRight, chartBottom);
  ctx.stroke();

  // Track year changes
  let lastYear = null;

  // Staggered growth: each bar grows after the previous one
  const framesPerBar = 12; // Each bar takes 12 frames (0.2s) to grow
  const totalStaggeredFrames = gitData.length * framesPerBar;

  gitData.forEach((week, i) => {
    const x = chartLeft + gap + i * (barWidth + gap);
    const targetHeight = (week.commits / maxCommits) * chartHeight * 0.8;

    // Each bar starts growing at its own time
    const barStartFrame = i * framesPerBar;
    const barEndFrame = barStartFrame + framesPerBar;

    // Calculate progress for this specific bar
    let barProgress = 0;
    if (frame >= barStartFrame) {
      barProgress = Math.min(1, (frame - barStartFrame) / framesPerBar);
    }

    // Use easing for smooth growth
    const easeOutQuad = t => 1 - (1 - t) * (1 - t);
    const currentHeight = targetHeight * easeOutQuad(barProgress);

    // Extract year and week
    const [year, weekNum] = week.period.split('-');
    const showYear = year !== lastYear;
    if (showYear) lastYear = year;

    const intensity = Math.min(1, week.commits / 150);
    const r = Math.floor(56 + intensity * 69);
    const g = Math.floor(189 + intensity * 22);
    const b = Math.floor(248 - intensity * 50);

    // Only draw bar if it has height > 0
    if (currentHeight > 0) {
      ctx.fillStyle = `rgba(${r}, ${g}, ${b}, 0.8)`;
      ctx.fillRect(x, chartBottom - currentHeight, barWidth, currentHeight);

      ctx.strokeStyle = `rgba(${r}, ${g}, ${b}, 1)`;
      ctx.lineWidth = 2;
      ctx.strokeRect(x, chartBottom - currentHeight, barWidth, currentHeight);
    }

    // Only show labels after bar has finished growing
    if (barProgress >= 1) {
      // Week label
      ctx.fillStyle = '#fff';
      ctx.font = '18px Arial, sans-serif';
      ctx.textAlign = 'center';
      ctx.save();
      ctx.translate(x + barWidth / 2, chartBottom + 25);
      ctx.rotate(-Math.PI / 4);
      ctx.fillText(weekNum, 0, 0);
      ctx.restore();

      // Year label (only when year changes)
      if (showYear) {
        ctx.fillStyle = '#7dd3fc';
        ctx.font = 'bold 16px Arial, sans-serif';
        ctx.textAlign = 'center';
        ctx.save();
        ctx.translate(x + barWidth / 2, chartBottom + 55);
        ctx.rotate(-Math.PI / 4);
        ctx.fillText(year, 0, 0);
        ctx.restore();
      }

      // Commit count (fade in)
      const labelFade = Math.min(1, (frame - barEndFrame) / 15);
      if (labelFade > 0) {
        const alpha = Math.min(1, labelFade);
        ctx.fillStyle = `rgba(0, 255, 136, ${alpha})`;
        ctx.font = 'bold 24px Arial, sans-serif';
        ctx.textAlign = 'center';
        ctx.fillText(week.commits.toString(), x + barWidth / 2, chartBottom - currentHeight - 15);
      }
    }
  });

  // Show summary statistics after all bars have grown
  const allBarsFinished = frame >= (gitData.length * framesPerBar) + 30;
  if (allBarsFinished) {
    const statsProgress = Math.min(1, (frame - (gitData.length * framesPerBar + 30)) / 30);
    const statsAlpha = statsProgress;
    const totalCommits = gitData.reduce((sum, d) => sum + d.commits, 0);
    const totalAdditions = gitData.reduce((sum, d) => sum + d.additions, 0);
    const totalDeletions = gitData.reduce((sum, d) => sum + d.deletions, 0);
    const netLoc = totalAdditions - totalDeletions;

    ctx.fillStyle = `rgba(255, 255, 255, ${statsAlpha})`;
    ctx.font = '28px Arial, sans-serif';
    ctx.textAlign = 'left';

    const statsY = 920;
    const statsX = chartLeft;
    const lineHeight = 35;

    ctx.fillText(`Total Commits: ${totalCommits.toLocaleString()}`, statsX, statsY);
    ctx.fillText(`Lines Added: ${totalAdditions.toLocaleString()}`, statsX, statsY + lineHeight);
    ctx.fillText(`Lines Deleted: ${totalDeletions.toLocaleString()}`, statsX, statsY + lineHeight * 2);
    ctx.fillText(`Net LOC: ${netLoc.toLocaleString()}`, statsX, statsY + lineHeight * 3);
  }

  const progressWidth = (frame / totalFrames) * chartWidth;
  ctx.fillStyle = '#333';
  ctx.fillRect(chartLeft, height - 30, chartWidth, 4);
  ctx.fillStyle = '#7dd3fc';
  ctx.fillRect(chartLeft, height - 30, progressWidth, 4);

  const framePath = `${framesDir}/frame-${frame.toString().padStart(4, '0')}.png`;
  const out = fs.createWriteStream(framePath);
  const stream = canvas.createPNGStream();
  stream.pipe(out);

  if (frame % 30 === 0) {
    const progress = ((frame / totalFrames) * 100).toFixed(1);
    console.log(`  Frame ${frame}/${totalFrames} (${progress}%)`);
  }
}

console.log('✅ Frames saved to ./frames/');
console.log('🎬 Create video with:');
console.log(`   ffmpeg -r ${fps} -i ${framesDir}/frame-%04d.png -c:v libx264 -preset slow -crf 20 -pix_fmt yuv420p git-activity.mp4`);
