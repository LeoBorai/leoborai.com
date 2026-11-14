// Client-side enhancements. The site is fully readable without this script.
(() => {
	const root = document.documentElement;
	const $ = (sel, scope = document) => scope.querySelector(sel);
	const $$ = (sel, scope = document) => [...scope.querySelectorAll(sel)];

	// ── theme toggle (initial class is set by the inline script in <head>) ──
	const syncThemeLabels = () => {
		const dark = root.classList.contains('dark');
		$$('[data-theme-toggle]').forEach((btn) =>
			btn.setAttribute('aria-label', dark ? 'Switch to light theme' : 'Switch to dark theme')
		);
	};
	$$('[data-theme-toggle]').forEach((btn) =>
		btn.addEventListener('click', () => {
			const dark = root.classList.toggle('dark');
			try {
				localStorage.setItem('theme', dark ? 'dark' : 'light');
			} catch {
				/* storage unavailable */
			}
			syncThemeLabels();
		})
	);
	syncThemeLabels();

	// ── mobile menu ──
	const menuBtn = $('#menu-toggle');
	const menu = $('#mobile-menu');
	if (menuBtn && menu) {
		const setMenu = (open) => {
			menu.classList.toggle('hidden', !open);
			menu.classList.toggle('flex', open);
			menuBtn.setAttribute('aria-expanded', String(open));
			menuBtn.setAttribute('aria-label', open ? 'Close menu' : 'Open menu');
			$('[data-menu-open]', menuBtn).toggleAttribute('hidden', open);
			$('[data-menu-close]', menuBtn).toggleAttribute('hidden', !open);
		};
		menuBtn.addEventListener('click', () => setMenu(menu.classList.contains('hidden')));
		$$('a', menu).forEach((a) => a.addEventListener('click', () => setMenu(false)));
	}

	// ── local clock ──
	const clock = $('#clock');
	if (clock) {
		const format = new Intl.DateTimeFormat('en-GB', {
			timeZone: 'Europe/Berlin',
			hour: '2-digit',
			minute: '2-digit',
			second: '2-digit',
			hourCycle: 'h23',
			timeZoneName: 'short'
		});
		const tick = () => {
			const parts = format.formatToParts(new Date());
			const pick = (type) => parts.find((p) => p.type === type)?.value ?? '';
			clock.textContent = `${pick('hour')}:${pick('minute')}:${pick('second')} ${pick('timeZoneName')}`;
		};
		tick();
		setInterval(tick, 1000);
	}

	// ── "Copy as MD" ──
	$$('[data-copy-source]').forEach((btn) => {
		const source = document.getElementById(btn.dataset.copySource);
		const label = $('[data-copy-label]', btn);
		btn.addEventListener('click', async () => {
			try {
				await navigator.clipboard.writeText(source.value);
				label.textContent = 'Copied';
				setTimeout(() => (label.textContent = 'Copy as MD'), 2000);
			} catch {
				/* clipboard unavailable */
			}
		});
	});

	// ── canvas effects ──
	const reduced = matchMedia('(prefers-reduced-motion: reduce)').matches;

	// Glyphs that light up under the pointer.
	(() => {
		const canvas = $('#pixel-field');
		const ctx = canvas?.getContext('2d');
		if (!ctx || reduced) return;

		const CELL = 14; // cell edge in CSS px; one monospace glyph per cell
		const RADIUS = 5; // pointer influence radius in cells
		const GLYPHS = '01{}[]<>()/\\|=+-*#$%&;:~^?!_';
		const DECAY = 0.93;
		const randomGlyph = () => GLYPHS[Math.floor(Math.random() * GLYPHS.length)];

		let cols = 0, rows = 0, width = 0, height = 0, frame = 0;
		let heat = new Float32Array(0);
		let glyphs = [];
		let font = '12px monospace';
		let color = '157 111 212';

		const resize = () => {
			const dpr = window.devicePixelRatio || 1;
			width = window.innerWidth;
			height = window.innerHeight * 0.85;
			cols = Math.ceil(width / CELL);
			rows = Math.ceil(height / CELL);
			canvas.width = Math.ceil(width * dpr);
			canvas.height = Math.ceil(height * dpr);
			ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
			heat = new Float32Array(cols * rows);
			glyphs = Array.from({ length: cols * rows }, randomGlyph);
			const styles = getComputedStyle(canvas);
			font = `${CELL - 2}px ${styles.getPropertyValue('--font-mono').trim() || 'monospace'}`;
			color = (styles.getPropertyValue('--color-accent').trim() || '#9d6fd4')
				.replace(/^#/, '')
				.replace(/(..)(..)(..)/, (_, r, g, b) => [r, g, b].map((h) => parseInt(h, 16)).join(' '));
		};

		const draw = () => {
			ctx.clearRect(0, 0, width, height);
			ctx.font = font;
			ctx.textAlign = 'center';
			ctx.textBaseline = 'middle';
			let alive = false;

			for (let i = 0; i < heat.length; i++) {
				const v = heat[i];
				if (v < 0.02) {
					heat[i] = 0;
					continue;
				}
				alive = true;
				if (Math.random() < 0.12) glyphs[i] = randomGlyph();
				ctx.fillStyle = `rgb(${color} / ${Math.min(1, v * 0.9)})`;
				ctx.fillText(glyphs[i], (i % cols) * CELL + CELL / 2, Math.floor(i / cols) * CELL + CELL / 2);
				heat[i] = v * DECAY;
			}

			frame = alive ? requestAnimationFrame(draw) : 0;
		};

		const onMove = (e) => {
			const cx = Math.floor(e.clientX / CELL);
			const cy = Math.floor(e.clientY / CELL);

			for (let dy = -RADIUS; dy <= RADIUS; dy++) {
				for (let dx = -RADIUS; dx <= RADIUS; dx++) {
					const x = cx + dx;
					const y = cy + dy;
					if (x < 0 || y < 0 || x >= cols || y >= rows) continue;
					const falloff = 1 - Math.hypot(dx, dy) / RADIUS;
					if (falloff <= 0) continue;
					const i = y * cols + x;
					heat[i] = Math.max(heat[i], falloff * (0.4 + Math.random() * 0.6));
				}
			}

			if (!frame) frame = requestAnimationFrame(draw);
		};

		resize();
		window.addEventListener('resize', resize);
		window.addEventListener('pointermove', onMove, { passive: true });
	})();

	// Mesh + gold glow on the side rails (xl screens and up).
	(() => {
		const canvas = $('#rail-glow');
		const ctx = canvas?.getContext('2d');
		if (!ctx) return;

		const SIZE = 28; // must match the 56px tile (2x2 cells) of `.grid-background`
		const CONTENT_HALF = 640; // content half-width (max-w-7xl = 80rem)
		const MIN_WIDTH = 1280; // rails only exist from the `xl` breakpoint up
		const RADIUS = 110;
		const DECAY = 0.93;
		const GOLD = '255, 170, 30';

		let width = 0, height = 0, rows = 0, frame = 0;
		let rails = [];

		const triangle = (ox, col, row, k) => {
			const x = ox + col * SIZE;
			const y = row * SIZE;
			const a = [x, y], b = [x + SIZE, y], c = [x + SIZE, y + SIZE], d = [x, y + SIZE];
			if ((col + row) % 2 === 0) return k === 0 ? [a, b, c] : [a, c, d];
			return k === 0 ? [a, b, d] : [b, c, d];
		};

		const centroid = (pts) => [
			(pts[0][0] + pts[1][0] + pts[2][0]) / 3,
			(pts[0][1] + pts[1][1] + pts[2][1]) / 3
		];

		const resize = () => {
			const dpr = window.devicePixelRatio || 1;
			width = window.innerWidth;
			height = window.innerHeight;
			canvas.width = Math.ceil(width * dpr);
			canvas.height = Math.ceil(height * dpr);
			ctx.setTransform(dpr, 0, 0, dpr, 0, 0);

			rows = Math.ceil(height / SIZE);
			const railWidth = Math.max(0, width / 2 - CONTENT_HALF + 1);
			const cols = Math.ceil(railWidth / SIZE);
			const make = (ox) => ({ ox, width: railWidth, cols, heat: new Float32Array(cols * rows * 2) });
			// right rail starts after its 1px left border
			rails = width >= MIN_WIDTH ? [make(0), make(width - railWidth + 1)] : [];
			draw();
		};

		const drawMesh = (rail) => {
			const { ox, cols } = rail;
			const dark = root.classList.contains('dark');
			ctx.beginPath();
			for (let col = 0; col <= cols; col++) {
				ctx.moveTo(ox + col * SIZE + 0.5, 0);
				ctx.lineTo(ox + col * SIZE + 0.5, rows * SIZE);
			}
			for (let row = 0; row <= rows; row++) {
				ctx.moveTo(ox, row * SIZE + 0.5);
				ctx.lineTo(ox + cols * SIZE, row * SIZE + 0.5);
			}
			for (let row = 0; row < rows; row++) {
				for (let col = 0; col < cols; col++) {
					const x = ox + col * SIZE + 0.5;
					const y = row * SIZE + 0.5;
					if ((col + row) % 2 === 0) {
						ctx.moveTo(x, y);
						ctx.lineTo(x + SIZE, y + SIZE);
					} else {
						ctx.moveTo(x + SIZE, y);
						ctx.lineTo(x, y + SIZE);
					}
				}
			}
			ctx.strokeStyle = dark ? 'rgba(255, 255, 255, 0.06)' : 'rgba(0, 0, 0, 0.07)';
			ctx.lineWidth = 1;
			ctx.stroke();
		};

		const draw = () => {
			ctx.clearRect(0, 0, width, height);
			let alive = false;

			for (const rail of rails) {
				const { heat, cols, ox, width: railWidth } = rail;
				ctx.save();
				ctx.beginPath();
				ctx.rect(ox, 0, railWidth, height);
				ctx.clip();
				drawMesh(rail);
				for (let i = 0; i < heat.length; i++) {
					const v = heat[i];
					if (v < 0.02) {
						heat[i] = 0;
						continue;
					}
					alive = true;

					const cell = i >> 1;
					const pts = triangle(ox, cell % cols, Math.floor(cell / cols), i & 1);
					const [cx] = centroid(pts);
					if (cx < ox || cx > ox + railWidth) {
						heat[i] = 0;
						continue;
					}

					ctx.beginPath();
					pts.forEach(([px, py], n) => {
						if (n === 0) ctx.moveTo(px + 0.5, py + 0.5);
						else ctx.lineTo(px + 0.5, py + 0.5);
					});
					ctx.closePath();
					ctx.strokeStyle = `rgba(${GOLD}, ${Math.min(1, v * 1.2)})`;
					ctx.lineWidth = 1.5;
					ctx.lineJoin = 'round';
					ctx.stroke();
					heat[i] = v * DECAY;
				}
				ctx.restore();
			}

			frame = alive ? requestAnimationFrame(draw) : 0;
		};

		const onMove = (e) => {
			for (const rail of rails) {
				const { heat, cols, ox } = rail;
				if (e.clientX < ox - RADIUS || e.clientX > ox + rail.width + RADIUS) continue;

				const c0 = Math.max(0, Math.floor((e.clientX - RADIUS - ox) / SIZE));
				const c1 = Math.min(cols - 1, Math.floor((e.clientX + RADIUS - ox) / SIZE));
				const r0 = Math.max(0, Math.floor((e.clientY - RADIUS) / SIZE));
				const r1 = Math.min(rows - 1, Math.floor((e.clientY + RADIUS) / SIZE));

				for (let row = r0; row <= r1; row++) {
					for (let col = c0; col <= c1; col++) {
						for (const k of [0, 1]) {
							const [cx, cy] = centroid(triangle(ox, col, row, k));
							const falloff = 1 - Math.hypot(cx - e.clientX, cy - e.clientY) / RADIUS;
							if (falloff <= 0) continue;
							const i = (row * cols + col) * 2 + k;
							heat[i] = Math.max(heat[i], falloff * (0.5 + Math.random() * 0.5));
						}
					}
				}
			}

			if (!frame) frame = requestAnimationFrame(draw);
		};

		resize();
		window.addEventListener('resize', resize);
		if (!reduced) window.addEventListener('pointermove', onMove, { passive: true });

		// redraw the mesh when the theme class toggles
		new MutationObserver(() => draw()).observe(root, {
			attributes: true,
			attributeFilter: ['class']
		});
	})();
})();
