/**
 * Git Activity Constellation - Embeddable Canvas Widget
 * Drop this script into any page and call GitConstellation.init()
 */
(function() {
    'use strict';

    const COLORS = {
        bg: '#0a0a0f',
        repo: '#f472b6',
        language: '#7dd3fc',
        contribution: '#a78bfa',
        extension: '#34d399',
        text: '#e0e0e0',
        textDim: '#666666',
        line: '#333333'
    };

    class GitConstellation {
        constructor(container, options = {}) {
            this.container = typeof container === 'string'
                ? document.querySelector(container)
                : container;

            this.options = {
                width: options.width || this.container.clientWidth || 800,
                height: options.height || this.container.clientHeight || 600,
                animate: options.animate !== false,
                showLabels: options.showLabels !== false,
                showStats: options.showStats !== false,
                theme: options.theme || 'dark',
                ...options
            };

            this.nodes = [];
            this.links = [];
            this.nodeMap = new Map();
            this.hoveredNode = null;
            this.time = 0;
            this.data = null;

            this.init();
        }

        init() {
            // Create canvas
            this.canvas = document.createElement('canvas');
            this.canvas.width = this.options.width;
            this.canvas.height = this.options.height;
            this.canvas.style.borderRadius = '12px';
            this.canvas.style.cursor = 'crosshair';
            this.ctx = this.canvas.getContext('2d');

            this.container.appendChild(this.canvas);

            // Mouse events
            this.canvas.addEventListener('mousemove', this.handleMouseMove.bind(this));
            this.canvas.addEventListener('mouseleave', () => { this.hoveredNode = null; });

            // Start animation loop
            if (this.options.animate) {
                this.animate();
            }
        }

        async loadFromUrl(url) {
            try {
                const response = await fetch(url);
                const data = await response.json();
                this.loadData(data);
            } catch (error) {
                console.error('Failed to load data:', error);
            }
        }

        loadData(data) {
            this.data = data;
            this.nodes = [];
            this.links = [];
            this.nodeMap.clear();

            this.buildGraph(data);

            if (!this.options.animate) {
                this.render();
            }
        }

        buildGraph(data) {
            const summary = data.summary;
            const repos = data.repositories || [];

            const cx = this.options.width / 2;
            const cy = this.options.height / 2;
            const scale = Math.min(this.options.width, this.options.height) / 800;

            // Repo nodes - center cluster
            repos.forEach((repo, i) => {
                const angle = (i / repos.length) * Math.PI * 2;
                const radius = 80 * scale;
                this.addNode({
                    id: `repo:${repo.name}`,
                    type: 'repo',
                    label: repo.name,
                    x: cx + Math.cos(angle) * radius,
                    y: cy + Math.sin(angle) * radius,
                    size: Math.max(8, Math.min(25, Math.sqrt(repo.total_commits) * 3)) * scale,
                    data: repo
                });
            });

            // Language nodes - outer ring
            const languages = Object.entries(summary.languages || {}).sort((a, b) => b[1] - a[1]);
            languages.slice(0, 12).forEach(([lang, lines], i) => {
                const angle = (i / Math.min(12, languages.length)) * Math.PI * 2 + 0.2;
                const radius = 200 * scale;
                const pct = summary.language_percentages?.[lang] || 0;

                this.addNode({
                    id: `lang:${lang}`,
                    type: 'language',
                    label: lang,
                    x: cx + Math.cos(angle) * radius,
                    y: cy + Math.sin(angle) * radius,
                    size: Math.max(6, Math.min(20, Math.sqrt(pct) * 3)) * scale,
                    data: { lines, percentage: pct }
                });

                // Connect to repos
                repos.forEach(repo => {
                    if (repo.languages?.[lang]) {
                        this.addLink(`repo:${repo.name}`, `lang:${lang}`, 0.5);
                    }
                });
            });

            // Contribution type nodes
            const contributions = Object.entries(summary.contribution_types || {}).sort((a, b) => b[1] - a[1]);
            contributions.forEach(([type, lines], i) => {
                const angle = (i / contributions.length) * Math.PI * 2 + 0.8;
                const radius = 140 * scale;
                const pct = summary.contribution_percentages?.[type] || 0;

                this.addNode({
                    id: `contrib:${type}`,
                    type: 'contribution',
                    label: this.formatContribType(type),
                    x: cx + Math.cos(angle) * radius,
                    y: cy + Math.sin(angle) * radius,
                    size: Math.max(6, Math.min(18, Math.sqrt(pct) * 2.5)) * scale,
                    data: { lines, percentage: pct }
                });

                repos.forEach(repo => {
                    if (repo.contribution_types?.[type]) {
                        this.addLink(`repo:${repo.name}`, `contrib:${type}`, 0.3);
                    }
                });
            });

            // File extension nodes - outermost
            const extensions = Object.entries(summary.file_extensions || {}).sort((a, b) => b[1] - a[1]);
            extensions.slice(0, 16).forEach(([ext, lines], i) => {
                const angle = (i / Math.min(16, extensions.length)) * Math.PI * 2 + 1.5;
                const radius = 260 * scale;
                const pct = summary.file_extension_percentages?.[ext] || 0;

                this.addNode({
                    id: `ext:${ext}`,
                    type: 'extension',
                    label: ext,
                    x: cx + Math.cos(angle) * radius,
                    y: cy + Math.sin(angle) * radius,
                    size: Math.max(4, Math.min(14, Math.sqrt(pct) * 2)) * scale,
                    data: { lines, percentage: pct }
                });

                repos.forEach(repo => {
                    if (repo.file_extensions?.[ext]) {
                        this.addLink(`repo:${repo.name}`, `ext:${ext}`, 0.2);
                    }
                });
            });
        }

        addNode(node) {
            // Add some random offset for organic feel
            node.offsetX = (Math.random() - 0.5) * 20;
            node.offsetY = (Math.random() - 0.5) * 20;
            node.phase = Math.random() * Math.PI * 2;
            this.nodes.push(node);
            this.nodeMap.set(node.id, node);
        }

        addLink(sourceId, targetId, strength = 1) {
            if (this.nodeMap.has(sourceId) && this.nodeMap.has(targetId)) {
                this.links.push({ source: sourceId, target: targetId, strength });
            }
        }

        formatContribType(type) {
            const labels = {
                'production_code': 'Code',
                'tests': 'Tests',
                'documentation': 'Docs',
                'specs_config': 'Config',
                'infrastructure': 'Infra',
                'styling': 'Style',
                'other': 'Other'
            };
            return labels[type] || type;
        }

        handleMouseMove(e) {
            const rect = this.canvas.getBoundingClientRect();
            const x = e.clientX - rect.left;
            const y = e.clientY - rect.top;

            this.hoveredNode = null;

            for (const node of this.nodes) {
                const nx = node.x + Math.sin(this.time + node.phase) * 2;
                const ny = node.y + Math.cos(this.time + node.phase) * 2;
                const dist = Math.sqrt((x - nx) ** 2 + (y - ny) ** 2);

                if (dist < node.size + 8) {
                    this.hoveredNode = node;
                    break;
                }
            }
        }

        animate() {
            this.time += 0.015;
            this.render();
            requestAnimationFrame(() => this.animate());
        }

        render() {
            const ctx = this.ctx;
            const w = this.options.width;
            const h = this.options.height;

            // Clear with gradient background
            const bgGrad = ctx.createRadialGradient(w/2, h/2, 0, w/2, h/2, w/2);
            bgGrad.addColorStop(0, '#12121a');
            bgGrad.addColorStop(1, COLORS.bg);
            ctx.fillStyle = bgGrad;
            ctx.fillRect(0, 0, w, h);

            // Draw links
            ctx.globalAlpha = 0.15;
            this.links.forEach(link => {
                const source = this.nodeMap.get(link.source);
                const target = this.nodeMap.get(link.target);
                if (!source || !target) return;

                const isHighlighted = this.hoveredNode &&
                    (this.hoveredNode.id === link.source || this.hoveredNode.id === link.target);

                ctx.strokeStyle = isHighlighted ? '#555' : COLORS.line;
                ctx.lineWidth = isHighlighted ? 1.5 : 0.5;
                ctx.globalAlpha = isHighlighted ? 0.6 : 0.15;

                const sx = source.x + Math.sin(this.time + source.phase) * 2;
                const sy = source.y + Math.cos(this.time + source.phase) * 2;
                const tx = target.x + Math.sin(this.time + target.phase) * 2;
                const ty = target.y + Math.cos(this.time + target.phase) * 2;

                ctx.beginPath();
                ctx.moveTo(sx, sy);
                ctx.lineTo(tx, ty);
                ctx.stroke();
            });

            ctx.globalAlpha = 1;

            // Draw nodes
            this.nodes.forEach(node => {
                const x = node.x + Math.sin(this.time + node.phase) * 2;
                const y = node.y + Math.cos(this.time + node.phase) * 2;
                const isHovered = this.hoveredNode === node;
                const pulse = 1 + Math.sin(this.time * 2 + node.phase) * 0.08;
                const size = node.size * (isHovered ? 1.3 : pulse);

                // Glow
                const glow = ctx.createRadialGradient(x, y, 0, x, y, size * 3);
                const color = this.getColor(node.type);
                glow.addColorStop(0, color + '40');
                glow.addColorStop(0.5, color + '15');
                glow.addColorStop(1, 'transparent');
                ctx.fillStyle = glow;
                ctx.beginPath();
                ctx.arc(x, y, size * 3, 0, Math.PI * 2);
                ctx.fill();

                // Node circle
                const nodeGrad = ctx.createRadialGradient(x - size/3, y - size/3, 0, x, y, size);
                nodeGrad.addColorStop(0, this.lighten(color, 30));
                nodeGrad.addColorStop(1, color);
                ctx.fillStyle = nodeGrad;
                ctx.beginPath();
                ctx.arc(x, y, size, 0, Math.PI * 2);
                ctx.fill();

                // Label
                if (this.options.showLabels && (node.size > 8 || isHovered)) {
                    ctx.fillStyle = isHovered ? '#fff' : COLORS.text;
                    ctx.font = `${isHovered ? 'bold ' : ''}${Math.max(9, size * 0.8)}px "SF Mono", "Fira Code", monospace`;
                    ctx.textAlign = 'center';
                    ctx.textBaseline = 'top';
                    ctx.fillText(node.label, x, y + size + 4);
                }
            });

            // Draw stats panel
            if (this.options.showStats && this.data) {
                this.renderStats();
            }

            // Draw hover tooltip
            if (this.hoveredNode) {
                this.renderTooltip();
            }
        }

        renderStats() {
            const ctx = this.ctx;
            const summary = this.data.summary;
            const padding = 15;
            const panelWidth = 160;
            const panelHeight = 130;

            // Panel background
            ctx.fillStyle = 'rgba(10, 10, 20, 0.85)';
            ctx.strokeStyle = 'rgba(255, 255, 255, 0.1)';
            ctx.lineWidth = 1;
            this.roundRect(padding, padding, panelWidth, panelHeight, 10);
            ctx.fill();
            ctx.stroke();

            // Title
            ctx.fillStyle = COLORS.language;
            ctx.font = 'bold 12px "SF Mono", monospace';
            ctx.textAlign = 'left';
            ctx.textBaseline = 'top';
            ctx.fillText('Git Constellation', padding + 12, padding + 12);

            // Stats
            ctx.font = '10px "SF Mono", monospace';
            const stats = [
                ['Repos', summary.total_repos],
                ['Commits', this.formatNum(summary.total_commits)],
                ['Lines', this.formatNum(summary.total_lines_changed)],
                ['Languages', Object.keys(summary.languages || {}).length]
            ];

            stats.forEach(([label, value], i) => {
                const y = padding + 35 + i * 22;
                ctx.fillStyle = COLORS.textDim;
                ctx.fillText(label, padding + 12, y);
                ctx.fillStyle = '#fff';
                ctx.textAlign = 'right';
                ctx.fillText(String(value), padding + panelWidth - 12, y);
                ctx.textAlign = 'left';
            });
        }

        renderTooltip() {
            const ctx = this.ctx;
            const node = this.hoveredNode;
            const x = node.x + Math.sin(this.time + node.phase) * 2;
            const y = node.y + Math.cos(this.time + node.phase) * 2;

            const padding = 10;
            const tipWidth = 140;
            const tipHeight = 70;
            let tipX = x + node.size + 10;
            let tipY = y - tipHeight / 2;

            // Keep tooltip on screen
            if (tipX + tipWidth > this.options.width - 10) {
                tipX = x - node.size - tipWidth - 10;
            }
            if (tipY < 10) tipY = 10;
            if (tipY + tipHeight > this.options.height - 10) {
                tipY = this.options.height - tipHeight - 10;
            }

            // Background
            ctx.fillStyle = 'rgba(0, 0, 0, 0.9)';
            ctx.strokeStyle = this.getColor(node.type);
            ctx.lineWidth = 2;
            this.roundRect(tipX, tipY, tipWidth, tipHeight, 8);
            ctx.fill();
            ctx.stroke();

            // Content
            ctx.fillStyle = '#fff';
            ctx.font = 'bold 11px "SF Mono", monospace';
            ctx.textAlign = 'left';
            ctx.textBaseline = 'top';
            ctx.fillText(node.label, tipX + padding, tipY + padding);

            ctx.font = '10px "SF Mono", monospace';
            ctx.fillStyle = COLORS.textDim;

            if (node.type === 'repo' && node.data) {
                ctx.fillText(`${node.data.total_commits} commits`, tipX + padding, tipY + 30);
                ctx.fillText(`+${this.formatNum(node.data.total_lines_added)} / -${this.formatNum(node.data.total_lines_removed)}`, tipX + padding, tipY + 45);
            } else if (node.data) {
                ctx.fillText(`${this.formatNum(node.data.lines)} lines`, tipX + padding, tipY + 30);
                ctx.fillText(`${node.data.percentage?.toFixed(1) || 0}%`, tipX + padding, tipY + 45);
            }
        }

        roundRect(x, y, w, h, r) {
            this.ctx.beginPath();
            this.ctx.moveTo(x + r, y);
            this.ctx.lineTo(x + w - r, y);
            this.ctx.quadraticCurveTo(x + w, y, x + w, y + r);
            this.ctx.lineTo(x + w, y + h - r);
            this.ctx.quadraticCurveTo(x + w, y + h, x + w - r, y + h);
            this.ctx.lineTo(x + r, y + h);
            this.ctx.quadraticCurveTo(x, y + h, x, y + h - r);
            this.ctx.lineTo(x, y + r);
            this.ctx.quadraticCurveTo(x, y, x + r, y);
            this.ctx.closePath();
        }

        getColor(type) {
            return COLORS[type] || '#ffffff';
        }

        lighten(hex, percent) {
            const num = parseInt(hex.replace('#', ''), 16);
            const amt = Math.round(2.55 * percent);
            const R = Math.min(255, (num >> 16) + amt);
            const G = Math.min(255, ((num >> 8) & 0x00FF) + amt);
            const B = Math.min(255, (num & 0x0000FF) + amt);
            return `#${(0x1000000 + R * 0x10000 + G * 0x100 + B).toString(16).slice(1)}`;
        }

        formatNum(n) {
            if (n >= 1000000) return (n / 1000000).toFixed(1) + 'M';
            if (n >= 1000) return (n / 1000).toFixed(1) + 'K';
            return String(n);
        }

        resize(width, height) {
            this.options.width = width;
            this.options.height = height;
            this.canvas.width = width;
            this.canvas.height = height;
            if (this.data) {
                this.loadData(this.data);
            }
        }

        destroy() {
            this.canvas.remove();
        }
    }

    // Export
    window.GitConstellation = {
        create: (container, options) => new GitConstellation(container, options),

        // Quick init with data URL
        init: async (container, dataUrl, options = {}) => {
            const instance = new GitConstellation(container, options);
            if (dataUrl) {
                await instance.loadFromUrl(dataUrl);
            }
            return instance;
        }
    };
})();
