// Git Activity Constellation - Cosmos.gl Visualization
// Creates an interactive graph of your coding activity

const COLORS = {
    repo: '#f472b6',        // Pink
    language: '#7dd3fc',     // Cyan
    contribution: '#a78bfa', // Purple
    extension: '#34d399',    // Green
    edge: '#333333',
    edgeHighlight: '#666666'
};

const NODE_TYPES = {
    REPO: 'repo',
    LANGUAGE: 'language',
    CONTRIBUTION: 'contribution',
    EXTENSION: 'extension'
};

let cosmos = null;
let nodes = [];
let links = [];
let nodeMap = new Map();

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    setupDragDrop();

    // Check URL for data parameter
    const params = new URLSearchParams(window.location.search);
    const dataUrl = params.get('data');
    if (dataUrl) {
        fetchAndLoad(dataUrl);
    }
});

function setupDragDrop() {
    const container = document.getElementById('cosmos-container');

    container.addEventListener('dragover', (e) => {
        e.preventDefault();
        e.stopPropagation();
    });

    container.addEventListener('drop', (e) => {
        e.preventDefault();
        e.stopPropagation();

        const file = e.dataTransfer.files[0];
        if (file && file.name.endsWith('.json')) {
            const reader = new FileReader();
            reader.onload = (event) => {
                document.getElementById('git-data').value = event.target.result;
                loadData();
            };
            reader.readAsText(file);
        }
    });
}

async function fetchAndLoad(url) {
    try {
        showLoading();
        const response = await fetch(url);
        const data = await response.json();
        processData(data);
    } catch (error) {
        console.error('Failed to fetch data:', error);
        hideLoading();
    }
}

function loadData() {
    const textarea = document.getElementById('git-data');
    const jsonText = textarea.value.trim();

    if (!jsonText) {
        alert('Please paste your git activity JSON data');
        return;
    }

    try {
        showLoading();
        const data = JSON.parse(jsonText);
        processData(data);
    } catch (error) {
        console.error('Failed to parse JSON:', error);
        alert('Invalid JSON data. Please check the format.');
        hideLoading();
    }
}

function showLoading() {
    document.getElementById('input-panel').classList.add('hidden');
    document.getElementById('loading').classList.remove('hidden');
}

function hideLoading() {
    document.getElementById('loading').classList.add('hidden');
}

function processData(data) {
    // Reset
    nodes = [];
    links = [];
    nodeMap.clear();

    const summary = data.summary;
    const repos = data.repositories || [];

    // Update stats panel
    updateStats(summary, repos);

    // Build graph from data
    buildGraph(summary, repos);

    // Create visualization
    setTimeout(() => {
        createVisualization();
        hideLoading();
        showUI();
    }, 100);
}

function updateStats(summary, repos) {
    document.getElementById('stat-repos').textContent = formatNumber(summary.total_repos);
    document.getElementById('stat-commits').textContent = formatNumber(summary.total_commits);
    document.getElementById('stat-lines').textContent = formatNumber(summary.total_lines_changed);
    document.getElementById('stat-langs').textContent = Object.keys(summary.languages || {}).length;
    document.getElementById('stat-types').textContent = Object.keys(summary.file_extensions || {}).length;
}

function formatNumber(num) {
    if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
    if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
    return num.toString();
}

