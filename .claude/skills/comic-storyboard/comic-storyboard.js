#!/usr/bin/env node

/**
 * Comic Storyboard Generator
 *
 * Analyzes README files or project descriptions and generates 4-part comic storyboards
 * optimized for Freepik creation and visual storytelling.
 */

const fs = require('fs');
const path = require('path');

// Extract key features from README
function extractFeatures(readmeContent) {
  const features = [];

  // Extract feature sections (lines starting with - or *)
  const featureRegex = /^[\s]*[-*]\s+\*\*(.+?)\*\*:?\s*(.+?)$/gm;
  let match;

  while ((match = featureRegex.exec(readmeContent)) !== null) {
    features.push({
      name: match[1].trim(),
      description: match[2].trim()
    });
  }

  // If no features found, look for sections with "Features" heading
  if (features.length === 0) {
    const featuresSection = readmeContent.match(/##\s*[Ff]eatures?\n([\s\S]+?)(?=\n##|$)/);
    if (featuresSection) {
      const lines = featuresSection[1].split('\n');
      lines.forEach(line => {
        const featureMatch = line.match(/^[-*]\s+(.+)/);
        if (featureMatch) {
          features.push({
            name: featureMatch[1],
            description: ''
          });
        }
      });
    }
  }

  return features;
}

// Extract problem/solution from README
function extractProblemSolution(readmeContent) {
  const result = {
    problem: '',
    solution: '',
    targetAudience: ''
  };

  // Look for problem statements
  const problemPatterns = [
    /(?:problem|challenge|pain point)[s:]?:?\s*([^\n]+)/gi,
    /(?:struggles?|frustrated?|tired of)[^\n]*?([^\n]+)/gi
  ];

  for (const pattern of problemPatterns) {
    const match = pattern.exec(readmeContent);
    if (match) {
      result.problem = match[1];
      break;
    }
  }

  // Look for solution/benefit statements
  const solutionPatterns = [
    /(?:solution|helps?|enables?|allows?)[^\n]*?([^\n]+)/gi,
    /(?:benefit|advantage|value)[^\n]*?([^\n]+)/gi
  ];

  for (const pattern of solutionPatterns) {
    const match = pattern.exec(readmeContent);
    if (match) {
      result.solution = match[1];
      break;
    }
  }

  // Extract target audience
  const audiencePatterns = [
    /(?:for|targeting?|built for)[^\n]*?([A-Z][a-z]+(?:s|ers?))/gi,
    /(?:helps?|enables?)([^\n]*?)(?:to|with)/gi
  ];

  for (const pattern of audiencePatterns) {
    const match = pattern.exec(readmeContent);
    if (match) {
      result.targetAudience = match[1];
      break;
    }
  }

  return result;
}

// Generate storyboard from features
function generateStoryboard(features, problemSolution) {
  const storyboard = {
    title: '4-Part Comic Storyboard',
    panels: []
  };

  // Panel 1: The Problem
  storyboard.panels.push({
    part: 1,
    title: 'The Problem',
    emoji: '📉',
    scene: {
      character: problemSolution.targetAudience || 'User',
      setting: 'Home/Office',
      visualMetaphor: 'Overwhelmed, blocked path, or heavy burden'
    },
    visualElements: [
      'Character looking frustrated or exhausted',
      'Visual representation of the problem',
      'Failed attempts or cancelled plans'
    ],
    text: {
      dialogue: problemSolution.problem || "Why does this keep happening to me?",
      caption: `The daily struggle that ${problemSolution.targetAudience || 'users'} face.`
    },
    colors: ['Muted blues', 'Grays', 'Conveys struggle/pain']
  });

  // Panel 2: The Discovery
  const topFeature = features[0] || { name: 'the solution', description: 'addresses your needs' };
  storyboard.panels.push({
    part: 2,
    title: 'The Discovery',
    emoji: '🔍',
    scene: {
      character: problemSolution.targetAudience || 'User',
      setting: 'At computer/dashboard',
      visualStyle: 'Lightbulb moment with glow effect'
    },
    visualElements: [
      'Dashboard or interface showing insight',
      `Pattern or key realization highlighted`,
      '"Aha moment" expression'
    ],
    text: {
      dialogue: `Finally! ${topFeature.name} makes sense of it all.`,
      caption: `The insight that changes everything: ${topFeature.description}`
    },
    colors: ['Transition colors', 'Yellows for hope', 'Discovery phase']
  });

  // Panel 3: The Solution
  storyboard.panels.push({
    part: 3,
    title: 'The Solution',
    emoji: '🛡️',
    scene: {
      character: problemSolution.targetAudience || 'User',
      setting: 'Taking action with confidence',
      visualMetaphor: 'Shield, guidance system, or clear path'
    },
    visualElements: [
      'Character using the solution confidently',
      'Tool providing clear recommendation',
      'Empowerment symbol (checkmark, shield, progress bar)'
    ],
    text: {
      dialogue: `Today's risk: LOW. I can do this!`,
      caption: 'No more guessing. Just confidence and clarity.'
    },
    colors: ['Warm greens', 'Yellows', 'Safety and optimism']
  });

  // Panel 4: The Result
  storyboard.panels.push({
    part: 4,
    title: 'The Result',
    emoji: '🎉',
    scene: {
      character: problemSolution.targetAudience || 'User',
      setting: 'Success context, social setting, achievement',
      visualMetaphor: 'Transformation, connection, victory'
    },
    visualElements: [
      'Before/after comparison',
      'Social proof (friends, colleagues, family benefiting)',
      'Success metrics or stats'
    ],
    text: {
      dialogue: `I finally got my life back!`,
      caption: problemSolution.solution || 'Transformed by the power of smart solutions.'
    },
    colors: ['Vibrant mix', 'All colors', 'Joy, vitality, success']
  });

  return storyboard;
}

// Format storyboard as markdown
function formatMarkdown(storyboard) {
  let output = `# ${storyboard.title}\n\n`;

  storyboard.panels.forEach(panel => {
    output += `### **PART ${panel.part}: ${panel.title}** ${panel.emoji}\n\n`;
    output += `**Scene Setup:**\n`;
    output += `- **Character:** ${panel.scene.character}\n`;
    output += `- **Setting:** ${panel.scene.setting}\n`;
    if (panel.scene.visualStyle) {
      output += `- **Visual Style:** ${panel.scene.visualStyle}\n`;
    }
    if (panel.scene.visualMetaphor) {
      output += `- **Visual Metaphor:** ${panel.scene.visualMetaphor}\n`;
    }

    output += `\n**Visual Elements:**\n`;
    panel.visualElements.forEach(element => {
      output += `- ${element}\n`;
    });

    output += `\n**Text/Caption:**\n`;
    output += `> **Panel ${panel.part}:**\n`;
    output += `> "${panel.text.dialogue}"\n\n`;
    output += `> *Caption: "${panel.text.caption}"*\n`;

    output += `\n**Color Palette:** ${panel.colors.join(', ')}\n\n`;
    output += `---\n\n`;
  });

  // Add Freepik tips
  output += `## 📝 Freepik Creation Tips\n\n`;
  output += `**Search Keywords:**\n`;
  output += `- Panel 1: "exhausted professional", "frustrated user", "overwhelmed"\n`;
  output += `- Panel 2: "discovery moment", "aha realization", "insight"\n`;
  output += `- Panel 3: "confident user", "taking action", "empowered"\n`;
  output += `- Panel 4: "success", "celebration", "achievement", "happy team"\n\n`;

  output += `**Style Consistency:**\n`;
  output += `- Use the same illustration style across all 4 panels\n`;
  output += `- Keep character appearance consistent\n`;
  output += `- Simple backgrounds (don't clutter)\n`;
  output += `- Large, readable fonts for text\n\n`;

  return output;
}

// Main function
function generateStoryboardFromREADME(readmePath) {
  const content = fs.readFileSync(readmePath, 'utf8');
  const features = extractFeatures(content);
  const problemSolution = extractProblemSolution(content);
  const storyboard = generateStoryboard(features, problemSolution);

  return formatMarkdown(storyboard);
}

// CLI interface
if (require.main === module) {
  const args = process.argv.slice(2);

  if (args.length === 0) {
    console.log('Usage: node comic-storyboard.js <README.md>');
    console.log('');
    console.log('Generates a 4-part comic storyboard from README features');
    console.log('Optimized for Freepik visual creation.');
    process.exit(1);
  }

  const readmePath = args[0];

  if (!fs.existsSync(readmePath)) {
    console.error(`Error: File not found: ${readmePath}`);
    process.exit(1);
  }

  try {
    const storyboard = generateStoryboardFromREADME(readmePath);
    console.log(storyboard);
  } catch (error) {
    console.error(`Error generating storyboard: ${error.message}`);
    process.exit(1);
  }
}

module.exports = {
  extractFeatures,
  extractProblemSolution,
  generateStoryboard,
  generateStoryboardFromREADME,
  formatMarkdown
};
