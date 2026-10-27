<script lang="ts">
	import { onMount } from 'svelte';

	/** Must match the 56px tile (2x2 cells) of `.grid-background` in app.css. */
	const SIZE = 28;
	/** Content half-width (max-w-7xl = 80rem) the rails sit outside of. */
	const CONTENT_HALF = 640;
	/** Rails only exist from the `xl` breakpoint up. */
	const MIN_WIDTH = 1280;
	const RADIUS = 110;
	const DECAY = 0.93;

	type Point = [number, number];
	/** Glow color (gold) */
	const GOLD = '255, 170, 30';

	type Rail = { ox: number; width: number; cols: number; heat: Float32Array };

	let canvas: HTMLCanvasElement;

	onMount(() => {
		const reduced = matchMedia('(prefers-reduced-motion: reduce)').matches;

		const ctx = canvas.getContext('2d');
		if (!ctx) return;

		let width = 0;
		let height = 0;
		let rows = 0;
		let rails: Rail[] = [];
		let frame = 0;

		const triangle = (ox: number, col: number, row: number, k: 0 | 1): Point[] => {
			const x = ox + col * SIZE;
			const y = row * SIZE;
			const a: Point = [x, y];
			const b: Point = [x + SIZE, y];
			const c: Point = [x + SIZE, y + SIZE];
			const d: Point = [x, y + SIZE];
			if ((col + row) % 2 === 0) return k === 0 ? [a, b, c] : [a, c, d];
			return k === 0 ? [a, b, d] : [b, c, d];
		};

		const centroid = (pts: Point[]): Point => [
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
			const make = (ox: number): Rail => ({
				ox,
				width: railWidth,
				cols,
				heat: new Float32Array(cols * rows * 2)
			});
			// right rail starts after its 1px left border
			rails = width >= MIN_WIDTH ? [make(0), make(width - railWidth + 1)] : [];
			draw();
		};

		/** The mesh itself, drawn here so glow and lines share one geometry. */
		const drawMesh = (rail: Rail) => {
			const { ox, cols } = rail;
			const dark = document.documentElement.classList.contains('dark');
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
					const pts = triangle(ox, cell % cols, Math.floor(cell / cols), (i & 1) as 0 | 1);
					const [cx, cy] = centroid(pts);
					// skip triangles that spill past the rail edge
					if (cx < ox || cx > ox + railWidth) {
						heat[i] = 0;
						continue;
					}

					// stroke the mesh edges (+0.5 aligns with the 1px CSS lines)
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

		const onMove = (e: PointerEvent) => {
			for (const rail of rails) {
				const { heat, cols, ox } = rail;
				if (e.clientX < ox - RADIUS || e.clientX > ox + rail.width + RADIUS) continue;

				const c0 = Math.max(0, Math.floor((e.clientX - RADIUS - ox) / SIZE));
				const c1 = Math.min(cols - 1, Math.floor((e.clientX + RADIUS - ox) / SIZE));
				const r0 = Math.max(0, Math.floor((e.clientY - RADIUS) / SIZE));
				const r1 = Math.min(rows - 1, Math.floor((e.clientY + RADIUS) / SIZE));

				for (let row = r0; row <= r1; row++) {
					for (let col = c0; col <= c1; col++) {
						for (const k of [0, 1] as const) {
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
		const observer = new MutationObserver(() => draw());
		observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] });

		return () => {
			cancelAnimationFrame(frame);
			observer.disconnect();
			window.removeEventListener('resize', resize);
			window.removeEventListener('pointermove', onMove);
		};
	});
</script>

<canvas
	bind:this={canvas}
	aria-hidden="true"
	class="pointer-events-none fixed inset-0 z-0 hidden h-screen w-full [mask-image:linear-gradient(#000,transparent_min(95vh,900px))] select-none xl:block"
></canvas>