function buildGraph(summary, repos) {
    const centerX = 0;
    const centerY = 0;
    const repoRadius = 300;
    const langRadius = 500;
    const contribRadius = 400;
    const extRadius = 600;

    // Add repository nodes in a circle
    repos.forEach((repo, i) => {
        const angle = (i / repos.length) * Math.PI * 2;
        const x = centerX + Math.cos(angle) * repoRadius;
        const y = centerY + Math.sin(angle) * repoRadius;

        const size = Math.max(15, Math.min(50, Math.sqrt(repo.total_commits) * 5));

        addNode({
            id: `repo:${repo.name}`,
            type: NODE_TYPES.REPO,
            label: repo.name,
            x,
            y,
            size,
            data: {
                commits: repo.total_commits,
                linesAdded: repo.total_lines_added,
                linesRemoved: repo.total_lines_removed,
                languages: repo.languages,
                types: repo.contribution_types
            }
        });
    });

    // Add language nodes
    const languages = Object.entries(summary.languages || {});
    languages.sort((a, b) => b[1] - a[1]);

    languages.slice(0, 15).forEach(([ lang, lines ], i) => {
        const angle = (i / Math.min(15, languages.length)) * Math.PI * 2 + 0.3;
        const x = centerX + Math.cos(angle) * langRadius;
        const y = centerY + Math.sin(angle) * langRadius;

        const size = Math.max(10, Math.min(40, Math.sqrt(lines / 100)));
        const pct = summary.language_percentages?.[lang] || 0;

        addNode({
            id: `lang:${lang}`,
            type: NODE_TYPES.LANGUAGE,
            label: lang,
            x,
            y,
            size,
            data: { lines, percentage: pct }
        });

        // Connect to repos that use this language
        repos.forEach(repo => {
            if (repo.languages && repo.languages[lang]) {
                const weight = repo.languages[lang] / lines;
                addLink(`repo:${repo.name}`, `lang:${lang}`, weight);
            }
        });
    });

    // Add contribution type nodes
    const contributions = Object.entries(summary.contribution_types || {});
    contributions.sort((a, b) => b[1] - a[1]);

    contributions.forEach(([ type, lines ], i) => {
        const angle = (i / contributions.length) * Math.PI * 2 + 0.7;
        const x = centerX + Math.cos(angle) * contribRadius;
        const y = centerY + Math.sin(angle) * contribRadius;

        const size = Math.max(12, Math.min(35, Math.sqrt(lines / 50)));
        const pct = summary.contribution_percentages?.[type] || 0;

        const label = formatContributionType(type);

        addNode({
            id: `contrib:${type}`,
            type: NODE_TYPES.CONTRIBUTION,
            label,
            x,
            y,
            size,
            data: { lines, percentage: pct, rawType: type }
        });

        // Connect to repos
        repos.forEach(repo => {
            if (repo.contribution_types && repo.contribution_types[type]) {
                const weight = repo.contribution_types[type] / lines;
                addLink(`repo:${repo.name}`, `contrib:${type}`, weight);
            }
        });
    });

    // Add file extension nodes (top 20)
    const extensions = Object.entries(summary.file_extensions || {});
    extensions.sort((a, b) => b[1] - a[1]);

    extensions.slice(0, 20).forEach(([ ext, lines ], i) => {
        const angle = (i / Math.min(20, extensions.length)) * Math.PI * 2 + 1.2;
        const x = centerX + Math.cos(angle) * extRadius;
        const y = centerY + Math.sin(angle) * extRadius;

        const size = Math.max(8, Math.min(30, Math.sqrt(lines / 100)));
        const pct = summary.file_extension_percentages?.[ext] || 0;

        addNode({
            id: `ext:${ext}`,
            type: NODE_TYPES.EXTENSION,
            label: ext,
            x,
            y,
            size,
            data: { lines, percentage: pct }
        });

        // Connect to repos
        repos.forEach(repo => {
            if (repo.file_extensions && repo.file_extensions[ext]) {
                const weight = repo.file_extensions[ext] / lines;
                addLink(`repo:${repo.name}`, `ext:${ext}`, weight * 0.5);
            }
        });
    });
}

function addNode(node) {
    nodes.push(node);
    nodeMap.set(node.id, node);
}

function addLink(sourceId, targetId, weight = 1) {
    if (nodeMap.has(sourceId) && nodeMap.has(targetId)) {
        links.push({
            source: sourceId,
            target: targetId,
            weight: Math.min(1, weight)
        });
    }
}

function formatContributionType(type) {
    const labels = {
        'production_code': 'Production Code',
        'tests': 'Tests',
        'documentation': 'Documentation',
        'specs_config': 'Config & Specs',
        'infrastructure': 'Infrastructure',
        'styling': 'Styling',
        'other': 'Other'
    };
    return labels[type] || type;
}

function createVisualization() {
    const canvas = document.getElementById('cosmos-canvas');
    const container = document.getElementById('cosmos-container');

    // Prepare data for Cosmos
    const cosmosNodes = nodes.map(n => ({
        id: n.id,
        x: n.x,
        y: n.y,
        size: n.size,
        color: getNodeColor(n.type)
    }));

    const cosmosLinks = links.map(l => ({
        source: l.source,
        target: l.target,
        width: Math.max(0.5, l.weight * 2),
        color: COLORS.edge
    }));

    // Initialize Cosmos
    try {
        if (typeof Cosmos !== 'undefined') {
            cosmos = new Cosmos.Graph(canvas, {
                backgroundColor: '#0a0a0f',
                nodeColor: (n) => n.color,
                nodeSize: (n) => n.size,
                linkColor: (l) => l.color || COLORS.edge,
                linkWidth: (l) => l.width || 1,
                simulation: {
                    linkDistance: 100,
                    linkStrength: 0.1,
                    gravity: 0.1,
                    repulsion: 1,
                    friction: 0.9
                },
                events: {
                    onNodeMouseOver: handleNodeHover,
                    onNodeMouseOut: handleNodeOut
                }
            });

            cosmos.setData(cosmosNodes, cosmosLinks);
            cosmos.zoom(0.8);
        } else {
            // Fallback to canvas rendering if Cosmos not loaded
            renderCanvasFallback(canvas, container);
        }
    } catch (error) {
        console.error('Cosmos initialization failed:', error);
        renderCanvasFallback(canvas, container);
    }
}

function renderCanvasFallback(canvas, container) {
    // Simple canvas-based visualization fallback
    const ctx = canvas.getContext('2d');
    canvas.width = container.clientWidth;
    canvas.height = container.clientHeight;

    const centerX = canvas.width / 2;
    const centerY = canvas.height / 2;
    const scale = Math.min(canvas.width, canvas.height) / 1400;

    // Animation
    let time = 0;

    function animate() {
        time += 0.005;
        ctx.fillStyle = '#0a0a0f';
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        // Draw links
        ctx.strokeStyle = COLORS.edge;
        ctx.lineWidth = 0.5;
        ctx.globalAlpha = 0.3;

        links.forEach(link => {
            const source = nodeMap.get(link.source);
            const target = nodeMap.get(link.target);
            if (source && target) {
                ctx.beginPath();
                ctx.moveTo(centerX + source.x * scale, centerY + source.y * scale);
                ctx.lineTo(centerX + target.x * scale, centerY + target.y * scale);
                ctx.stroke();
            }
        });

        ctx.globalAlpha = 1;

        // Draw nodes with glow and animation
        nodes.forEach((node, i) => {
            const x = centerX + node.x * scale;
            const y = centerY + node.y * scale;
            const pulse = 1 + Math.sin(time * 2 + i * 0.5) * 0.1;
            const size = node.size * scale * pulse;

            // Glow
            const gradient = ctx.createRadialGradient(x, y, 0, x, y, size * 2);
            const color = getNodeColor(node.type);
            gradient.addColorStop(0, color);
            gradient.addColorStop(0.5, color + '44');
            gradient.addColorStop(1, 'transparent');

            ctx.fillStyle = gradient;
            ctx.beginPath();
            ctx.arc(x, y, size * 2, 0, Math.PI * 2);
            ctx.fill();

            // Node
            ctx.fillStyle = color;
            ctx.beginPath();
            ctx.arc(x, y, size, 0, Math.PI * 2);
            ctx.fill();

            // Label for larger nodes
            if (node.size > 15) {
                ctx.fillStyle = '#ffffff';
                ctx.font = `${Math.max(10, size * 0.6)}px "SF Mono", monospace`;
                ctx.textAlign = 'center';
                ctx.textBaseline = 'middle';
                ctx.fillText(node.label, x, y + size + 15);
            }
        });

        requestAnimationFrame(animate);
    }

    animate();

    // Add mouse interaction
    canvas.addEventListener('mousemove', (e) => {
        const rect = canvas.getBoundingClientRect();
        const mouseX = e.clientX - rect.left;
        const mouseY = e.clientY - rect.top;

        let hoveredNode = null;

        nodes.forEach(node => {
            const x = centerX + node.x * scale;
            const y = centerY + node.y * scale;
            const dist = Math.sqrt((mouseX - x) ** 2 + (mouseY - y) ** 2);

            if (dist < node.size * scale + 10) {
                hoveredNode = node;
            }
        });

        if (hoveredNode) {
            handleNodeHover(hoveredNode);
        } else {
            handleNodeOut();
        }
    });
}

function getNodeColor(type) {
    switch (type) {
        case NODE_TYPES.REPO: return COLORS.repo;
        case NODE_TYPES.LANGUAGE: return COLORS.language;
        case NODE_TYPES.CONTRIBUTION: return COLORS.contribution;
        case NODE_TYPES.EXTENSION: return COLORS.extension;
        default: return '#ffffff';
    }
}

function handleNodeHover(node) {
    const hoverInfo = document.getElementById('hover-info');
    const nameEl = document.getElementById('hover-name');
    const detailsEl = document.getElementById('hover-details');

    nameEl.textContent = node.label;

    let details = '';
    const data = node.data || {};

    switch (node.type) {
        case NODE_TYPES.REPO:
            details = `
                <div class="stat-row"><span class="stat-label">Commits</span><span class="stat-value">${data.commits || 0}</span></div>
                <div class="stat-row"><span class="stat-label">Lines +</span><span class="stat-value">${formatNumber(data.linesAdded || 0)}</span></div>
                <div class="stat-row"><span class="stat-label">Lines -</span><span class="stat-value">${formatNumber(data.linesRemoved || 0)}</span></div>
            `;
            break;
        case NODE_TYPES.LANGUAGE:
        case NODE_TYPES.CONTRIBUTION:
        case NODE_TYPES.EXTENSION:
            details = `
                <div class="stat-row"><span class="stat-label">Lines</span><span class="stat-value">${formatNumber(data.lines || 0)}</span></div>
                <div class="stat-row"><span class="stat-label">Share</span><span class="stat-value">${(data.percentage || 0).toFixed(1)}%</span></div>
            `;
            break;
    }

    detailsEl.innerHTML = details;
    hoverInfo.classList.add('visible');
}

function handleNodeOut() {
    document.getElementById('hover-info').classList.remove('visible');
}

function showUI() {
    document.getElementById('stats-panel').classList.remove('hidden');
    document.getElementById('legend').classList.remove('hidden');
}

// Export for embedding
window.GitConstellation = {
    loadFromJson: (jsonData) => {
        processData(typeof jsonData === 'string' ? JSON.parse(jsonData) : jsonData);
    },
    loadFromUrl: fetchAndLoad
};
